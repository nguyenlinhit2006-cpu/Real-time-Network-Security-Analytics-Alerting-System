use common::models::{AlertSeverity, ChannelType, CreateNotificationChannelDto, NotificationChannel};
use leptos::prelude::*;
use uuid::Uuid;

use crate::api::client::ApiClient;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let (channels, set_channels) = signal::<Vec<NotificationChannel>>(Vec::new());
    let (status_msg, set_status_msg) = signal::<Option<String>>(None);
    let (show_modal, set_show_modal) = signal(false);

    let (new_name, set_new_name) = signal(String::new());
    let (new_type, set_new_type) = signal(ChannelType::Webhook);
    let (webhook_url, set_webhook_url) = signal(String::new());

    let load_channels = move || {
        leptos::task::spawn_local(async move {
            if let Ok(data) = ApiClient::get_notification_channels().await {
                set_channels.set(data);
            }
        });
    };

    Effect::new(move |_| {
        load_channels();
    });

    let on_test_channel = move |id: Uuid| {
        set_status_msg.set(Some("Sending test alert...".to_string()));
        leptos::task::spawn_local(async move {
            match ApiClient::test_notification_channel(id).await {
                Ok(msg) => set_status_msg.set(Some(format!("✓ {}", msg))),
                Err(err) => set_status_msg.set(Some(format!("⚠ {}", err))),
            }
        });
    };

    let on_delete_channel = move |id: Uuid| {
        leptos::task::spawn_local(async move {
            if ApiClient::delete_notification_channel(id).await.is_ok() {
                set_channels.update(|list| list.retain(|c| c.id != id));
                set_status_msg.set(Some("Notification channel removed".to_string()));
            }
        });
    };

    let on_create_channel = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        let name = new_name.get();
        let ctype = new_type.get();
        let url = webhook_url.get();

        let config = serde_json::json!({
            "url": url,
            "secret": "secnet_sign_key"
        });

        leptos::task::spawn_local(async move {
            let dto = CreateNotificationChannelDto {
                name,
                r#type: ctype,
                config_json: config,
                min_severity: AlertSeverity::High,
                is_enabled: Some(true),
            };
            if let Ok(c) = ApiClient::create_notification_channel(&dto).await {
                set_channels.update(|list| list.push(c));
                set_show_modal.set(false);
                set_new_name.set(String::new());
                set_webhook_url.set(String::new());
                set_status_msg.set(Some("Channel created successfully".to_string()));
            }
        });
    };

    view! {
        <div class="space-y-6">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                    <h1 class="text-2xl font-extrabold text-white tracking-tight flex items-center gap-2.5">
                        <span>"🔔"</span>
                        "Multi-Channel Alerting Settings"
                    </h1>
                    <p class="text-xs text-slate-400 mt-1">
                        "Automated push notifications via Email (SMTP), Webhook, Telegram Bot, or Slack"
                    </p>
                </div>
                <button
                    on:click=move |_| set_show_modal.set(true)
                    class="px-3.5 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold shadow-lg shadow-indigo-600/20 transition flex items-center gap-2"
                >
                    <span>"➕"</span>
                    <span>"Add Notification Channel"</span>
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

            // Add Channel Modal
            {move || {
                if show_modal.get() {
                    view! {
                        <div class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                            <div class="bg-slate-900 border border-slate-800 rounded-2xl p-6 max-w-md w-full shadow-2xl">
                                <h3 class="text-base font-bold text-white mb-4">"Add Notification Channel"</h3>
                                <form on:submit=on_create_channel class="space-y-4">
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Channel Name"</label>
                                        <input
                                            type="text"
                                            required
                                            placeholder="Security Team Discord / Telegram Bot"
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition"
                                            prop:value=new_name
                                            on:input=move |e| set_new_name.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Channel Type"</label>
                                        <select
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-300 focus:outline-none focus:border-indigo-500 transition"
                                            on:change=move |e| {
                                                let val = event_target_value(&e);
                                                match val.as_str() {
                                                    "webhook" => set_new_type.set(ChannelType::Webhook),
                                                    "telegram" => set_new_type.set(ChannelType::Telegram),
                                                    "email" => set_new_type.set(ChannelType::Email),
                                                    _ => set_new_type.set(ChannelType::Webhook),
                                                }
                                            }
                                        >
                                            <option value="webhook">"Webhook (HTTP POST / Discord / Slack)"</option>
                                            <option value="telegram">"Telegram Bot"</option>
                                            <option value="email">"Email (SMTP)"</option>
                                        </select>
                                    </div>
                                    <div>
                                        <label class="block text-xs font-semibold text-slate-300 mb-1.5">"Target Endpoint / Webhook URL"</label>
                                        <input
                                            type="text"
                                            required
                                            placeholder="https://hooks.example.com/alerts"
                                            class="w-full bg-slate-950 border border-slate-800 rounded-xl px-3.5 py-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition"
                                            prop:value=webhook_url
                                            on:input=move |e| set_webhook_url.set(event_target_value(&e))
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
                                            class="px-4 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold transition shadow-lg shadow-indigo-600/20"
                                        >
                                            "Save Channel"
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

            // Channels List Grid
            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                <For
                    each=move || channels.get()
                    key=|c| c.id
                    children=move |channel| {
                        let chan_id = channel.id;
                        let type_icon = match channel.r#type {
                            ChannelType::Email => "📧 Email (SMTP)",
                            ChannelType::Webhook => "🔗 Webhook",
                            ChannelType::Telegram => "✈️ Telegram Bot",
                            ChannelType::Slack => "💬 Slack",
                        };

                        view! {
                            <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-xl backdrop-blur-md flex flex-col justify-between space-y-4">
                                <div>
                                    <div class="flex items-center justify-between gap-3 mb-2">
                                        <span class="text-xs font-mono font-bold text-indigo-400 uppercase bg-indigo-500/10 px-2.5 py-1 rounded-lg border border-indigo-500/20">
                                            {type_icon}
                                        </span>
                                        <span class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                                            "● Active"
                                        </span>
                                    </div>
                                    <h3 class="text-base font-bold text-white tracking-tight">
                                        {channel.name}
                                    </h3>
                                    <div class="text-[11px] font-mono text-slate-400 mt-1 truncate">
                                        {format!("{}", channel.config_json)}
                                    </div>
                                </div>

                                <div class="flex items-center justify-between pt-2 border-t border-slate-800/60">
                                    <button
                                        class="px-3 py-1.5 rounded-xl bg-indigo-600/20 hover:bg-indigo-600/30 text-indigo-400 border border-indigo-500/30 text-xs font-medium transition"
                                        on:click=move |_| on_test_channel(chan_id)
                                    >
                                        "⚡ Send Test Alert"
                                    </button>
                                    <button
                                        class="text-xs text-red-400 hover:text-red-300 transition"
                                        on:click=move |_| on_delete_channel(chan_id)
                                    >
                                        "Delete"
                                    </button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

        </div>
    }
}
