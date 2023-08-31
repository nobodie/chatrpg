use crate::model::NodeId;
use thiserror::Error;
use tokio::io;
use toml::de;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("General error ")]
    General,
    #[error("Io error {0}")]
    Io(#[from] io::Error),
    #[error("Toml error {0}")]
    Toml(#[from] de::Error),

    #[error("Serde error {0}")]
    Serde(#[from] serde_json::Error),

    #[error("ChatRpg error {0}")]
    ChatRpg(RpgError),
}

#[derive(Error, Debug)]
pub enum RpgError {
    #[error("Unknown node id {0}")]
    UnknownNodeId(NodeId),

    #[error("current node {0} is not connected to node {1}")]
    NodeNotConnected(NodeId, NodeId),
}

pub type GeneralResult<T> = Result<T, MyError>;
pub type RpgResult<T> = Result<T, RpgError>;
