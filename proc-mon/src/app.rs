use futures::StreamExt;
use leptos::leptos_dom::console_log;
use leptos::*;
use shared_types::ProcessPayload;
use tauri_sys::event;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
//     async fn listen(event: &str) -> JsValue;
// }
async fn list_on_proc_info(event_writer: WriteSignal<Vec<ProcessPayload>>) {
    let mut events = event::listen::<Vec<ProcessPayload>>("procs").await.unwrap();
    while let Some(ev) = events.next().await {
        console_log(&format!("Got event: {:?}", ev));
        event_writer.set(ev.payload);
    }
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
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (procs, set_procs) = create_signal::<Vec<ProcessPayload>>(cx, vec![]);

    create_local_resource(cx, move || set_procs, list_on_proc_info);

    view! { cx,
        <main class="container">
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
                    key=|p| p.pid
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
