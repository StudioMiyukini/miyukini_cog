use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetMode {
    OfflineLocal,
    HostAuthoritative,
    DedicatedClient,
}

impl NetMode {
    pub fn is_authoritative(self) -> bool {
        matches!(self, Self::OfflineLocal | Self::HostAuthoritative)
    }
}
