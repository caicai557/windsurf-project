use teleflow_core::persistence::store::DbStore;
use teleflow_core::persistence::checkpointer::Checkpointer;
use teleflow_core::domain::workflow::{WorkflowInstance, WorkflowStatus};
use serde_json::json;
use std::time::Duration;

#[tokio::test]
async fn test_durable_execution_persistence() -> anyhow::Result<()> {
    // Setup in-memory DB for testing
    let store = DbStore::new("sqlite::memory:").await?;
    let checkpointer = Checkpointer::new(store.clone());

    let workflow_id = "flow-test-001";
    
    // 1. Create and Persist
    let state = json!({ "step": 1, "data": "init" });
    let instance = WorkflowInstance::new(workflow_id.to_string(), state.clone());
    
    checkpointer.persist(instance.clone()).await?;

    // Allow some time for write-behind (if it was async spawned)
    // Our current implementation in persist spawns a task.
    tokio::time::sleep(Duration::from_millis(100)).await;

    // 2. Simulate "Crash" - verify via raw SQL
    let row = sqlx::query_as::<_, (String, String)>(
        r#"SELECT status, state FROM workflow_instances WHERE id = ?"#
    )
    .bind(workflow_id)
    .fetch_optional(store.pool())
    .await?;

    assert!(row.is_some());
    let (status, _state) = row.unwrap();
    assert_eq!(status, "Running"); // Default
    
    // 3. Update State
    let mut instance_v2 = instance.clone();
    instance_v2.state = json!({ "step": 2, "data": "processed" });
    instance_v2.status = WorkflowStatus::Completed;
    instance_v2.version += 1;

    checkpointer.persist(instance_v2).await?;
    tokio::time::sleep(Duration::from_millis(100)).await;

    // 4. Verify Update
    let loaded = checkpointer.load(workflow_id).await?.expect("Should exist");
    assert_eq!(loaded.version, 2);
    assert_eq!(loaded.status, WorkflowStatus::Completed);
    
    Ok(())
}
