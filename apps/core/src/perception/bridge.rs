use ractor::ActorRef;
use serde::{Deserialize, Serialize};
use tracing::{error, debug};
use crate::actors::AccountMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalSource {
    Fiber,
    DOM,
    Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionSignal {
    pub source: SignalSource,
    pub selector: String,
    pub value: serde_json::Value,
    pub timestamp: u64,
}

pub struct ConsoleBridge {
    account_actor: ActorRef<AccountMessage>,
}

impl ConsoleBridge {
    pub fn new(account_actor: ActorRef<AccountMessage>) -> Self {
        Self { account_actor }
    }

    /// Processes a raw console message. 
    /// Expected format: "TELEFLOW_SIG:<Base64(MessagePack(PerceptionSignal))>"
    pub fn process_log(&self, log_text: &str) {
        if !log_text.starts_with("TELEFLOW_SIG:") {
            return;
        }

        let payload_b64 = &log_text["TELEFLOW_SIG:".len()..];
        
        match self.decode_signal(payload_b64) {
            Ok(signal) => {
                debug!("Bridge received signal: {:?}", signal);
                // Route to Actor
                let msg = AccountMessage::Signal { 
                    source: format!("{:?}", signal.source), 
                    payload: serde_json::to_vec(&signal.value).unwrap_or_default()
                };
                
                if let Err(e) = self.account_actor.cast(msg) {
                    error!("Failed to cast signal to actor: {}", e);
                }
            },
            Err(e) => {
                error!("Bridge decode failed: {}", e);
            }
        }
    }

    fn decode_signal(&self, b64_data: &str) -> anyhow::Result<PerceptionSignal> {
        use base64::{Engine as _, engine::general_purpose};
        
        let bytes = general_purpose::STANDARD.decode(b64_data)?;
        
        // Try MessagePack first
        if let Ok(signal) = rmp_serde::from_slice::<PerceptionSignal>(&bytes) {
            return Ok(signal);
        }

        // Fallback to JSON (for MVP/Debug scripts that just Base64 JSON)
        let signal: PerceptionSignal = serde_json::from_slice(&bytes)?;
        Ok(signal)
    }
}
