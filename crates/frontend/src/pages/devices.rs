use common::models::{Device, TrafficEvent};
use leptos::prelude::*;

use crate::api::client::ApiClient;

#[component]
pub fn DevicesPage() -> impl IntoView {
    let (devices, set_devices) = signal::<Vec<Device>>(Vec::new());
    let (search_query, set_search_query) = signal(String::new());
    let (_is_loading, set_is_loading) = signal(true);

    // Selected device history modal state
    let (selected_device, set_selected_device) = signal::<Option<Device>>(None);
    let (device_history, set_device_history) = signal::<Vec<TrafficEvent>>(Vec::new());
    let (history_loading, set_history_loading) = signal(false);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(data) = ApiClient::get_devices().await {
                set_devices.set(data);
            }
            set_is_loading.set(false);
        });
    });

    let filtered_devices = Memo::new(move |_| {
        let q = search_query.get().to_lowercase();
        devices.get().into_iter().filter(|d| {
            if q.is_empty() {
                return true;
            }
            let ip = d.ip_address.to_string().to_lowercase();
            let mac = d.mac_address.clone().unwrap_or_default().to_lowercase();
            let host = d.hostname.clone().unwrap_or_default().to_lowercase();
            let dev_type = d.device_type.to_lowercase();

            ip.contains(&q) || mac.contains(&q) || host.contains(&q) || dev_type.contains(&q)
        }).collect::<Vec<Device>>()
    });

    let on_view_history = move |dev: Device| {
        let dev_id = dev.id;
        set_selected_device.set(Some(dev));
        set_history_loading.set(true);
        set_device_history.set(Vec::new());

        leptos::task::spawn_local(async move {
            if let Ok(hist) = ApiClient::get_device_history(dev_id).await {
                set_device_history.set(hist);
            }
            set_history_loading.set(false);
        });
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-extrabold text-white tracking-tight flex items-center gap-2.5">
                        <span>"💻"</span>
                        "Network Node & Device Inventory"
                    </h1>
                    <p class="text-xs text-slate-400 mt-1">
                        "Passive ARP and IP mapping of connected endpoints, servers, and IoT devices"
                    </p>
                </div>
                <div>
                    <input
                        type="text"
                        placeholder="Search IP, MAC, hostname..."
                        class="bg-slate-900 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 w-64 transition"
                        on:input=move |e| set_search_query.set(event_target_value(&e))
                    />
                </div>
            </div>

            // Device History Modal
            {move || {
                if let Some(dev) = selected_device.get() {
                    view! {
                        <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                            <div class="bg-slate-900 border border-slate-800 rounded-2xl p-6 max-w-3xl w-full shadow-2xl space-y-4 max-h-[85vh] flex flex-col">
                                <div class="flex items-center justify-between border-b border-slate-800/80 pb-3">
                                    <div>
                                        <h3 class="text-base font-bold text-white flex items-center gap-2">
                                            <span>"📜 Traffic History for"</span>
                                            <span class="font-mono text-indigo-400">{format!("{}", dev.ip_address)}</span>
                                        </h3>
                                        <p class="text-xs text-slate-400 mt-0.5">
                                            {format!("Host: {} | MAC: {}", dev.hostname.unwrap_or_else(|| "Unknown".to_string()), dev.mac_address.unwrap_or_else(|| "-".to_string()))}
                                        </p>
                                    </div>
                                    <button
                                        on:click=move |_| set_selected_device.set(None)
                                        class="text-slate-400 hover:text-white text-base font-bold p-1"
                                    >
                                        "✕"
                                    </button>
                                </div>

                                <div class="overflow-y-auto flex-1">
                                    {move || {
                                        if history_loading.get() {
                                            view! { <div class="text-xs text-slate-400 py-6 text-center">"Loading event history..."</div> }.into_any()
                                        } else if device_history.get().is_empty() {
                                            view! { <div class="text-xs text-slate-500 py-6 text-center italic">"No recent traffic recorded for this host"</div> }.into_any()
                                        } else {
                                            view! {
                                                <table class="w-full text-left text-xs text-slate-300">
                                                    <thead class="text-slate-400 border-b border-slate-800 font-semibold uppercase text-[10px]">
                                                        <tr>
                                                            <th class="pb-2">"Time"</th>
                                                            <th class="pb-2">"Protocol"</th>
                                                            <th class="pb-2">"Peer Endpoint"</th>
                                                            <th class="pb-2">"Bytes"</th>
                                                            <th class="pb-2">"Flags"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody class="divide-y divide-slate-800/40">
                                                        {device_history.get().into_iter().map(|ev| {
                                                            let peer = if ev.src_ip == dev.ip_address {
                                                                format!("→ {}:{}", ev.dst_ip, ev.dst_port)
                                                            } else {
                                                                format!("← {}:{}", ev.src_ip, ev.src_port)
                                                            };
                                                            view! {
                                                                <tr class="hover:bg-slate-800/30 transition">
                                                                    <td class="py-2 font-mono text-[11px] text-slate-400 whitespace-nowrap">
                                                                        {ev.time.format("%H:%M:%S").to_string()}
                                                                    </td>
                                                                    <td class="py-2 uppercase font-bold text-[10px] text-indigo-400">
                                                                        {ev.protocol}
                                                                    </td>
                                                                    <td class="py-2 font-mono text-slate-200">
                                                                        {peer}
                                                                    </td>
                                                                    <td class="py-2 font-mono text-emerald-400">
                                                                        {format!("{} B", ev.bytes_transferred)}
                                                                    </td>
                                                                    <td class="py-2 font-mono text-slate-500 text-[10px]">
                                                                        {ev.flags}
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </tbody>
                                                </table>
                                            }.into_any()
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md overflow-x-auto">
                <table class="w-full text-left text-xs text-slate-300">
                    <thead class="text-slate-400 border-b border-slate-800/80 uppercase tracking-wider font-semibold">
                        <tr>
                            <th class="pb-3 px-3">"IP Address"</th>
                            <th class="pb-3 px-3">"MAC Address"</th>
                            <th class="pb-3 px-3">"Hostname"</th>
                            <th class="pb-3 px-3">"Device Type"</th>
                            <th class="pb-3 px-3">"Trust Status"</th>
                            <th class="pb-3 px-3">"Last Seen"</th>
                            <th class="pb-3 px-3 text-right">"History"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800/40">
                        <For
                            each=move || filtered_devices.get()
                            key=|d| d.id
                            children=move |device| {
                                let is_trusted = device.is_trusted;
                                let dev_clone = device.clone();
                                view! {
                                    <tr class="hover:bg-slate-800/30 transition">
                                        <td class="py-3.5 px-3 font-mono font-bold text-white whitespace-nowrap">
                                            {format!("{}", device.ip_address)}
                                        </td>
                                        <td class="py-3.5 px-3 font-mono text-slate-400 whitespace-nowrap">
                                            {device.mac_address.clone().unwrap_or_else(|| "-".to_string())}
                                        </td>
                                        <td class="py-3.5 px-3 text-slate-200">
                                            {device.hostname.clone().unwrap_or_else(|| "Unknown".to_string())}
                                        </td>
                                        <td class="py-3.5 px-3 whitespace-nowrap">
                                            <span class="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-semibold bg-slate-800 text-slate-300 border border-slate-700">
                                                {device.device_type}
                                            </span>
                                        </td>
                                        <td class="py-3.5 px-3 whitespace-nowrap">
                                            <span class=format!("inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-semibold {}", if is_trusted { "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20" } else { "bg-amber-500/10 text-amber-400 border border-amber-500/20" })>
                                                {if is_trusted { "✓ Trusted" } else { "⚠ Untrusted" }}
                                            </span>
                                        </td>
                                        <td class="py-3.5 px-3 text-slate-500 font-mono text-[11px] whitespace-nowrap">
                                            {device.last_seen.format("%Y-%m-%d %H:%M:%S").to_string()}
                                        </td>
                                        <td class="py-3.5 px-3 text-right whitespace-nowrap">
                                            <button
                                                on:click=move |_| on_view_history(dev_clone.clone())
                                                class="px-2.5 py-1 rounded-lg bg-indigo-600/15 hover:bg-indigo-600/25 text-indigo-400 border border-indigo-500/30 text-[11px] font-medium transition"
                                            >
                                                "View Events →"
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>
        </div>
    }
}
