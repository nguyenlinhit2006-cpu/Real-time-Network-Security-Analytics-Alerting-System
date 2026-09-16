use common::models::{CreateUserDto, LoginDto, UserPublicDto, UserRole};
use leptos::prelude::*;

use crate::api::client::ApiClient;

#[component]
pub fn LoginPage(
    set_active_tab: WriteSignal<String>,
    set_current_user: WriteSignal<Option<UserPublicDto>>,
) -> impl IntoView {
    let (is_register, set_is_register) = signal(false);
    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (role, set_role) = signal(UserRole::Analyst);
    let (error_msg, set_error_msg) = signal::<Option<String>>(None);
    let (is_loading, set_is_loading) = signal(false);

    // If already authenticated, go directly to dashboard
    Effect::new(move |_| {
        if ApiClient::is_authenticated() {
            if let Some(user) = ApiClient::get_current_user() {
                set_current_user.set(Some(user));
            }
            set_active_tab.set("dashboard".to_string());
        }
    });

    let on_submit = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        set_error_msg.set(None);
        set_is_loading.set(true);

        let u = username.get();
        let p = password.get();
        let em = email.get();
        let r = role.get();
        let reg = is_register.get();

        leptos::task::spawn_local(async move {
            if reg {
                let dto = CreateUserDto {
                    username: u,
                    email: em,
                    password: p,
                    role: Some(r),
                };
                match ApiClient::register(&dto).await {
                    Ok(auth_data) => {
                        set_is_loading.set(false);
                        set_current_user.set(Some(auth_data.user));
                        set_active_tab.set("dashboard".to_string());
                        if let Some(window) = web_sys::window() {
                            let _ = window.history().and_then(|h| h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/")));
                        }
                    }
                    Err(err) => {
                        set_is_loading.set(false);
                        set_error_msg.set(Some(err));
                    }
                }
            } else {
                let dto = LoginDto {
                    username: u,
                    password: p,
                };
                match ApiClient::login(&dto).await {
                    Ok(auth_data) => {
                        set_is_loading.set(false);
                        set_current_user.set(Some(auth_data.user));
                        set_active_tab.set("dashboard".to_string());
                        if let Some(window) = web_sys::window() {
                            let _ = window.history().and_then(|h| h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/")));
                        }
                    }
                    Err(err) => {
                        set_is_loading.set(false);
                        set_error_msg.set(Some(err));
                    }
                }
            }
        });
    };

    view! {
        <div class="min-h-[calc(100vh-4rem)] flex items-center justify-center p-6 bg-slate-950">
            <div class="w-full max-w-md bg-slate-900/90 border border-slate-800/80 rounded-2xl p-8 shadow-2xl backdrop-blur-md">
                // Header
                <div class="text-center mb-8">
                    <div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-indigo-600/10 border border-indigo-500/20 text-indigo-400 mb-4">
                        <span class="text-2xl">"🛡️"</span>
                    </div>
                    <h2 class="text-xl font-bold text-white tracking-tight">
                        {move || if is_register.get() { "Create SecNet Account" } else { "SecNet Security Portal" }}
                    </h2>
                    <p class="text-xs text-slate-400 mt-1">
                        "Real-time packet analytics & intrusion response"
                    </p>
                </div>

                // Error alert
                {move || {
                    if let Some(err) = error_msg.get() {
                        view! {
                            <div class="mb-5 p-3 rounded-xl bg-red-500/10 border border-red-500/30 text-red-400 text-xs flex items-center gap-2">
                                <span>"⚠️"</span>
                                <span>{err}</span>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}

                // Auth Form
                <form on:submit=on_submit class="space-y-4">
                    <div>
                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Username"</label>
                        <input
                            type="text"
                            required
                            placeholder="e.g. admin or analyst"
                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition"
                            prop:value=username
                            on:input=move |e| set_username.set(event_target_value(&e))
                        />
                    </div>

                    {move || {
                        if is_register.get() {
                            view! {
                                <div class="space-y-4">
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Email Address"</label>
                                        <input
                                            type="email"
                                            required
                                            placeholder="analyst@secnet.internal"
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition"
                                            prop:value=email
                                            on:input=move |e| set_email.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Role"</label>
                                        <select
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-300 focus:outline-none focus:border-indigo-500 transition"
                                            on:change=move |e| {
                                                let val = event_target_value(&e);
                                                match val.as_str() {
                                                    "admin" => set_role.set(UserRole::Admin),
                                                    "analyst" => set_role.set(UserRole::Analyst),
                                                    _ => set_role.set(UserRole::Viewer),
                                                }
                                            }
                                        >
                                            <option value="analyst">"Analyst (Alert resolution, rule view)"</option>
                                            <option value="viewer">"Viewer (Read-only operations)"</option>
                                            <option value="admin">"Admin (Full configuration)"</option>
                                        </select>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}

                    <div>
                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Password"</label>
                        <input
                            type="password"
                            required
                            placeholder="••••••••"
                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition"
                            prop:value=password
                            on:input=move |e| set_password.set(event_target_value(&e))
                        />
                    </div>

                    <button
                        type="submit"
                        disabled=move || is_loading.get()
                        class="w-full mt-2 py-2.5 px-4 rounded-xl bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white text-xs font-semibold shadow-lg shadow-indigo-600/20 transition flex items-center justify-center gap-2"
                    >
                        {move || if is_loading.get() {
                            view! { <span>"Authenticating..."</span> }.into_any()
                        } else if is_register.get() {
                            view! { <span>"Create Account"</span> }.into_any()
                        } else {
                            view! { <span>"Sign In to Dashboard"</span> }.into_any()
                        }}
                    </button>
                </form>

                // Toggle register/login
                <div class="mt-6 pt-6 border-t border-slate-800/80 text-center">
                    <button
                        class="text-xs text-indigo-400 hover:text-indigo-300 transition"
                        on:click=move |_| {
                            set_is_register.update(|v| *v = !*v);
                            set_error_msg.set(None);
                        }
                    >
                        {move || if is_register.get() {
                            "Already have an account? Sign in"
                        } else {
                            "Need an account? Register new user"
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}
