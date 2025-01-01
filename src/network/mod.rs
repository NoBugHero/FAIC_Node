pub mod peer;
pub mod server;

use crate::error::NodeResult;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub address: String,
    pub node_type: NodeType,
    pub models: Vec<String>,
    pub latency: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeType {
    Validator,
    Compute,
    Router,
}

#[async_trait]
pub trait NetworkService {
    async fn connect(&self, address: &str) -> NodeResult<()>;
    async fn disconnect(&self) -> NodeResult<()>;
    async fn broadcast(&self, message: Vec<u8>) -> NodeResult<()>;
    async fn send_to(&self, peer_id: &str, message: Vec<u8>) -> NodeResult<()>;
}
