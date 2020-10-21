use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum RemoteCredentials {
    PKI { pubkey: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ContextType {
    Local,
    Remote {
        host: String,
        credentials: RemoteCredentials,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Context {
    pub(crate) type_: ContextType,
    pub(crate) name: String,
}
