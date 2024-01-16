// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use proc_mon_sys::{list_all_pids, ProcessType};
use shared_types::ProcessPayload;
use tauri::async_runtime::spawn;
use tauri::Manager;

fn main() {
    #[cfg(not(target_os = "macos"))]
    panic!("This application only works on macOS");

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            spawn(async move {
                loop {
                    let procs = list_all_pids(ProcessType::All);
                    let mut procs = procs
                        .iter()
                        .flat_map(|p| {
                            let info = p.usage();
                            if let Ok(info) = info {
                                Some(ProcessPayload {
                                    name: p.name(),
                                    pid: p.pid(),
                                    mem_usage: info.ri_phys_footprint,
                                })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<ProcessPayload>>();
                    procs.sort_by(|a, b| b.mem_usage.cmp(&a.mem_usage));
                    let procs = procs.iter().take(10).collect::<Vec<_>>();
                    window.emit("procs", Some(procs)).unwrap();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });
            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
