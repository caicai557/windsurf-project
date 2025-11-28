# Actor System Specification

## 1. Ractor Implementation
* **Framework**: `ractor` crate.
* **Topology**:
    `System` -> `AccountSupervisor` -> [`BrowserActor`, `WorkflowActor`]

## 2. Message Protocol (The Nervous System)
Messages must be strictly typed Enums.

```rust
// Example Structure
enum AccountMessage {
    // Commands
    Start { flow_id: String },
    Stop,
    // Signals (From Perception)
    Signal { 
        source: String, // "DOM", "Fiber", "OCR"
        payload: Vec<u8> // MessagePack binary
    },
    // Internals
    Heartbeat,
}
```

## 3. Fault Tolerance (Self-Healing)
If `BrowserActor` crashes (CDP disconnect), `AccountSupervisor` terminates the process and spawns a new one.
`WorkflowActor` reloads state from SQLite and resumes execution.
