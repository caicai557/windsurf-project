use ractor::{Actor, ActorProcessingErr, ActorRef};
use tracing::info;

use crate::actors::messages::AccountMessage;

/// Placeholder state that will later host browser/session handles.
#[derive(Debug, Clone, Default)]
pub struct AccountState;

/// Arguments provided when spawning a new AccountActor instance.
#[derive(Debug, Clone)]
pub struct AccountArgs {
    pub id: String,
    pub generation: usize,
}

#[derive(Debug, Clone)]
pub struct AccountActor;

impl Actor for AccountActor {
    type Msg = AccountMessage;
    type State = AccountState;
    type Arguments = AccountArgs;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        info!("Lord {} has risen (generation {})", args.id, args.generation);
        Ok(AccountState::default())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            AccountMessage::Start { flow_id } => {
                info!(target: "account", "[{}] Start workflow {}", myself.get_id(), flow_id);
            }
            AccountMessage::Stop => {
                info!(target: "account", "[{}] Stop command received", myself.get_id());
            }
            AccountMessage::Signal { source, payload } => {
                info!(
                    target: "account",
                    "[{}] Signal from {} ({} bytes)",
                    myself.get_id(),
                    source,
                    payload.len()
                );
            }
            AccountMessage::Heartbeat => {
                info!(target: "account", "[{}] Heartbeat", myself.get_id());
            }
            AccountMessage::Kill => {
                // The Phoenix test relies on this panic to trigger supervisor restart.
                panic!("{} has fallen by decree", myself.get_id());
            }
        }
        Ok(())
    }
}
