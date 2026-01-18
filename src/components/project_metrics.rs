use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::components::gauge::Gauge;
use crate::contexts::sse_context::use_sse_metrics;

#[function_component(ProjectMetrics)]
pub fn project_metrics() -> Html
{
    let (i18n, _) = use_translation();
    let current_metrics = use_sse_metrics();

    html!
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("project_dashboard.card_title_metrics") }</h2>
            <div class="metrics-grid">
                {
                    if let Some(m) = &current_metrics
                    {
                        html!
                        {
                            <>
                                <Gauge
                                    label="CPU"
                                    value={m.cpu_usage}
                                    max_value={100.0}
                                    unit="%"
                                />
                                <Gauge
                                    label="RAM"
                                    value={m.memory_usage}
                                    max_value={m.memory_limit}
                                    unit="MiB"
                                />
                            </>
                        }
                    }
                    else
                    {
                        html! { <p>{ i18n.t("common.loading") }</p> }
                    }
                }
            </div>
        </div>
    }
}