use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteCredentials {
    PKI { pubkey: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContextType {
    Local,
    Remote {
        host: String,
        credentials: RemoteCredentials,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub type_: ContextType,
    pub name: String,
}
