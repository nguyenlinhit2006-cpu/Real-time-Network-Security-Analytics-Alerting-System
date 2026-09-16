use common::models::{Alert, AlertSeverity};
use leptos::prelude::*;

fn play_critical_sound() {
    let _ = js_sys::eval(r#"
        try {
            const ctx = new (window.AudioContext || window.webkitAudioContext)();
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            osc.type = 'sawtooth';
            osc.frequency.setValueAtTime(880, ctx.currentTime);
            osc.frequency.exponentialRampToValueAtTime(440, ctx.currentTime + 0.35);
            gain.gain.setValueAtTime(0.25, ctx.currentTime);
            gain.gain.linearRampToValueAtTime(0.01, ctx.currentTime + 0.35);
            osc.connect(gain);
            gain.connect(ctx.destination);
            osc.start();
            osc.stop(ctx.currentTime + 0.35);
        } catch(e) {}
    "#);
}

#[component]
pub fn ToastNotification(
    latest_alert: ReadSignal<Option<Alert>>,
    set_latest_alert: WriteSignal<Option<Alert>>,
) -> impl IntoView {
    Effect::new(move |_| {
        if let Some(ref alert) = latest_alert.get() {
            if alert.severity == AlertSeverity::Critical {
                play_critical_sound();
            }
        }
    });

    view! {
        {move || {
            if let Some(alert) = latest_alert.get() {
                let is_critical = alert.severity == AlertSeverity::Critical;
                let bg = if is_critical { "bg-red-950/95 border-red-500/50 text-red-100" } else { "bg-slate-900/95 border-indigo-500/50 text-slate-100" };

                view! {
                    <div class=format!("fixed bottom-6 right-6 max-w-sm w-full border rounded-2xl p-4 shadow-2xl backdrop-blur-md z-50 animate-bounce transition-all {}", bg)>
                        <div class="flex items-start justify-between gap-3">
                            <div class="flex items-center gap-2">
                                <span class="text-xl">{if is_critical { "🚨" } else { "🔔" }}</span>
                                <div>
                                    <div class="text-xs font-bold uppercase tracking-wider text-red-400">
                                        {format!("NEW {:?} ALERT", alert.severity)}
                                    </div>
                                    <div class="text-xs font-semibold mt-0.5 text-white">
                                        {alert.title}
                                    </div>
                                    <div class="text-[11px] text-slate-300 mt-1 truncate">
                                        {alert.description}
                                    </div>
                                </div>
                            </div>
                            <button
                                class="text-slate-400 hover:text-white text-xs font-bold p-1"
                                on:click=move |_| set_latest_alert.set(None)
                            >
                                "✕"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }
}
