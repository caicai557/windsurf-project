use teleflow_core::actors::{AccountActor, AccountArgs};
use ractor::Actor;

#[tokio::test]
async fn test_scalability() -> anyhow::Result<()> {
    let mut actors = Vec::new();
    
    // Spawn 50 Actors
    for i in 0..50 {
        let id = format!("account-{}", i);
        let args = AccountArgs {
            id: id.clone(),
            generation: 1,
        };
        
        let (actor_ref, join_handle) = Actor::spawn(Some(id), AccountActor, args).await?;
        actors.push((actor_ref, join_handle));
    }
    
    // Give them a moment to spin up
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Verify they are alive
    for (actor_ref, _) in &actors {
        let status = actor_ref.get_status();
        assert!(status == ractor::ActorStatus::Running || status == ractor::ActorStatus::Starting);
    }
    
    // Cleanup
    for (actor_ref, _) in actors {
        actor_ref.stop(None);
    }
    
    Ok(())
}
