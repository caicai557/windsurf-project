use std::collections::HashMap;

use ractor::{Actor, ActorId, ActorProcessingErr, ActorRef, SupervisionEvent};
use tracing::{error, info};

use crate::actors::account::{AccountActor, AccountArgs};
use crate::actors::messages::{AccountMessage, SupervisorMessage};

#[derive(Debug, Default)]
pub struct SupervisorState {
    pub children: HashMap<String, ActorRef<AccountMessage>>,
    pub generations: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
pub struct SystemSupervisor;

impl SupervisorState {
    fn next_generation(&mut self, id: &str) -> usize {
        let entry = self.generations.entry(id.to_string()).or_insert(0);
        *entry += 1;
        *entry
    }

    fn detach_child(&mut self, actor_id: &ActorId) -> Option<String> {
        let key = self
            .children
            .iter()
            .find_map(|(name, child)| {
                if child.get_id() == *actor_id {
                    Some(name.clone())
                } else {
                    None
                }
            });
        if let Some(id) = &key {
            self.children.remove(id);
        }
        key
    }
}

impl SystemSupervisor {
    async fn spawn_account(
        &self,
        id: String,
        state: &mut SupervisorState,
        parent: &ActorRef<SupervisorMessage>,
    ) -> Result<(), ActorProcessingErr> {
        let generation = state.next_generation(&id);
        let args = AccountArgs {
            id: id.clone(),
            generation,
        };

        match Actor::spawn_linked(Some(id.clone()), AccountActor, args, parent.get_cell()).await {
            Ok((child_ref, _handle)) => {
                state.children.insert(id.clone(), child_ref);
                info!("Crown spawned Lord {} (generation {})", id, generation);
            }
            Err(err) => {
                error!("Failed to spawn {}: {}", id, err);
                return Err(ActorProcessingErr::from(err.to_string()));
            }
        }

        Ok(())
    }
}

impl Actor for SystemSupervisor {
    type Msg = SupervisorMessage;
    type State = SupervisorState;
    type Arguments = SupervisorState;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        info!("SystemSupervisor online");
        Ok(args)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SupervisorMessage::SpawnAccount { id } => {
                self.spawn_account(id, state, &myself).await?;
            }
            SupervisorMessage::Dispatch { id, message } => {
                if let Some(child) = state.children.get(&id) {
                    let _ = child.cast(message);
                } else {
                    error!("Attempted to dispatch to unknown lord {}", id);
                }
            }
            SupervisorMessage::GetGeneration { id, respond_to } => {
                let _ = respond_to.send(state.generations.get(&id).copied());
            }
        }
        Ok(())
    }

    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        message: SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SupervisionEvent::ActorFailed(actor_cell, error) => {
                let actor_id = actor_cell.get_id();
                error!("Lord {} has fallen: {}", actor_id, error);
                if let Some(child_name) = state.detach_child(&actor_id) {
                    self.spawn_account(child_name, state, &myself).await?;
                }
            }
            SupervisionEvent::ActorTerminated(actor_cell, _, _) => {
                let actor_id = actor_cell.get_id();
                info!("Lord {} terminated", actor_id);
                if let Some(child_name) = state.detach_child(&actor_id) {
                    self.spawn_account(child_name, state, &myself).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
