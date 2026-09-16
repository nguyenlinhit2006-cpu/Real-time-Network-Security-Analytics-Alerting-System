use common::models::{BlockedIp, CreateBlockedIpDto};
use leptos::prelude::*;
use uuid::Uuid;

use crate::api::client::ApiClient;

#[component]
pub fn BlocklistPage() -> impl IntoView {
    let (blocklist, set_blocklist) = signal::<Vec<BlockedIp>>(Vec::new());
    let (ip_input, set_ip_input) = signal(String::new());
    let (reason_input, set_reason_input) = signal(String::new());
    let (show_modal, set_show_modal) = signal(false);
    let (status_msg, set_status_msg) = signal::<Option<String>>(None);

    let load_blocklist = move || {
        leptos::task::spawn_local(async move {
            if let Ok(data) = ApiClient::get_blocklist().await {
                set_blocklist.set(data);
            }
        });
    };

    Effect::new(move |_| {
        load_blocklist();
    });

    let on_unblock = move |id: Uuid| {
        leptos::task::spawn_local(async move {
            if ApiClient::remove_from_blocklist(id).await.is_ok() {
                set_blocklist.update(|list| list.retain(|item| item.id != id));
                set_status_msg.set(Some("IP successfully removed from blocklist".to_string()));
            }
        });
    };

    let on_add_blocked = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        let ip = ip_input.get();
        let reason = reason_input.get();

        leptos::task::spawn_local(async move {
            let dto = CreateBlockedIpDto {
                ip_address: ip,
                reason,
                duration_seconds: Some(86400), // Default 24h
            };
            if let Ok(new_block) = ApiClient::add_to_blocklist(&dto).await {
                set_blocklist.update(|list| list.insert(0, new_block));
                set_show_modal.set(false);
                set_ip_input.set(String::new());
                set_reason_input.set(String::new());
                set_status_msg.set(Some("IP successfully added to blocklist".to_string()));
            }
        });
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-extrabold text-white tracking-tight flex items-center gap-2.5">
                        <span>"⛔"</span>
                        "Active Threat Mitigation & IP Blocklist"
                    </h1>
                    <p class="text-xs text-slate-400 mt-1">
                        "Automated firewall synchronization blocking flagged malicious threat actors"
                    </p>
                </div>
                <button
                    on:click=move |_| set_show_modal.set(true)
                    class="px-3.5 py-2 rounded-xl bg-red-600 hover:bg-red-500 text-white text-xs font-semibold shadow-lg shadow-red-600/20 transition flex items-center gap-2"
                >
                    <span>"➕"</span>
                    <span>"Block Host IP"</span>
                </button>
            </div>

            // Status message
            {move || {
                if let Some(msg) = status_msg.get() {
                    view! {
                        <div class="p-3 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs flex items-center justify-between">
                            <span>{msg}</span>
                            <button on:click=move |_| set_status_msg.set(None) class="text-slate-400 hover:text-white">"✕"</button>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Add Block Modal
            {move || {
                if show_modal.get() {
                    view! {
                        <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                            <div class="bg-slate-900 border border-slate-800 rounded-2xl p-6 max-w-md w-full shadow-2xl">
                                <h3 class="text-base font-bold text-white mb-4">"Add Host IP to Blocklist"</h3>
                                <form on:submit=on_add_blocked class="space-y-4">
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"IP Address (CIDR)"</label>
                                        <input
                                            type="text"
                                            required
                                            placeholder="192.168.1.50 or 10.0.0.0/24"
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-red-500 transition"
                                            prop:value=ip_input
                                            on:input=move |e| set_ip_input.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Reason / Incident Reference"</label>
                                        <input
                                            type="text"
                                            required
                                            placeholder="Brute-force SSH attack detected"
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-red-500 transition"
                                            prop:value=reason_input
                                            on:input=move |e| set_reason_input.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div class="flex items-center justify-end gap-3 pt-2">
                                        <button
                                            type="button"
                                            class="px-3.5 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-medium transition"
                                            on:click=move |_| set_show_modal.set(false)
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            type="submit"
                                            class="px-4 py-2 rounded-xl bg-red-600 hover:bg-red-500 text-white text-xs font-semibold transition shadow-lg shadow-red-600/20"
                                        >
                                            "Confirm Block"
                                        </button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Blocklist Table
            <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md overflow-x-auto">
                <table class="w-full text-left text-xs text-slate-300">
                    <thead class="text-slate-400 border-b border-slate-800/80 uppercase tracking-wider font-semibold">
                        <tr>
                            <th class="pb-3 px-3">"Blocked IP / CIDR"</th>
                            <th class="pb-3 px-3">"Mitigation Reason"</th>
                            <th class="pb-3 px-3">"Blocked At"</th>
                            <th class="pb-3 px-3 text-right">"Action"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800/40">
                        <For
                            each=move || blocklist.get()
                            key=|b| b.id
                            children=move |item| {
                                let block_id = item.id;
                                view! {
                                    <tr class="hover:bg-slate-800/30 transition">
                                        <td class="py-3.5 px-3 font-mono font-bold text-red-400 whitespace-nowrap">
                                            {format!("{}", item.ip_address)}
                                        </td>
                                        <td class="py-3.5 px-3 text-slate-200">
                                            {item.reason}
                                        </td>
                                        <td class="py-3.5 px-3 text-slate-500 font-mono text-[11px] whitespace-nowrap">
                                            {item.blocked_at.format("%Y-%m-%d %H:%M:%S").to_string()}
                                        </td>
                                        <td class="py-3.5 px-3 text-right whitespace-nowrap">
                                            <button
                                                class="px-2.5 py-1 rounded-lg bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 text-[11px] font-medium transition"
                                                on:click=move |_| on_unblock(block_id)
                                            >
                                                "Unblock"
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
