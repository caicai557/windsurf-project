use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

/// Messages routed to AccountActor instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountMessage {
    /// Start executing a workflow for the given flow identifier.
    Start { flow_id: String },
    /// Gracefully stop workflow execution.
    Stop,
    /// Perception signals (Fiber/DOM/OCR) encoded as MessagePack binary.
    Signal { source: String, payload: Vec<u8> },
    /// Internal heartbeat used for liveness checks.
    Heartbeat,
    /// Testing hook used by the Phoenix test to force a panic/restart.
    Kill,
}

/// Messages handled by the SystemSupervisor (The Crown).
#[derive(Debug)]
pub enum SupervisorMessage {
    /// Spawn (or respawn) an AccountActor with the provided identifier.
    SpawnAccount { id: String },
    /// Dispatch an AccountMessage to a specific child actor.
    Dispatch { id: String, message: AccountMessage },
    /// Query the current generation count (number of spawns) for an account.
    GetGeneration {
        id: String,
        respond_to: oneshot::Sender<Option<usize>>,
    },
}
