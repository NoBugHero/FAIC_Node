use super::{NetworkService, NodeInfo};
use crate::error::NodeResult;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct NetworkServer {
    peers: RwLock<HashMap<String, NodeInfo>>,
}

impl NetworkServer {
    pub fn new() -> Self {
        Self {
            peers: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl NetworkService for NetworkServer {
    async fn connect(&self, _address: &str) -> NodeResult<()> {
        // TODO: 实现连接逻辑
        Ok(())
    }

    async fn disconnect(&self) -> NodeResult<()> {
        // TODO: 实现断开连接逻辑
        Ok(())
    }

    async fn broadcast(&self, _message: Vec<u8>) -> NodeResult<()> {
        // TODO: 实现广播逻辑
        Ok(())
    }

    async fn send_to(&self, _peer_id: &str, _message: Vec<u8>) -> NodeResult<()> {
        // TODO: 实现点对点发送逻辑
        Ok(())
    }
}
