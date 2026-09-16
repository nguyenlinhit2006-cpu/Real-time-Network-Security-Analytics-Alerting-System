use common::models::{Alert, AlertSeverity};
use leptos::prelude::*;

#[component]
pub fn SeverityDonut(alerts: ReadSignal<Vec<Alert>>) -> impl IntoView {
    let counts = Memo::new(move |_| {
        let mut critical = 0;
        let mut high = 0;
        let mut medium = 0;
        let mut low = 0;

        for a in alerts.get() {
            match a.severity {
                AlertSeverity::Critical => critical += 1,
                AlertSeverity::High => high += 1,
                AlertSeverity::Medium => medium += 1,
                AlertSeverity::Low => low += 1,
            }
        }
        (critical, high, medium, low)
    });

    view! {
        <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md">
            <h3 class="text-sm font-bold text-white tracking-tight mb-1">
                "Incidents by Severity"
            </h3>
            <p class="text-xs text-slate-400 mb-4">"Classification of detected threat events"</p>

            <div class="flex items-center justify-between gap-6">
                // Donut SVG representation
                <div class="relative w-28 h-28 flex items-center justify-center">
                    <svg viewBox="0 0 36 36" class="w-full h-full -rotate-90">
                        <circle cx="18" cy="18" r="14" fill="none" stroke="#1e293b" stroke-width="4"/>
                        <circle
                            cx="18" cy="18" r="14" fill="none" stroke="#ef4444" stroke-width="4"
                            stroke-dasharray="25 100" stroke-dashoffset="0"
                        />
                        <circle
                            cx="18" cy="18" r="14" fill="none" stroke="#f59e0b" stroke-width="4"
                            stroke-dasharray="35 100" stroke-dashoffset="-25"
                        />
                        <circle
                            cx="18" cy="18" r="14" fill="none" stroke="#3b82f6" stroke-width="4"
                            stroke-dasharray="25 100" stroke-dashoffset="-60"
                        />
                    </svg>
                    <div class="absolute text-center">
                        <span class="text-xs font-mono font-bold text-white">
                            {move || alerts.get().len()}
                        </span>
                        <div class="text-[9px] text-slate-500 uppercase">"Total"</div>
                    </div>
                </div>

                // Legend
                <div class="space-y-1.5 text-xs flex-1">
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-1.5">
                            <span class="w-2 h-2 rounded-full bg-red-500"></span>
                            "Critical"
                        </span>
                        <span class="font-mono font-bold text-red-400">{move || counts.get().0}</span>
                    </div>
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-1.5">
                            <span class="w-2 h-2 rounded-full bg-amber-500"></span>
                            "High"
                        </span>
                        <span class="font-mono font-bold text-amber-400">{move || counts.get().1}</span>
                    </div>
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-1.5">
                            <span class="w-2 h-2 rounded-full bg-blue-500"></span>
                            "Medium"
                        </span>
                        <span class="font-mono font-bold text-blue-400">{move || counts.get().2}</span>
                    </div>
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-1.5">
                            <span class="w-2 h-2 rounded-full bg-slate-500"></span>
                            "Low"
                        </span>
                        <span class="font-mono font-bold text-slate-400">{move || counts.get().3}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
