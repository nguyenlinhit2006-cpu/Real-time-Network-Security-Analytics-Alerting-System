use common::models::{AlertSeverity, CreateRuleDto, DetectionRule, RuleType, UpdateRuleDto};
use leptos::prelude::*;
use uuid::Uuid;

use crate::api::client::ApiClient;

#[component]
pub fn RulesPage() -> impl IntoView {
    let (rules, set_rules) = signal::<Vec<DetectionRule>>(Vec::new());
    let (_is_loading, set_is_loading) = signal(true);
    let (status_msg, set_status_msg) = signal::<Option<String>>(None);
    let (show_create_modal, set_show_create_modal) = signal(false);

    // Form fields for new rule
    let (new_name, set_new_name) = signal(String::new());
    let (new_rule_type, set_new_rule_type) = signal(RuleType::Threshold);
    let (new_severity, set_new_severity) = signal(AlertSeverity::High);
    let (new_threshold, set_new_threshold) = signal(100.0f64);
    let (new_window, set_new_window) = signal(60i32);

    let load_rules = move || {
        set_is_loading.set(true);
        leptos::task::spawn_local(async move {
            if let Ok(data) = ApiClient::get_rules().await {
                set_rules.set(data);
            }
            set_is_loading.set(false);
        });
    };

    Effect::new(move |_| {
        load_rules();
    });

    let on_toggle_rule = move |id: Uuid, current_state: bool| {
        let new_state = !current_state;
        leptos::task::spawn_local(async move {
            let dto = UpdateRuleDto {
                name: None,
                rule_type: None,
                condition_json: None,
                severity: None,
                is_enabled: Some(new_state),
                threshold_value: None,
                time_window_seconds: None,
            };
            if let Ok(updated) = ApiClient::update_rule(id, &dto).await {
                set_rules.update(|list| {
                    if let Some(pos) = list.iter().position(|r| r.id == id) {
                        list[pos] = updated;
                    }
                });
                set_status_msg.set(Some(format!("Rule successfully {}", if new_state { "enabled" } else { "disabled" })));
            }
        });
    };

    let on_create_rule = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        let name = new_name.get();
        let rtype = new_rule_type.get();
        let sev = new_severity.get();
        let thresh = new_threshold.get();
        let win = new_window.get();

        leptos::task::spawn_local(async move {
            let dto = CreateRuleDto {
                name,
                rule_type: rtype,
                condition_json: serde_json::json!({ "metric": "packet_rate", "operator": ">=" }),
                severity: sev,
                is_enabled: Some(true),
                threshold_value: thresh,
                time_window_seconds: win,
            };

            if let Ok(created) = ApiClient::create_rule(&dto).await {
                set_rules.update(|list| list.push(created));
                set_show_create_modal.set(false);
                set_new_name.set(String::new());
                set_status_msg.set(Some("Rule created successfully".to_string()));
            }
        });
    };

    view! {
        <div class="space-y-6">
            // Header
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-extrabold text-white tracking-tight flex items-center gap-2.5">
                        <span>"⚙️"</span>
                        "Intrusion Detection Engine Rules"
                    </h1>
                    <p class="text-xs text-slate-400 mt-1">
                        "Manage multi-pattern detection thresholds, time windows, and real-time triggers."
                    </p>
                </div>
                <button
                    on:click=move |_| set_show_create_modal.set(true)
                    class="px-3.5 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold shadow-lg shadow-indigo-600/20 transition flex items-center gap-2"
                >
                    <span>"➕"</span>
                    <span>"Create Detection Rule"</span>
                </button>
            </div>

            // Status message
            {move || {
                if let Some(msg) = status_msg.get() {
                    view! {
                        <div class="p-3 rounded-xl bg-indigo-500/10 border border-indigo-500/30 text-indigo-300 text-xs flex items-center justify-between">
                            <span>{msg}</span>
                            <button on:click=move |_| set_status_msg.set(None) class="text-slate-400 hover:text-white">"✕"</button>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Create Rule Modal
            {move || {
                if show_create_modal.get() {
                    view! {
                        <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                            <div class="bg-slate-900 border border-slate-800 rounded-2xl p-6 max-w-md w-full shadow-2xl">
                                <h3 class="text-base font-bold text-white mb-4">"New Detection Rule"</h3>
                                <form on:submit=on_create_rule class="space-y-4">
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Rule Name"</label>
                                        <input
                                            type="text"
                                            required
                                            placeholder="e.g. Excessive DNS Query Flood"
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition"
                                            prop:value=new_name
                                            on:input=move |e| set_new_name.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div class="grid grid-cols-2 gap-3">
                                        <div>
                                            <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Rule Type"</label>
                                            <select
                                                class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3 py-2 text-xs text-slate-300 focus:outline-none focus:border-indigo-500 transition"
                                                on:change=move |e| {
                                                    match event_target_value(&e).as_str() {
                                                        "pattern" => set_new_rule_type.set(RuleType::Pattern),
                                                        "anomaly" => set_new_rule_type.set(RuleType::Anomaly),
                                                        _ => set_new_rule_type.set(RuleType::Threshold),
                                                    }
                                                }
                                            >
                                                <option value="threshold">"Threshold"</option>
                                                <option value="pattern">"Pattern"</option>
                                                <option value="anomaly">"Anomaly"</option>
                                            </select>
                                        </div>
                                        <div>
                                            <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Severity"</label>
                                            <select
                                                class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3 py-2 text-xs text-slate-300 focus:outline-none focus:border-indigo-500 transition"
                                                on:change=move |e| {
                                                    match event_target_value(&e).as_str() {
                                                        "critical" => set_new_severity.set(AlertSeverity::Critical),
                                                        "high" => set_new_severity.set(AlertSeverity::High),
                                                        "medium" => set_new_severity.set(AlertSeverity::Medium),
                                                        _ => set_new_severity.set(AlertSeverity::Low),
                                                    }
                                                }
                                            >
                                                <option value="high">"High"</option>
                                                <option value="critical">"Critical"</option>
                                                <option value="medium">"Medium"</option>
                                                <option value="low">"Low"</option>
                                            </select>
                                        </div>
                                    </div>
                                    <div class="grid grid-cols-2 gap-3">
                                        <div>
                                            <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Threshold Value"</label>
                                            <input
                                                type="number"
                                                required
                                                min="1"
                                                class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 focus:outline-none focus:border-indigo-500 transition"
                                                prop:value=new_threshold
                                                on:input=move |e| {
                                                    if let Ok(val) = event_target_value(&e).parse::<f64>() {
                                                        set_new_threshold.set(val);
                                                    }
                                                }
                                            />
                                        </div>
                                        <div>
                                            <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Window (seconds)"</label>
                                            <input
                                                type="number"
                                                required
                                                min="1"
                                                class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 focus:outline-none focus:border-indigo-500 transition"
                                                prop:value=new_window
                                                on:input=move |e| {
                                                    if let Ok(val) = event_target_value(&e).parse::<i32>() {
                                                        set_new_window.set(val);
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                    <div class="flex items-center justify-end gap-3 pt-2">
                                        <button
                                            type="button"
                                            class="px-3.5 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-medium transition"
                                            on:click=move |_| set_show_create_modal.set(false)
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            type="submit"
                                            class="px-4 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold transition shadow-lg shadow-indigo-600/20"
                                        >
                                            "Save Rule"
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

            // Rules Cards Grid
            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                <For
                    each=move || rules.get()
                    key=|r| r.id
                    children=move |rule| {
                        let rule_id = rule.id;
                        let is_enabled = rule.is_enabled;
                        let current_thresh = rule.threshold_value;
                        let current_window = rule.time_window_seconds;

                        let sev_badge = match rule.severity {
                            AlertSeverity::Critical => "bg-red-500/10 text-red-400 border-red-500/30",
                            AlertSeverity::High => "bg-amber-500/10 text-amber-400 border-amber-500/30",
                            AlertSeverity::Medium => "bg-blue-500/10 text-blue-400 border-blue-500/30",
                            AlertSeverity::Low => "bg-slate-500/10 text-slate-400 border-slate-500/30",
                        };

                        let on_quick_adjust = move |delta: f64| {
                            leptos::task::spawn_local(async move {
                                let new_t = (current_thresh + delta).max(1.0);
                                let dto = UpdateRuleDto {
                                    name: None,
                                    rule_type: None,
                                    condition_json: None,
                                    severity: None,
                                    is_enabled: None,
                                    threshold_value: Some(new_t),
                                    time_window_seconds: None,
                                };
                                if let Ok(updated) = ApiClient::update_rule(rule_id, &dto).await {
                                    set_rules.update(|list| {
                                        if let Some(pos) = list.iter().position(|r| r.id == rule_id) {
                                            list[pos] = updated;
                                        }
                                    });
                                }
                            });
                        };

                        view! {
                            <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-xl backdrop-blur-md flex flex-col justify-between space-y-4">
                                <div>
                                    <div class="flex items-center justify-between gap-3 mb-2">
                                        <div class="flex items-center gap-2">
                                            <span class=format!("inline-flex items-center px-2 py-0.5 rounded text-[10px] font-bold border {}", sev_badge)>
                                                {format!("{:?}", rule.severity).to_uppercase()}
                                            </span>
                                            <span class="text-[10px] font-mono text-indigo-400 uppercase bg-indigo-500/10 px-2 py-0.5 rounded border border-indigo-500/20">
                                                {format!("{:?}", rule.rule_type)}
                                            </span>
                                        </div>

                                        // Toggle Switch
                                        <button
                                            class=move || {
                                                if is_enabled {
                                                    "px-3 py-1 rounded-full text-[11px] font-semibold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 transition"
                                                } else {
                                                    "px-3 py-1 rounded-full text-[11px] font-semibold bg-slate-800 text-slate-400 border border-slate-700 transition"
                                                }
                                            }
                                            on:click=move |_| on_toggle_rule(rule_id, is_enabled)
                                        >
                                            {if is_enabled { "Active ●" } else { "Disabled ○" }}
                                        </button>
                                    </div>

                                    <h3 class="text-base font-bold text-white tracking-tight">
                                        {rule.name}
                                    </h3>
                                </div>

                                // Metrics details & quick adjust buttons
                                <div class="grid grid-cols-2 gap-3 p-3 bg-slate-950/60 rounded-xl border border-slate-800/60 text-xs">
                                    <div>
                                        <div class="text-[10px] text-slate-500 uppercase font-semibold">"Trigger Threshold"</div>
                                        <div class="flex items-center justify-between mt-1">
                                            <span class="font-mono font-bold text-slate-200">
                                                {format!("{:.0} events", current_thresh)}
                                            </span>
                                            <div class="flex items-center gap-1">
                                                <button
                                                    on:click=move |_| on_quick_adjust(-10.0)
                                                    class="px-1.5 py-0.5 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded text-[10px]"
                                                >
                                                    "-10"
                                                </button>
                                                <button
                                                    on:click=move |_| on_quick_adjust(10.0)
                                                    class="px-1.5 py-0.5 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded text-[10px]"
                                                >
                                                    "+10"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    <div>
                                        <div class="text-[10px] text-slate-500 uppercase font-semibold">"Time Window"</div>
                                        <div class="font-mono font-bold text-slate-200 mt-1">
                                            {format!("{} seconds", current_window)}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
