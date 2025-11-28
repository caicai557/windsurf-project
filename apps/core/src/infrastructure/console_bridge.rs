use base64::{engine::general_purpose::STANDARD, Engine};
use lazy_static::lazy_static;
use regex::Regex;
use rmp_serde::decode::Error as RmpError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignalPayload {
    pub channel: SignalChannel,
    pub selector: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("payload does not match Teleflow prefix")]
    InvalidPrefix,
    #[error("invalid base64: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
    #[error("invalid messagepack: {0}")]
    InvalidMsgPack(#[from] RmpError),
}

const PREFIX: &str = "__TELEFLOW__:";

lazy_static! {
    static ref PREFIX_REGEX: Regex = Regex::new(r"^__TELEFLOW__:\s*(?P<payload>.+)$").unwrap();
}

pub struct ConsoleBridge;

impl ConsoleBridge {
    pub fn decode_signal(console_message: &str) -> Result<SignalPayload, BridgeError> {
        let captures = PREFIX_REGEX
            .captures(console_message)
            .ok_or(BridgeError::InvalidPrefix)?;
        let payload = captures
            .name("payload")
            .ok_or(BridgeError::InvalidPrefix)?
            .as_str();

        let bytes = STANDARD.decode(payload)?;
        let signal: SignalPayload = rmp_serde::from_slice(&bytes)?;
        Ok(signal)
    }
}
