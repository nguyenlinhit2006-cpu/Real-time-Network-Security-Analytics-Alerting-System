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

    let arcs = Memo::new(move |_| {
        let (c, h, m, l) = counts.get();
        let total = (c + h + m + l).max(1) as f64;
        let seg = |n: i32| (n as f64 / total) * 100.0;
        (seg(c), seg(h), seg(m), seg(l))
    });

    view! {
        <div class="bg-ink-900/60 border border-ink-600 rounded-lg p-6 h-full">
            <h3 class="text-sm font-bold text-white tracking-tight mb-0.5">
                "Incidents by severity"
            </h3>
            <p class="text-xs text-ink-500 mb-5">"Classification of detected threat events"</p>

            <div class="flex items-center justify-between gap-6">
                // Donut SVG representation
                <div class="relative w-28 h-28 flex items-center justify-center shrink-0">
                    <svg viewBox="0 0 36 36" class="w-full h-full -rotate-90">
                        <circle cx="18" cy="18" r="14" fill="none" stroke="#141B27" stroke-width="3.5"/>
                        <circle
                            cx="18" cy="18" r="14" fill="none" stroke="#FB4B5A" stroke-width="3.5" stroke-linecap="round"
                            stroke-dasharray=move || format!("{} {}", arcs.get().0, 100.0 - arcs.get().0) stroke-dashoffset="0"
                        />
                        <circle
                            cx="18" cy="18" r="14" fill="none" stroke="#FF9D4D" stroke-width="3.5" stroke-linecap="round"
                            stroke-dasharray=move || format!("{} {}", arcs.get().1, 100.0 - arcs.get().1) stroke-dashoffset=move || -arcs.get().0
                        />
                        <circle
                            cx="18" cy="18" r="14" fill="none" stroke="#FFD166" stroke-width="3.5" stroke-linecap="round"
                            stroke-dasharray=move || format!("{} {}", arcs.get().2, 100.0 - arcs.get().2) stroke-dashoffset=move || -(arcs.get().0 + arcs.get().1)
                        />
                        <circle
                            cx="18" cy="18" r="14" fill="none" stroke="#7A879C" stroke-width="3.5" stroke-linecap="round"
                            stroke-dasharray=move || format!("{} {}", arcs.get().3, 100.0 - arcs.get().3) stroke-dashoffset=move || -(arcs.get().0 + arcs.get().1 + arcs.get().2)
                        />
                    </svg>
                    <div class="absolute text-center">
                        <span class="block text-base font-mono font-bold text-white leading-none">
                            {move || alerts.get().len()}
                        </span>
                        <div class="text-[9px] text-ink-500 uppercase tracking-wide mt-1">"total"</div>
                    </div>
                </div>

                // Legend
                <div class="space-y-2 text-xs flex-1">
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full bg-sev-critical"></span>
                            "Critical"
                        </span>
                        <span class="font-mono font-bold text-sev-critical">{move || counts.get().0}</span>
                    </div>
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full bg-sev-high"></span>
                            "High"
                        </span>
                        <span class="font-mono font-bold text-sev-high">{move || counts.get().1}</span>
                    </div>
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full bg-sev-medium"></span>
                            "Medium"
                        </span>
                        <span class="font-mono font-bold text-sev-medium">{move || counts.get().2}</span>
                    </div>
                    <div class="flex items-center justify-between text-slate-300">
                        <span class="flex items-center gap-2">
                            <span class="w-2 h-2 rounded-full bg-sev-low"></span>
                            "Low"
                        </span>
                        <span class="font-mono font-bold text-sev-low">{move || counts.get().3}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
