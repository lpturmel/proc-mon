// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use proc_mon::{list_all_pids, ProcessType};
use shared_types::ProcessPayload;

#[tauri::command]
fn get_processes() -> Vec<ProcessPayload> {
    let procs = list_all_pids(ProcessType::All);
    let mut procs = procs
        .into_iter()
        .flat_map(|p| {
            let usage = p.usage();
            if let Ok(usage) = usage {
                Some(ProcessPayload {
                    id: uuid::Uuid::new_v4().to_string(),
                    pid: p.pid(),
                    name: p.name(),
                    mem_usage: usage.ri_phys_footprint,
                })
            } else {
                None
            }
        })
        .collect::<Vec<ProcessPayload>>();
    procs.sort_by(|a, b| b.mem_usage.cmp(&a.mem_usage));
    procs.into_iter().take(10).collect::<Vec<_>>()
}
fn main() {
    #[cfg(not(target_os = "macos"))]
    panic!("This application only works on macOS");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_processes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
