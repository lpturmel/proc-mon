use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ProcessPayload {
    pub name: String,
    pub pid: i32,
    pub mem_usage: u64,
}
