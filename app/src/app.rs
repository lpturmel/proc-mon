use std::time::Duration;

// use futures::StreamExt;
use leptos::leptos_dom::console_log;
use leptos::*;
use serde::Serialize;
use shared_types::ProcessPayload;
// use tauri_sys::event;
use tauri_sys::tauri::invoke;
// use wasm_bindgen_futures::spawn_local;

// async fn list_on_proc_info(event_writer: WriteSignal<Vec<ProcessPayload>>) {
//     let mut events = event::listen::<Vec<ProcessPayload>>("procs").await.unwrap();
//     while let Some(ev) = events.next().await {
//         event_writer.set(ev.payload);
//     }
// }
async fn get_procs(event_writer: WriteSignal<Vec<ProcessPayload>>) {
    let procs = invoke::<_, Vec<ProcessPayload>>("get_processes", &())
        .await
        .unwrap();
    // for proc in &procs {
    //     console_log(format!("{}: {}", proc.pid, proc.name).as_str());
    // }
    event_writer.set(procs);
}
fn format_bytes_as_hr(bytes: u64) -> String {
    let bytes_per_kb: f64 = 1024.0;
    let bytes_per_mb: f64 = 1024.0 * 1024.0;
    let bytes_per_gb: f64 = 1024.0 * 1024.0 * 1024.0;

    if bytes as f64 >= bytes_per_gb {
        format!("{:.2} GB", bytes as f64 / bytes_per_gb)
    } else if bytes as f64 >= bytes_per_mb {
        format!("{:.2} MB", bytes as f64 / bytes_per_mb)
    } else if bytes as f64 >= bytes_per_kb {
        format!("{:.2} KB", bytes as f64 / bytes_per_kb)
    } else {
        format!("{} bytes", bytes)
    }
}
#[derive(Serialize)]
#[non_exhaustive]
struct GetProcCmdArgs;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (procs, set_procs) = create_signal::<Vec<ProcessPayload>>(cx, vec![]);

    set_interval_with_handle(
        move || spawn_local(get_procs(set_procs)),
        Duration::from_secs(1),
    )
    .expect("Failed to set interval");

    create_effect(cx, move |_| {
        let procs = procs.get();
        let json_str = serde_json::to_string_pretty(&procs).unwrap();
        console_log(json_str.as_str());
    });
    view! { cx,
        <main class="container w-full h-full">
            <div class="overflow-x-auto">
          <table class="table table-lg">
            <thead>
              <tr>
                <th>"Pid"</th>
                <th>"Name"</th>
                <th>"Memory"</th>
              </tr>
            </thead>
            <tbody>
                <For
                    each=move || procs.get()
                    key=|p| p.id.clone()
                    view=move |cx, e: ProcessPayload| {
                    view! { cx, <tr>
                        <td>{ e.pid }</td>
                        <td>{ e.name }</td>
                        <td>{ format_bytes_as_hr(e.mem_usage) }</td>
                    </tr> }
                    }
                />
            </tbody>
          </table>
    </div>
        </main>
    }
}
