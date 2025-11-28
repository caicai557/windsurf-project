use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SignalChannel {
    Fiber,
    Dom,
    Network,
    Unknown,
}

impl From<&str> for SignalChannel {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "FIBER" => SignalChannel::Fiber,
            "DOM" => SignalChannel::Dom,
            "NETWORK" => SignalChannel::Network,
            _ => SignalChannel::Unknown,
        }
    }
}

impl SignalChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SignalChannel::Fiber => "FIBER",
            SignalChannel::Dom => "DOM",
            SignalChannel::Network => "NETWORK",
            SignalChannel::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SignalPayload {
    Perception {
        channel: SignalChannel,
        selector: String,
        data: Value,
        timestamp: u64,
    },
}

impl SignalPayload {
    pub fn channel(&self) -> SignalChannel {
        match self {
            SignalPayload::Perception { channel, .. } => *channel,
        }
    }
}
