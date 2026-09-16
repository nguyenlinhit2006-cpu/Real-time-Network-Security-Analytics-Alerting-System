use common::models::{UserPublicDto, UserRole};
use leptos::prelude::*;

#[component]
pub fn Sidebar(
    current_user: ReadSignal<Option<UserPublicDto>>,
    active_tab: ReadSignal<String>,
    set_active_tab: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <aside class="w-64 bg-slate-950 border-r border-slate-800/80 p-4 flex flex-col justify-between hidden md:flex min-h-[calc(100vh-4rem)]">
            <div class="space-y-1">
                <div class="px-3 py-2 text-[11px] font-semibold text-slate-500 uppercase tracking-wider">
                    "Operations"
                </div>

                {move || {
                    let user_role = current_user.get().map(|u| u.role).unwrap_or(UserRole::Viewer);

                    let mut nav_items = vec![
                        ("dashboard", "📊 Overview", "Dashboard"),
                        ("alerts", "🚨 Incidents", "Alerts"),
                        ("traffic", "🌐 Traffic Feed", "Traffic"),
                        ("rules", "⚙️ Detection Rules", "Rules"),
                        ("devices", "💻 Devices", "Devices"),
                    ];

                    if user_role == UserRole::Analyst || user_role == UserRole::Admin {
                        nav_items.push(("blocklist", "⛔ Blocklist", "Blocklist"));
                    }

                    if user_role == UserRole::Admin {
                        nav_items.push(("settings", "🔔 Settings", "Settings"));
                    }

                    nav_items.into_iter().map(|(id, label, _)| {
                        let item_id = id.to_string();
                        let is_active = move || active_tab.get() == item_id;

                        view! {
                            <button
                                class=move || {
                                    if is_active() {
                                        "w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-semibold bg-indigo-600/15 text-indigo-400 border border-indigo-500/30 transition shadow-sm"
                                    } else {
                                        "w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-medium text-slate-400 hover:text-slate-200 hover:bg-slate-900/60 transition"
                                    }
                                }
                                on:click=move |_| set_active_tab.set(id.to_string())
                            >
                                <span>{label}</span>
                            </button>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            // Bottom trust status
            <div class="p-4 rounded-xl bg-slate-900/60 border border-slate-800/60 text-center">
                <div class="inline-flex items-center gap-1.5 text-[11px] font-medium text-emerald-400">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
                    "Zero Trust Policy Enforced"
                </div>
                <p class="text-[10px] text-slate-500 mt-1">
                    "OWASP Top 10 Verified"
                </p>
            </div>
        </aside>
    }
}
