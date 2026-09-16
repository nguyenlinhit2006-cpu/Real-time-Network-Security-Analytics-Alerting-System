use common::models::{Alert, AlertSeverity, TrafficSummaryDto};
use leptos::prelude::*;

use crate::api::client::ApiClient;
use crate::components::{SeverityDonut, TrafficChart};

#[component]
pub fn DashboardPage(
    alerts: ReadSignal<Vec<Alert>>,
    throughput: ReadSignal<u64>,
    set_active_tab: WriteSignal<String>,
) -> impl IntoView {
    let (summary, set_summary) = signal::<Option<TrafficSummaryDto>>(None);
    let (blocked_count, set_blocked_count) = signal(0usize);

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(data) = ApiClient::get_dashboard_summary().await {
                set_summary.set(Some(data));
            }
            if let Ok(blocklist) = ApiClient::get_blocklist().await {
                set_blocked_count.set(blocklist.len());
            }
        });
    });

    let critical_alerts = Memo::new(move |_| {
        alerts
            .get()
            .iter()
            .filter(|a| a.severity == AlertSeverity::Critical)
            .count()
    });

    let recent_alerts = Memo::new(move |_| {
        let all = alerts.get();
        all.into_iter().take(5).collect::<Vec<Alert>>()
    });

    let on_export_report = move |_| {
        leptos::task::spawn_local(async move {
            let _ = ApiClient::export_csv_file().await;
        });
    };

    view! {
        <div class="space-y-6">
            // Page Header & Quick Controls
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-extrabold text-white tracking-tight flex items-center gap-2.5">
                        <span class="w-3 h-3 rounded-full bg-emerald-500 animate-pulse"></span>
                        "Security Operations Center (SOC) Overview"
                    </h1>
                    <p class="text-xs text-slate-400 mt-1">
                        "High-speed packet capture stream & automated intrusion detection metrics"
                    </p>
                </div>
                <div class="flex items-center gap-3">
                    <button
                        on:click=on_export_report
                        class="px-3.5 py-2 rounded-xl bg-slate-900 hover:bg-slate-800 text-slate-200 text-xs font-semibold border border-slate-700/80 shadow-sm transition flex items-center gap-2"
                    >
                        <span>"📥"</span>
                        <span>"Export CSV Report"</span>
                    </button>
                    <button
                        on:click=move |_| set_active_tab.set("alerts".to_string())
                        class="px-3.5 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold shadow-lg shadow-indigo-600/20 transition flex items-center gap-2"
                    >
                        <span>"🚨"</span>
                        <span>"Investigate Incidents"</span>
                    </button>
                </div>
            </div>

            // Metrics Cards Grid
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                // 1. Throughput
                <div class="bg-slate-900/80 border border-slate-800/80 rounded-2xl p-5 shadow-lg backdrop-blur-sm">
                    <div class="flex items-center justify-between">
                        <span class="text-xs font-semibold text-slate-400 uppercase tracking-wider">"Current Throughput"</span>
                        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-ping"></span>
                    </div>
                    <div class="mt-2 text-2xl font-bold font-mono text-white">
                        {move || format!("{} B/s", throughput.get())}
                    </div>
                    <div class="mt-2 text-[11px] text-emerald-400 flex items-center gap-1">
                        <span>"⚡ Live pnet capture stream"</span>
                    </div>
                </div>

                // 2. Total Alerts
                <div class="bg-slate-900/80 border border-slate-800/80 rounded-2xl p-5 shadow-lg backdrop-blur-sm">
                    <div class="flex items-center justify-between">
                        <span class="text-xs font-semibold text-slate-400 uppercase tracking-wider">"Total Detected Alerts"</span>
                        <span class="text-lg">"📊"</span>
                    </div>
                    <div class="mt-2 text-2xl font-bold font-mono text-white">
                        {move || alerts.get().len()}
                    </div>
                    <div class="mt-2 text-[11px] text-slate-400">
                        {move || summary.get().map(|s| format!("{} packets analyzed", s.total_packets)).unwrap_or_else(|| "Analyzing stream...".to_string())}
                    </div>
                </div>

                // 3. Critical Incidents
                <div class="bg-slate-900/80 border border-slate-800/80 rounded-2xl p-5 shadow-lg backdrop-blur-sm">
                    <div class="flex items-center justify-between">
                        <span class="text-xs font-semibold text-red-400 uppercase tracking-wider">"Critical Incidents"</span>
                        <span class="text-lg animate-bounce">"🚨"</span>
                    </div>
                    <div class="mt-2 text-2xl font-bold font-mono text-red-400">
                        {move || critical_alerts.get()}
                    </div>
                    <div class="mt-2 text-[11px] text-red-400/80 flex items-center gap-1">
                        <span>"Requires immediate analyst triage"</span>
                    </div>
                </div>

                // 4. Blocked IPs
                <div class="bg-slate-900/80 border border-slate-800/80 rounded-2xl p-5 shadow-lg backdrop-blur-sm">
                    <div class="flex items-center justify-between">
                        <span class="text-xs font-semibold text-slate-400 uppercase tracking-wider">"Active IP Blocklist"</span>
                        <span class="text-lg">"⛔"</span>
                    </div>
                    <div class="mt-2 text-2xl font-bold font-mono text-white">
                        {move || blocked_count.get()}
                    </div>
                    <div class="mt-2 text-[11px] text-emerald-400">
                        "Automated threat mitigation active"
                    </div>
                </div>
            </div>

            // Visual Charts Row
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2">
                    <TrafficChart throughput=throughput />
                </div>
                <div>
                    <SeverityDonut alerts=alerts />
                </div>
            </div>

            // Two-column section: Top 5 Sources & Recent Threat Incidents
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // Top 5 Source IPs
                <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md">
                    <div class="flex items-center justify-between mb-4">
                        <div>
                            <h3 class="text-sm font-bold text-white tracking-tight flex items-center gap-2">
                                <span>"🌐"</span>
                                "Top 5 Traffic Sources"
                            </h3>
                            <p class="text-xs text-slate-400">"Highest packet originators in last 24h"</p>
                        </div>
                    </div>

                    <div class="space-y-3">
                        {move || {
                            if let Some(s) = summary.get() {
                                if s.top_src_ips.is_empty() {
                                    view! { <div class="text-xs text-slate-500 italic">"No traffic captured yet"</div> }.into_any()
                                } else {
                                    view! {
                                        <div class="divide-y divide-slate-800/60">
                                            {s.top_src_ips.into_iter().take(5).map(|item| {
                                                view! {
                                                    <div class="py-2.5 flex items-center justify-between text-xs">
                                                        <span class="font-mono text-slate-200">{item.key}</span>
                                                        <div class="text-right">
                                                            <div class="font-mono font-bold text-emerald-400">{format!("{} pkts", item.count)}</div>
                                                            <div class="text-[10px] text-slate-400">{format!("{} KB", item.bytes / 1024)}</div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                }
                            } else {
                                view! { <div class="text-xs text-slate-500">"Loading traffic summary..."</div> }.into_any()
                            }
                        }}
                    </div>
                </div>

                // Recent Threat Incidents Table Preview (Span 2)
                <div class="lg:col-span-2 bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md">
                    <div class="flex items-center justify-between mb-4">
                        <div>
                            <h3 class="text-sm font-bold text-white tracking-tight flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full bg-red-500"></span>
                                "Latest Threat Incidents"
                            </h3>
                            <p class="text-xs text-slate-400">"Real-time event stream preview"</p>
                        </div>
                        <button
                            on:click=move |_| set_active_tab.set("alerts".to_string())
                            class="text-xs text-indigo-400 hover:text-indigo-300 transition font-medium"
                        >
                            "View All Alerts →"
                        </button>
                    </div>

                    <div class="overflow-x-auto">
                        <table class="w-full text-left text-xs text-slate-300">

                        <thead class="text-slate-400 border-b border-slate-800/80 uppercase tracking-wider font-semibold">
                            <tr>
                                <th class="pb-3 px-3">"Severity"</th>
                                <th class="pb-3 px-3">"Title"</th>
                                <th class="pb-3 px-3">"Source IP"</th>
                                <th class="pb-3 px-3">"Target IP"</th>
                                <th class="pb-3 px-3">"Detected At"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-800/40">
                            <For
                                each=move || recent_alerts.get()
                                key=|a| a.id
                                children=move |alert| {
                                    let sev_badge = match alert.severity {
                                        AlertSeverity::Critical => "bg-red-500/10 text-red-400 border-red-500/30",
                                        AlertSeverity::High => "bg-amber-500/10 text-amber-400 border-amber-500/30",
                                        AlertSeverity::Medium => "bg-blue-500/10 text-blue-400 border-blue-500/30",
                                        AlertSeverity::Low => "bg-slate-500/10 text-slate-400 border-slate-500/30",
                                    };
                                    view! {
                                        <tr class="hover:bg-slate-800/30 transition">
                                            <td class="py-3 px-3 whitespace-nowrap">
                                                <span class=format!("inline-flex items-center px-2 py-0.5 rounded-md text-[10px] font-semibold border {}", sev_badge)>
                                                    {format!("{:?}", alert.severity).to_uppercase()}
                                                </span>
                                            </td>
                                            <td class="py-3 px-3 font-semibold text-slate-200">
                                                {alert.title}
                                            </td>
                                            <td class="py-3 px-3 font-mono text-slate-400">
                                                {format!("{}", alert.src_ip)}
                                            </td>
                                            <td class="py-3 px-3 font-mono text-slate-400">
                                                {format!("{}", alert.dst_ip)}
                                            </td>
                                            <td class="py-3 px-3 text-slate-500 whitespace-nowrap">
                                                {alert.detected_at.format("%Y-%m-%d %H:%M:%S").to_string()}
                                            </td>
                                        </tr>
                                    }
                                }
                            />
                        </tbody>

                    </table>
                </div>
            </div>
        </div>
    </div>
    }
}

