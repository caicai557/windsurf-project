use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum WorkflowStatus {
    Running,
    Paused,
    Failed,
    Completed,
}

impl Default for WorkflowStatus {
    fn default() -> Self {
        Self::Running
    }
}

impl WorkflowStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkflowStatus::Running => "RUNNING",
            WorkflowStatus::Paused => "PAUSED",
            WorkflowStatus::Failed => "FAILED",
            WorkflowStatus::Completed => "COMPLETED",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_uppercase().as_str() {
            "RUNNING" => Some(WorkflowStatus::Running),
            "PAUSED" => Some(WorkflowStatus::Paused),
            "FAILED" => Some(WorkflowStatus::Failed),
            "COMPLETED" => Some(WorkflowStatus::Completed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRecord {
    pub id: String,
    pub account_id: String,
    pub flow_definition_id: String,
    pub state: Value,
    pub status: WorkflowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub kind: String,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition<S> {
    pub next_state: S,
    pub status: WorkflowStatus,
    pub actions: Vec<Action>,
}

pub trait WorkflowBehavior: Send + Sync + 'static {
    type State: Clone + Serialize + DeserializeOwned + Send + Sync + 'static;
    type Event: Clone + Send + Sync + 'static;

    fn compute_next(current_state: Self::State, event: Self::Event) -> Transition<Self::State>;
}
