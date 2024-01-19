use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
/// Simplification of the Process struct from proc_mon.
///
/// This is the data that is shared between the frontend and backend of Tauri.
pub struct ProcessPayload {
    /// Virtual id for rendering purposes.
    pub id: String,
    /// The process ID of the process.
    pub pid: i32,
    /// The name of the process as identified by libproc.
    pub name: String,
    /// Initially represents the ri_phys_footprint field of the RUsageInfoV4 struct.
    ///
    /// This is the resident memory usage of the process in bytes
    pub mem_usage: u64,
}
