use common::models::TrafficEvent;
use leptos::prelude::*;

#[component]
pub fn TrafficPage(
    traffic: ReadSignal<Vec<TrafficEvent>>,
    throughput: ReadSignal<u64>,
) -> impl IntoView {
    let (selected_proto, set_selected_proto) = signal::<Option<String>>(None);
    let (search_text, set_search_text) = signal(String::new());
    let (is_paused, set_is_paused) = signal(false);

    let filtered_traffic = Memo::new(move |_| {
        let query = search_text.get().to_lowercase();
        let proto = selected_proto.get();

        traffic.get().into_iter().filter(|t| {
            if let Some(ref p) = proto {
                if !t.protocol.eq_ignore_ascii_case(p) {
                    return false;
                }
            }
            if !query.is_empty() {
                let s_ip = t.src_ip.to_string().to_lowercase();
                let d_ip = t.dst_ip.to_string().to_lowercase();
                if !s_ip.contains(&query) && !d_ip.contains(&query) {
                    return false;
                }
            }
            true
        }).collect::<Vec<TrafficEvent>>()
    });

    view! {
        <div class="space-y-6">
            // Header
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-extrabold text-white tracking-tight flex items-center gap-2.5">
                        <span class="w-3 h-3 rounded-full bg-emerald-500 animate-pulse"></span>
                        "Live Network Packet Stream"
                    </h1>
                    <p class="text-xs text-slate-400 mt-1">
                        "High-speed TimescaleDB hypertable feed capturing raw packet metadata & flags"
                    </p>
                </div>
                <div class="flex items-center gap-3">
                    <div class="px-3 py-1.5 rounded-xl bg-slate-900 border border-slate-800 text-xs font-mono text-emerald-400">
                        {move || format!("Throughput: {} B/s", throughput.get())}
                    </div>
                    <button
                        class=move || {
                            if is_paused.get() {
                                "px-3.5 py-1.5 rounded-xl bg-amber-500/20 text-amber-400 border border-amber-500/30 text-xs font-semibold transition"
                            } else {
                                "px-3.5 py-1.5 rounded-xl bg-slate-900 hover:bg-slate-800 text-slate-300 border border-slate-800 text-xs font-semibold transition"
                            }
                        }
                        on:click=move |_| set_is_paused.update(|p| *p = !*p)
                    >
                        {move || if is_paused.get() { "▶ Resume Stream" } else { "⏸ Pause Display" }}
                    </button>
                </div>
            </div>

            // Controls & Filters Bar
            <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-4 shadow-xl flex flex-wrap items-center justify-between gap-4">
                <div class="flex items-center gap-2 flex-1 max-w-sm">
                    <input
                        type="text"
                        placeholder="Filter by Source or Target IP..."
                        class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition"
                        on:input=move |e| set_search_text.set(event_target_value(&e))
                    />
                </div>

                <div class="flex items-center gap-2">
                    <button
                        class=move || {
                            if selected_proto.get().is_none() {
                                "px-3 py-1.5 rounded-xl text-xs font-semibold bg-indigo-600 text-white transition"
                            } else {
                                "px-3 py-1.5 rounded-xl text-xs font-medium text-slate-400 bg-slate-950 hover:text-white transition"
                            }
                        }
                        on:click=move |_| set_selected_proto.set(None)
                    >
                        "ALL"
                    </button>
                    <button
                        class=move || {
                            if selected_proto.get().as_deref() == Some("tcp") {
                                "px-3 py-1.5 rounded-xl text-xs font-semibold bg-indigo-600 text-white transition"
                            } else {
                                "px-3 py-1.5 rounded-xl text-xs font-medium text-slate-400 bg-slate-950 hover:text-white transition"
                            }
                        }
                        on:click=move |_| set_selected_proto.set(Some("tcp".to_string()))
                    >
                        "TCP"
                    </button>
                    <button
                        class=move || {
                            if selected_proto.get().as_deref() == Some("udp") {
                                "px-3 py-1.5 rounded-xl text-xs font-semibold bg-indigo-600 text-white transition"
                            } else {
                                "px-3 py-1.5 rounded-xl text-xs font-medium text-slate-400 bg-slate-950 hover:text-white transition"
                            }
                        }
                        on:click=move |_| set_selected_proto.set(Some("udp".to_string()))
                    >
                        "UDP"
                    </button>
                    <button
                        class=move || {
                            if selected_proto.get().as_deref() == Some("icmp") {
                                "px-3 py-1.5 rounded-xl text-xs font-semibold bg-indigo-600 text-white transition"
                            } else {
                                "px-3 py-1.5 rounded-xl text-xs font-medium text-slate-400 bg-slate-950 hover:text-white transition"
                            }
                        }
                        on:click=move |_| set_selected_proto.set(Some("icmp".to_string()))
                    >
                        "ICMP"
                    </button>
                    <button
                        class=move || {
                            if selected_proto.get().as_deref() == Some("dns") {
                                "px-3 py-1.5 rounded-xl text-xs font-semibold bg-indigo-600 text-white transition"
                            } else {
                                "px-3 py-1.5 rounded-xl text-xs font-medium text-slate-400 bg-slate-950 hover:text-white transition"
                            }
                        }
                        on:click=move |_| set_selected_proto.set(Some("dns".to_string()))
                    >
                        "DNS"
                    </button>
                </div>
            </div>

            // Traffic Table
            <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md overflow-x-auto">
                <table class="w-full text-left text-xs text-slate-300">
                    <thead class="text-slate-400 border-b border-slate-800/80 uppercase tracking-wider font-semibold">
                        <tr>
                            <th class="pb-3 px-3">"Protocol"</th>
                            <th class="pb-3 px-3">"Source IP:Port"</th>
                            <th class="pb-3 px-3">"Target IP:Port"</th>
                            <th class="pb-3 px-3">"Bytes"</th>
                            <th class="pb-3 px-3">"Flags"</th>
                            <th class="pb-3 px-3">"Timestamp"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800/40">
                        <For
                            each=move || filtered_traffic.get()
                            key=|ev| ev.id
                            children=move |event| {
                                let proto_upper = event.protocol.to_uppercase();
                                let proto_badge = match proto_upper.as_str() {
                                    "TCP" => "bg-blue-500/10 text-blue-400 border-blue-500/30",
                                    "UDP" => "bg-purple-500/10 text-purple-400 border-purple-500/30",
                                    "ICMP" => "bg-amber-500/10 text-amber-400 border-amber-500/30",
                                    "DNS" => "bg-emerald-500/10 text-emerald-400 border-emerald-500/30",
                                    "ARP" => "bg-pink-500/10 text-pink-400 border-pink-500/30",
                                    _ => "bg-slate-500/10 text-slate-400 border-slate-500/30",
                                };

                                view! {
                                    <tr class="hover:bg-slate-800/30 transition">
                                        <td class="py-3 px-3 whitespace-nowrap">
                                            <span class=format!("inline-flex items-center px-2 py-0.5 rounded text-[10px] font-bold border {}", proto_badge)>
                                                {proto_upper}
                                            </span>
                                        </td>
                                        <td class="py-3 px-3 font-mono text-slate-200">
                                            {format!("{}:{}", event.src_ip, event.src_port)}
                                        </td>
                                        <td class="py-3 px-3 font-mono text-slate-200">
                                            {format!("{}:{}", event.dst_ip, event.dst_port)}
                                        </td>
                                        <td class="py-3 px-3 font-mono text-emerald-400 font-semibold">
                                            {format!("{} B", event.bytes_transferred)}
                                        </td>
                                        <td class="py-3 px-3 font-mono text-slate-400">
                                            {event.flags.clone()}
                                        </td>
                                        <td class="py-3 px-3 text-slate-500 whitespace-nowrap font-mono text-[11px]">
                                            {event.time.format("%H:%M:%S.%3f").to_string()}
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
