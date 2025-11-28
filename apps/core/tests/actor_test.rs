use std::time::Duration;

use ractor::Actor;
use tokio::sync::oneshot;
use teleflow_core::actors::{AccountMessage, SupervisorMessage, SupervisorState, SystemSupervisor};

async fn get_generation(
    supervisor: &ractor::ActorRef<SupervisorMessage>,
    id: &str,
) -> Option<usize> {
    let (tx, rx) = oneshot::channel();
    let _ = supervisor.cast(SupervisorMessage::GetGeneration {
        id: id.to_string(),
        respond_to: tx,
    });
    rx.await.ok().flatten()
}

#[tokio::test]
async fn phoenix_test() -> anyhow::Result<()> {
    let (supervisor_ref, supervisor_handle) =
        Actor::spawn(Some("system".into()), SystemSupervisor, SupervisorState::default()).await?;

    // Spawn the first account actor.
    let account_id = "account-phoenix".to_string();
    let _ = supervisor_ref.cast(SupervisorMessage::SpawnAccount {
        id: account_id.clone(),
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let generation_one = get_generation(&supervisor_ref, &account_id).await;
    assert_eq!(generation_one, Some(1), "First spawn should register generation 1");

    // Issue a Kill command via the Supervisor dispatch channel.
    let _ = supervisor_ref.cast(SupervisorMessage::Dispatch {
        id: account_id.clone(),
        message: AccountMessage::Kill,
    });

    // Allow the supervisor to observe the crash and respawn the actor.
    tokio::time::sleep(Duration::from_millis(300)).await;

    let generation_two = get_generation(&supervisor_ref, &account_id).await;
    assert_eq!(generation_two, Some(2), "Supervisor must resurrect the fallen lord");

    supervisor_ref.stop(None);
    let _ = supervisor_handle.await;
    Ok(())
}
