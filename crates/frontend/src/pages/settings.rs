use common::models::{AlertSeverity, ChannelType, CreateNotificationChannelDto, NotificationChannel};
use leptos::prelude::*;
use uuid::Uuid;

use crate::api::client::ApiClient;
use crate::components::icons::{IconBell, IconClose, IconPlus};

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
                Ok(msg) => set_status_msg.set(Some(msg)),
                Err(err) => set_status_msg.set(Some(err)),
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
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-5 border-b border-ink-700">
                <div>
                    <h1 class="text-2xl font-bold text-white tracking-tight">
                        "Alert channels"
                    </h1>
                    <p class="text-xs text-ink-500 mt-1">
                        "Automated push notifications via Email (SMTP), Webhook, Telegram Bot, or Slack"
                    </p>
                </div>
                <button
                    on:click=move |_| set_show_modal.set(true)
                    class="px-3.5 py-2 rounded-md bg-brand hover:bg-brand-light text-ink-950 text-xs font-bold transition-colors flex items-center gap-2"
                >
                    <IconPlus class="w-3.5 h-3.5".to_string() />
                    <span>"Add channel"</span>
                </button>
            </div>

            // Status message
            {move || {
                if let Some(msg) = status_msg.get() {
                    view! {
                        <div class="p-3 rounded-md bg-brand/10 border border-brand/30 text-brand text-xs font-mono flex items-center justify-between">
                            <span>{msg}</span>
                            <button on:click=move |_| set_status_msg.set(None) class="text-ink-500 hover:text-white"><IconClose class="w-3 h-3".to_string() /></button>
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
                            <div class="bg-ink-900 border border-ink-600 rounded-lg p-6 max-w-md w-full shadow-console">
                                <h3 class="text-base font-bold text-white mb-4">"Add notification channel"</h3>
                                <form on:submit=on_create_channel class="space-y-4">
                                    <div>
                                        <label class="block text-[11px] font-mono font-semibold text-ink-500 uppercase tracking-wide mb-1.5">"Channel name"</label>
                                        <input
                                            type="text"
                                            required
                                            placeholder="Security Team Discord / Telegram Bot"
                                            class="w-full bg-ink-950 border border-ink-600 rounded-md px-3.5 py-2 text-xs text-slate-200 placeholder-ink-600 focus:outline-none focus:border-brand/60 transition-colors"
                                            prop:value=new_name
                                            on:input=move |e| set_new_name.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-[11px] font-mono font-semibold text-ink-500 uppercase tracking-wide mb-1.5">"Channel type"</label>
                                        <select
                                            class="w-full bg-ink-950 border border-ink-600 rounded-md px-3.5 py-2 text-xs text-slate-300 focus:outline-none focus:border-brand/60 transition-colors"
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
                                        <label class="block text-[11px] font-mono font-semibold text-ink-500 uppercase tracking-wide mb-1.5">"Target endpoint / webhook URL"</label>
                                        <input
                                            type="text"
                                            required
                                            placeholder="https://hooks.example.com/alerts"
                                            class="w-full bg-ink-950 border border-ink-600 rounded-md px-3.5 py-2 text-xs font-mono text-slate-200 placeholder-ink-600 focus:outline-none focus:border-brand/60 transition-colors"
                                            prop:value=webhook_url
                                            on:input=move |e| set_webhook_url.set(event_target_value(&e))
                                        />
                                    </div>
                                    <div class="flex items-center justify-end gap-3 pt-2">
                                        <button
                                            type="button"
                                            class="px-3.5 py-2 rounded-md bg-ink-800 hover:bg-ink-700 text-slate-300 text-xs font-medium transition-colors"
                                            on:click=move |_| set_show_modal.set(false)
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            type="submit"
                                            class="px-4 py-2 rounded-md bg-brand hover:bg-brand-light text-ink-950 text-xs font-bold transition-colors"
                                        >
                                            "Save channel"
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
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <For
                    each=move || channels.get()
                    key=|c| c.id
                    children=move |channel| {
                        let chan_id = channel.id;
                        let type_label = match channel.r#type {
                            ChannelType::Email => "Email (SMTP)",
                            ChannelType::Webhook => "Webhook",
                            ChannelType::Telegram => "Telegram bot",
                            ChannelType::Slack => "Slack",
                        };

                        view! {
                            <div class="bg-ink-900/60 border border-ink-600 border-l-2 border-l-brand rounded-lg p-6 flex flex-col justify-between space-y-4">
                                <div>
                                    <div class="flex items-center justify-between gap-3 mb-2">
                                        <span class="inline-flex items-center gap-1.5 text-[10px] font-mono font-bold text-brand uppercase bg-brand/10 px-2.5 py-1 rounded border border-brand/20">
                                            <IconBell class="w-3 h-3".to_string() />
                                            {type_label}
                                        </span>
                                        <span class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-mono font-semibold bg-brand/10 text-brand border border-brand/20">
                                            "ACTIVE"
                                        </span>
                                    </div>
                                    <h3 class="text-base font-bold text-white tracking-tight">
                                        {channel.name}
                                    </h3>
                                    <div class="text-[11px] font-mono text-ink-500 mt-1 truncate">
                                        {format!("{}", channel.config_json)}
                                    </div>
                                </div>

                                <div class="flex items-center justify-between pt-2 border-t border-ink-700">
                                    <button
                                        class="px-3 py-1.5 rounded-md bg-brand/10 hover:bg-brand/20 text-brand border border-brand/30 text-xs font-medium transition-colors"
                                        on:click=move |_| on_test_channel(chan_id)
                                    >
                                        "Send test alert"
                                    </button>
                                    <button
                                        class="text-xs text-sev-critical hover:brightness-125 transition-all"
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
