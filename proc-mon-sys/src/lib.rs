use crate::ffi::{proc_name, proc_pid_rusage};
use libc::c_void;

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessType {
    All = ffi::PROC_ALL_PIDS as isize,
    PGRP = ffi::PROC_PGRP_ONLY as isize,
    TTY = ffi::PROC_TTY_ONLY as isize,
    UID = ffi::PROC_UID_ONLY as isize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RUsageInfoV4 {
    pub ri_uuid: [u8; 16],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum ProcPidRusageError {
    InvalidArgument = libc::EINVAL as isize,
    NoSuchProcess = libc::ESRCH as isize,
    PermissionDenied = libc::EACCES as isize,
    OutOfMemory = libc::ENOMEM as isize,
    BadAddress = libc::EFAULT as isize,
    Unknown = -1,
}

impl ProcPidRusageError {
    fn from_errno(errno: i32) -> Self {
        match errno {
            libc::EINVAL => ProcPidRusageError::InvalidArgument,
            libc::ESRCH => ProcPidRusageError::NoSuchProcess,
            libc::EACCES => ProcPidRusageError::PermissionDenied,
            libc::ENOMEM => ProcPidRusageError::OutOfMemory,
            libc::EFAULT => ProcPidRusageError::BadAddress,
            _ => ProcPidRusageError::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Process {
    pid: i32,
}
impl Process {
    pub fn new(pid: i32) -> Self {
        Self { pid }
    }
    pub fn pid(&self) -> i32 {
        self.pid
    }
    pub fn name(&self) -> String {
        let mut name_buf = [0u8; 1024];
        let res = unsafe {
            proc_name(
                self.pid,
                name_buf.as_mut_ptr() as *mut c_void,
                name_buf.len() as u32,
            )
        };
        if res >= 0 {
            let name = std::str::from_utf8(&name_buf[0..res as usize]).unwrap();
            return name.to_string();
        }
        panic!("Failed to get process name")
    }
    /// Returns the usage information for the process
    ///
    /// This is based on the rusage_info_v2 struct version
    ///
    /// # Errors
    ///
    /// See [ProcPidRusageError](enum.ProcPidRusageError.html) for more information
    pub fn usage(&self) -> Result<RUsageInfoV4, ProcPidRusageError> {
        let mut rusage = RUsageInfoV4::default();
        let flavor = ffi::RUSAGE_INFO_V4;
        let result = unsafe {
            proc_pid_rusage(
                self.pid,
                flavor as i32,
                &mut rusage as *mut _ as *mut *mut c_void,
            )
        };
        if result == -1 {
            let errno = unsafe { *libc::__error() };
            let error = ProcPidRusageError::from_errno(errno);
            // Handle the error accordingly
            return Err(error);
        }
        Ok(rusage)
    }
}
/// Lists all the active processes on the system
pub fn list_all_pids(proc_type: ProcessType) -> Vec<Process> {
    let typeinfo: u32 = 0;
    let buffer_size: i32 =
        unsafe { ffi::proc_listpids(proc_type as u32, typeinfo, std::ptr::null_mut(), 0) };
    let mut buffer: Vec<i32> =
        Vec::with_capacity((buffer_size as usize) / std::mem::size_of::<i32>());
    let _bytes_filled = unsafe {
        ffi::proc_listpids(
            proc_type as u32,
            typeinfo,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            buffer_size,
        )
    };
    unsafe {
        buffer.set_len((buffer_size as usize) / std::mem::size_of::<i32>());
    }
    buffer.iter().map(|pid| Process::new(*pid)).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_get_processes() {
        let pids = list_all_pids(ProcessType::All);
        assert!(!pids.is_empty());
    }
    #[test]
    fn it_can_get_process_name() {
        let pids = list_all_pids(ProcessType::All);
        let process = pids.first().unwrap();
        assert!(!process.name().is_empty());
    }
    #[test]
    fn it_can_get_process_usage() {
        let pids = list_all_pids(ProcessType::All);
        let process = pids.first().unwrap();
        let usage = process.usage();
        assert!(usage.is_ok());
        if let Ok(usage) = usage {
            println!("{:?}", usage);
        }
    }
    #[test]
    fn it_can_get_process_usage_for_invalid_pid() {
        let process = Process::new(-1);
        let usage = process.usage();
        assert!(usage.is_err());
        if let Err(error) = usage {
            println!("{:?}", error);
        }
    }
    #[test]
    fn it_can_friendly_display_process_usage() {
        let pids = list_all_pids(ProcessType::All);
        let processes = pids
            .iter()
            .take(10)
            .map(|p| (p.name(), p.usage()))
            .filter_map(|p| match p.1 {
                Ok(usage) => Some((p.0, usage)),
                Err(_) => None,
            });
        for (name, info) in processes {
            println!(
                "{}\t(Bytes read: {} Bytes written: {})",
                name, info.ri_diskio_bytesread, info.ri_diskio_byteswritten
            );
        }
    }
    #[test]
    fn it_can_list_top_ten_expensive() {
        let pids = list_all_pids(ProcessType::All);
        let mut processes = pids
            .iter()
            .map(|p| (p.name(), p.usage()))
            .filter_map(|p| match p.1 {
                Ok(usage) => Some((p.0, usage)),
                Err(_) => None,
            })
            .collect::<Vec<_>>();
        processes.sort_by(|a, b| b.1.ri_phys_footprint.cmp(&a.1.ri_phys_footprint));
        for (name, info) in processes.iter().take(10) {
            println!("{}\t {}bytes", name, info.ri_resident_size);
        }
        assert!(!processes.is_empty());
    }
}
