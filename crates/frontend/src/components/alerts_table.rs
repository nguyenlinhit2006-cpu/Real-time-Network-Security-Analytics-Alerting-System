use common::models::{Alert, AlertSeverity, AlertStatus};
use leptos::prelude::*;
use uuid::Uuid;

use crate::api::client::ApiClient;

#[component]
pub fn AlertsTable(
    alerts: ReadSignal<Vec<Alert>>,
    set_alerts: WriteSignal<Vec<Alert>>,
) -> impl IntoView {
    let (filter_severity, set_filter_severity) = signal::<Option<AlertSeverity>>(None);
    let (filter_status, set_filter_status) = signal::<Option<AlertStatus>>(None);
    let (search_query, set_search_query) = signal(String::new());

    let filtered_alerts = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        alerts.get().into_iter().filter(|a| {
            if let Some(sev) = filter_severity.get() {
                if a.severity != sev {
                    return false;
                }
            }
            if let Some(st) = filter_status.get() {
                if a.status != st {
                    return false;
                }
            }
            if !query.is_empty() {
                let ip_str = format!("{} {}", a.src_ip, a.dst_ip).to_lowercase();
                let title_str = a.title.to_lowercase();
                if !ip_str.contains(&query) && !title_str.contains(&query) {
                    return false;
                }
            }
            true
        }).collect::<Vec<Alert>>()
    });

    let on_update_status = move |id: Uuid, new_status: AlertStatus| {
        leptos::task::spawn_local(async move {
            if let Ok(updated) = ApiClient::update_alert_status(id, new_status).await {
                set_alerts.update(|list| {
                    if let Some(pos) = list.iter().position(|item| item.id == id) {
                        list[pos] = updated;
                    }
                });
            }
        });
    };

    view! {
        <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md">
            // Header and controls
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-6">
                <div>
                    <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
                        <span class="w-2.5 h-2.5 rounded-full bg-red-500 animate-pulse"></span>
                        "Security Incidents & Alerts"
                    </h2>
                    <p class="text-xs text-slate-400 mt-0.5">
                        "Real-time event analysis and automated incident response feed"
                    </p>
                </div>

                // Search and filters
                <div class="flex flex-wrap items-center gap-2">
                    <input
                        type="text"
                        placeholder="Search IP, incident..."
                        class="bg-slate-950 border border-slate-800 rounded-xl px-3 py-1.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 w-48 transition"
                        on:input=move |e| set_search_query.set(event_target_value(&e))
                    />

                    // Severity filter
                    <select
                        class="bg-slate-950 border border-slate-800 rounded-xl px-3 py-1.5 text-xs text-slate-300 focus:outline-none focus:border-indigo-500 transition"
                        on:change=move |e| {
                            let val = event_target_value(&e);
                            match val.as_str() {
                                "critical" => set_filter_severity.set(Some(AlertSeverity::Critical)),
                                "high" => set_filter_severity.set(Some(AlertSeverity::High)),
                                "medium" => set_filter_severity.set(Some(AlertSeverity::Medium)),
                                "low" => set_filter_severity.set(Some(AlertSeverity::Low)),
                                _ => set_filter_severity.set(None),
                            }
                        }
                    >
                        <option value="all">"All Severities"</option>
                        <option value="critical">"Critical Only"</option>
                        <option value="high">"High Only"</option>
                        <option value="medium">"Medium Only"</option>
                        <option value="low">"Low Only"</option>
                    </select>

                    // Status filter
                    <select
                        class="bg-slate-950 border border-slate-800 rounded-xl px-3 py-1.5 text-xs text-slate-300 focus:outline-none focus:border-indigo-500 transition"
                        on:change=move |e| {
                            let val = event_target_value(&e);
                            match val.as_str() {
                                "open" => set_filter_status.set(Some(AlertStatus::Open)),
                                "acknowledged" => set_filter_status.set(Some(AlertStatus::Acknowledged)),
                                "resolved" => set_filter_status.set(Some(AlertStatus::Resolved)),
                                _ => set_filter_status.set(None),
                            }
                        }
                    >
                        <option value="all">"All Statuses"</option>
                        <option value="open">"Open"</option>
                        <option value="acknowledged">"Acknowledged"</option>
                        <option value="resolved">"Resolved"</option>
                    </select>
                </div>
            </div>

            // Table Content
            <div class="overflow-x-auto">
                <table class="w-full text-left text-xs text-slate-300">
                    <thead class="text-slate-400 border-b border-slate-800/80 uppercase tracking-wider font-semibold">
                        <tr>
                            <th class="pb-3 px-3">"Severity"</th>
                            <th class="pb-3 px-3">"Title & Description"</th>
                            <th class="pb-3 px-3">"Source IP"</th>
                            <th class="pb-3 px-3">"Target IP"</th>
                            <th class="pb-3 px-3">"Status"</th>
                            <th class="pb-3 px-3 text-right">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800/40">
                        <For
                            each=move || filtered_alerts.get()
                            key=|alert| alert.id
                            children=move |alert| {
                                let alert_id = alert.id;
                                let sev_class = match alert.severity {
                                    AlertSeverity::Critical => "bg-red-500/10 text-red-400 border-red-500/30",
                                    AlertSeverity::High => "bg-amber-500/10 text-amber-400 border-amber-500/30",
                                    AlertSeverity::Medium => "bg-blue-500/10 text-blue-400 border-blue-500/30",
                                    AlertSeverity::Low => "bg-slate-500/10 text-slate-400 border-slate-500/30",
                                };

                                let status_badge = match alert.status {
                                    AlertStatus::Open => "bg-red-500/10 text-red-400 border border-red-500/20",
                                    AlertStatus::Acknowledged => "bg-amber-500/10 text-amber-400 border border-amber-500/20",
                                    AlertStatus::Resolved => "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20",
                                };

                                view! {
                                    <tr class="hover:bg-slate-800/30 transition group">
                                        // Severity Pill
                                        <td class="py-3.5 px-3 whitespace-nowrap">
                                            <span class=format!("inline-flex items-center px-2 py-0.5 rounded-md text-[11px] font-semibold border {}", sev_class)>
                                                {format!("{:?}", alert.severity).to_uppercase()}
                                            </span>
                                        </td>

                                        // Title & Description
                                        <td class="py-3.5 px-3 max-w-sm">
                                            <div class="font-semibold text-slate-100 group-hover:text-indigo-300 transition">
                                                {alert.title}
                                            </div>
                                            <div class="text-slate-400 text-[11px] truncate mt-0.5">
                                                {alert.description}
                                            </div>
                                        </td>

                                        // Source IP
                                        <td class="py-3.5 px-3 font-mono text-slate-300 whitespace-nowrap">
                                            {format!("{}", alert.src_ip)}
                                        </td>

                                        // Target IP
                                        <td class="py-3.5 px-3 font-mono text-slate-300 whitespace-nowrap">
                                            {format!("{}", alert.dst_ip)}
                                        </td>

                                        // Status
                                        <td class="py-3.5 px-3 whitespace-nowrap">
                                            <span class=format!("inline-flex items-center px-2 py-0.5 rounded-full text-[11px] font-medium {}", status_badge)>
                                                {format!("{:?}", alert.status)}
                                            </span>
                                        </td>

                                        // Actions
                                        <td class="py-3.5 px-3 text-right whitespace-nowrap">
                                            <div class="inline-flex items-center gap-1.5">
                                                {if alert.status == AlertStatus::Open {
                                                    view! {
                                                        <button
                                                            class="px-2.5 py-1 rounded-lg bg-amber-500/10 hover:bg-amber-500/20 text-amber-400 border border-amber-500/30 text-[11px] font-medium transition"
                                                            on:click=move |_| on_update_status(alert_id, AlertStatus::Acknowledged)
                                                        >
                                                            "Acknowledge"
                                                        </button>
                                                    }.into_any()
                                                } else if alert.status == AlertStatus::Acknowledged {
                                                    view! {
                                                        <button
                                                            class="px-2.5 py-1 rounded-lg bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 text-[11px] font-medium transition"
                                                            on:click=move |_| on_update_status(alert_id, AlertStatus::Resolved)
                                                        >
                                                            "Resolve"
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <span class="text-slate-600 text-[11px] italic">"Closed"</span>
                                                    }.into_any()
                                                }}
                                            </div>
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

