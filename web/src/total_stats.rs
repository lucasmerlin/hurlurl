use yew::{function_component, html};
use shared::TotalStats as Stats;
use crate::use_fetch::use_fetch;

#[function_component(TotalStats)]
pub fn total_stats() -> Html {

    let data: Option<Stats> = use_fetch("/api/stats");

    let value = |val| {
        if let Some(val) = val {
            format!("{}", val)
        } else {
            "".to_string()
        }
    };

    html! {
        <div class="stats shadow stats-vertical sm:stats-horizontal">
            <div class="stat">
                <div class="stat-figure text-primary">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244" />
                    </svg>
                </div>
                <div class="stat-title">{"Shortened Links"}</div>
                <div class="stat-value text-primary">{value(data.as_ref().map(|d| d.links))}</div>
            </div>

            <div class="stat">
                <div class="stat-figure text-primary">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
                        <path fill-rule="evenodd" d="M9.315 7.584C12.195 3.883 16.695 1.5 21.75 1.5a.75.75 0 01.75.75c0 5.056-2.383 9.555-6.084 12.436A6.75 6.75 0 019.75 22.5a.75.75 0 01-.75-.75v-4.131A15.838 15.838 0 016.382 15H2.25a.75.75 0 01-.75-.75 6.75 6.75 0 017.815-6.666zM15 6.75a2.25 2.25 0 100 4.5 2.25 2.25 0 000-4.5z" clip-rule="evenodd" />
                        <path d="M5.26 17.242a.75.75 0 10-.897-1.203 5.243 5.243 0 00-2.05 5.022.75.75 0 00.625.627 5.243 5.243 0 005.022-2.051.75.75 0 10-1.202-.897 3.744 3.744 0 01-3.008 1.51c0-1.23.592-2.323 1.51-3.008z" />
                    </svg>
                </div>
                <div class="stat-title">{"Clicks total"}</div>
                <div class="stat-value text-primary">{value(data.as_ref().map(|d| d.redirects))}</div>
            </div>
        </div>
    }
}
