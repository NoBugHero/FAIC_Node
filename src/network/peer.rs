use super::NodeInfo;
use crate::error::NodeResult;
use tokio::net::TcpStream;

pub struct Peer {
    pub info: NodeInfo,
    pub connection: Option<TcpStream>,
}

impl Peer {
    pub async fn new(info: NodeInfo) -> Self {
        Self {
            info,
            connection: None,
        }
    }

    pub async fn connect(&mut self) -> NodeResult<()> {
        // TODO: 实现连接逻辑
        Ok(())
    }

    pub async fn disconnect(&mut self) -> NodeResult<()> {
        // TODO: 实现断开连接逻辑
        Ok(())
    }
}
