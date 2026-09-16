use common::models::UserPublicDto;
use leptos::prelude::*;
use crate::api::client::ApiClient;

#[component]
pub fn Navbar(
    current_user: ReadSignal<Option<UserPublicDto>>,
    set_current_user: WriteSignal<Option<UserPublicDto>>,
    set_active_tab: WriteSignal<String>,
    is_ws_connected: ReadSignal<bool>,
    critical_count: Signal<usize>,
) -> impl IntoView {

    let on_logout = move |_| {
        ApiClient::logout();
        set_current_user.set(None);
        set_active_tab.set("login".to_string());
    };

    let on_sign_in = move |_| {
        set_active_tab.set("login".to_string());
    };

    let (is_dark, set_is_dark) = signal(true);
    let on_toggle_theme = move |_| {
        set_is_dark.update(|d| *d = !*d);
        let _ = js_sys::eval("document.documentElement.classList.toggle('dark')");
    };

    view! {
        <header class="h-16 bg-slate-900/95 border-b border-slate-800/80 px-6 flex items-center justify-between sticky top-0 z-40 backdrop-blur-md">
            // Brand Logo & Status
            <div class="flex items-center gap-4">
                <button
                    on:click=move |_| {
                        if current_user.get().is_some() {
                            set_active_tab.set("dashboard".to_string());
                        } else {
                            set_active_tab.set("login".to_string());
                        }
                    }
                    class="flex items-center gap-2.5 text-left hover:opacity-90 transition focus:outline-none"
                >
                    <div class="w-9 h-9 rounded-xl bg-gradient-to-tr from-indigo-600 to-emerald-500 flex items-center justify-center text-white shadow-lg shadow-indigo-500/20">
                        <span class="text-lg">"🛡️"</span>
                    </div>
                    <div>
                        <h1 class="text-sm font-bold text-white tracking-tight leading-none">
                            "SecNet Analytics"
                        </h1>
                        <span class="text-[10px] text-slate-400 font-mono">
                            "v1.0.0-phase5"
                        </span>
                    </div>
                </button>

                // WS Status Badge
                <div class="hidden sm:flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[11px] font-medium border bg-slate-950/80 border-slate-800">
                    {move || if is_ws_connected.get() {
                        view! {
                            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
                            <span class="text-emerald-400">"LIVE WS CONNECTED"</span>
                        }.into_any()
                    } else {
                        view! {
                            <span class="w-2 h-2 rounded-full bg-amber-500"></span>
                            <span class="text-amber-400">"RECONNECTING..."</span>
                        }.into_any()
                    }}
                </div>
            </div>

            // Right side: Theme Toggle, Critical Alert Counter, User, Logout
            <div class="flex items-center gap-3">
                // Dark/Light Theme Toggle
                <button
                    on:click=on_toggle_theme
                    class="p-2 rounded-xl bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 text-xs transition"
                    title="Toggle Theme"
                >
                    {move || if is_dark.get() { "🌙" } else { "☀️" }}
                </button>

                // Critical Alert Badge
                {move || {
                    let count = critical_count.get();
                    if count > 0 {
                        view! {
                            <div class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-red-500/10 border border-red-500/30 text-red-400 text-xs font-semibold animate-pulse">
                                <span>"🚨"</span>
                                <span>{format!("{} CRITICAL", count)}</span>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}

                // User Profile
                {move || match current_user.get() {
                    Some(u) => view! {
                        <div class="flex items-center gap-3">
                            <div class="text-right hidden md:block">
                                <div class="text-xs font-semibold text-slate-200">{u.username}</div>
                                <div class="text-[10px] font-mono text-indigo-400 uppercase">{format!("{:?}", u.role)}</div>
                            </div>
                            <button
                                class="px-3 py-1.5 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-medium border border-slate-700/60 transition"
                                on:click=on_logout
                            >
                                "Logout"
                            </button>
                        </div>
                    }.into_any(),
                    None => view! {
                        <button
                            on:click=on_sign_in
                            class="px-3 py-1.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-medium transition"
                        >
                            "Sign In"
                        </button>
                    }.into_any(),
                }}
            </div>
        </header>
    }
}
