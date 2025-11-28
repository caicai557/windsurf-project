use serde::{Deserialize, Serialize};
use serde_json::{json, from_value};
use sqlx::SqlitePool;
use teleflow_core::domain::workflow::{Action, Transition, WorkflowBehavior, WorkflowStatus};
use teleflow_core::{Checkpointer, NewWorkflowInstance, init_db};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryState {
    step: u32,
}

#[derive(Debug, Clone)]
struct Advance;

struct MemoryWorkflow;

impl WorkflowBehavior for MemoryWorkflow {
    type State = MemoryState;
    type Event = Advance;

    fn compute_next(current_state: Self::State, _event: Self::Event) -> Transition<Self::State> {
        let next_step = current_state.step + 1;
        Transition {
            next_state: MemoryState { step: next_step },
            status: WorkflowStatus::Running,
            actions: vec![Action {
                kind: "log".into(),
                payload: json!({ "step": next_step }),
            }],
        }
    }
}

#[tokio::test]
async fn highlander_test() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:?cache=shared").await?;
    init_db(&pool).await?;

    let checkpointer = Checkpointer::<MemoryWorkflow>::new(pool.clone());

    checkpointer
        .bootstrap_instance(NewWorkflowInstance {
            id: "wf-highlander".into(),
            account_id: "account-1".into(),
            flow_definition_id: "flow-basic".into(),
            state: MemoryState { step: 1 },
            status: WorkflowStatus::Running,
        })
        .await?;

    let actions = checkpointer
        .process_event("wf-highlander", Advance)
        .await?;
    assert_eq!(actions.len(), 1);

    drop(checkpointer);

    let resurrected = Checkpointer::<MemoryWorkflow>::new(pool.clone());
    let record = resurrected
        .load_record("wf-highlander")
        .await?
        .expect("record must exist");

    let persisted_state: MemoryState = from_value(record.state)?;
    assert_eq!(persisted_state.step, 2);

    Ok(())
}
