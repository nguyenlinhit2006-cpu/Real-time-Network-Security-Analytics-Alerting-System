use leptos::prelude::*;

#[component]
pub fn TrafficChart(throughput: ReadSignal<u64>) -> impl IntoView {
    let (history, set_history) = signal(vec![120, 180, 140, 220, 310, 280, 350, 420, 390, 500]);

    Effect::new(move |_| {
        let current = throughput.get();
        if current > 0 {
            set_history.update(|h| {
                h.push((current % 600 + 100) as usize);
                if h.len() > 15 {
                    h.remove(0);
                }
            });
        }
    });

    let path_data = Memo::new(move |_| {
        let data = history.get();
        if data.is_empty() {
            return String::new();
        }

        let max_val = *data.iter().max().unwrap_or(&1) as f64;
        let width = 500.0;
        let height = 140.0;
        let step = width / (data.len() - 1).max(1) as f64;

        let mut d = String::new();
        for (i, &val) in data.iter().enumerate() {
            let x = i as f64 * step;
            let y = height - (val as f64 / max_val.max(1.0) * (height - 20.0)) - 10.0;
            if i == 0 {
                d.push_str(&format!("M {:.1} {:.1}", x, y));
            } else {
                d.push_str(&format!(" L {:.1} {:.1}", x, y));
            }
        }
        d
    });

    view! {
        <div class="bg-slate-900/90 border border-slate-800/80 rounded-2xl p-6 shadow-2xl backdrop-blur-md">
            <div class="flex items-center justify-between mb-4">
                <div>
                    <h3 class="text-sm font-bold text-white tracking-tight flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
                        "Live Network Throughput"
                    </h3>
                    <p class="text-xs text-slate-400">"Real-time packet stream from capture-engine"</p>
                </div>
                <div class="text-right">
                    <span class="text-xs font-mono font-bold text-emerald-400">
                        {move || format!("{} B/s", throughput.get())}
                    </span>
                </div>
            </div>

            // SVG Line Chart
            <div class="w-full h-40 flex items-center justify-center">
                <svg viewBox="0 0 500 140" class="w-full h-full overflow-visible">
                    <defs>
                        <linearGradient id="trafficGradient" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color="#10b981" stop-opacity="0.3"/>
                            <stop offset="100%" stop-color="#10b981" stop-opacity="0.0"/>
                        </linearGradient>
                    </defs>

                    // Grid lines
                    <line x1="0" y1="35" x2="500" y2="35" stroke="#1e293b" stroke-dasharray="4"/>
                    <line x1="0" y1="70" x2="500" y2="70" stroke="#1e293b" stroke-dasharray="4"/>
                    <line x1="0" y1="105" x2="500" y2="105" stroke="#1e293b" stroke-dasharray="4"/>

                    // Dynamic Curve
                    <path
                        d=move || path_data.get()
                        fill="none"
                        stroke="#10b981"
                        stroke-width="3"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    />
                </svg>
            </div>
        </div>
    }
}
