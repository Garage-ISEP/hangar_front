use yew::prelude::*;

use crate::contexts::sse_context::use_sse_system_events;
use crate::services::sse_service::{SystemEvent, SystemEventLevel};

#[function_component(SystemNotifications)]
pub fn system_notifications() -> Html
{
    let system_events = use_sse_system_events();
    let dismissed_events = use_state(Vec::<usize>::new);

    if system_events.is_empty()
    {
        return html! {};
    }

    let visible_events: Vec<(usize, &SystemEvent)> = system_events
        .iter()
        .enumerate()
        .filter(|(idx, _)| !dismissed_events.contains(idx))
        .collect();

    if visible_events.is_empty()
    {
        return html! {};
    }

    html!
    {
        <div style="position: fixed; top: 80px; right: 20px; z-index: 1000; max-width: 400px;">
            {
                for visible_events.iter().map(|(idx, event)|
                {
                    let idx = *idx;
                    let level_class = match event.level
                    {
                        SystemEventLevel::Info => "notification-info",
                        SystemEventLevel::Warning => "notification-warning",
                        SystemEventLevel::Error => "notification-error",
                    };

                    let icon = match event.level
                    {
                        SystemEventLevel::Info => "ℹ️",
                        SystemEventLevel::Warning => "⚠️",
                        SystemEventLevel::Error => "❌",
                    };

                    let on_dismiss =
                    {
                        let dismissed_events = dismissed_events.clone();
                        Callback::from(move |_|
                        {
                            let mut dismissed = (*dismissed_events).clone();
                            dismissed.push(idx);
                            dismissed_events.set(dismissed);
                        })
                    };

                    html!
                    {
                        <div
                            class={classes!("system-notification", level_class)}
                            style="background: var(--color-background); border: 1px solid var(--color-border); border-radius: 8px; padding: 12px 16px; margin-bottom: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); display: flex; align-items: start; gap: 12px; animation: slideIn 0.3s ease-out;"
                        >
                            <span style="font-size: 20px; flex-shrink: 0;">{ icon }</span>
                            <div style="flex: 1; min-width: 0;">
                                <p style="margin: 0; font-weight: 500; word-break: break-word;">
                                    { &event.message }
                                </p>
                                {
                                    if let Some(context) = &event.context
                                    {
                                        html!
                                        {
                                            <p style="margin: 4px 0 0 0; font-size: 0.9em; color: var(--color-text-secondary); word-break: break-word;">
                                                { context.to_string() }
                                            </p>
                                        }
                                    }
                                    else
                                    {
                                        html! {}
                                    }
                                }
                            </div>
                            <button
                                onclick={on_dismiss}
                                style="background: transparent; border: none; cursor: pointer; font-size: 20px; line-height: 1; padding: 0; color: var(--color-text-secondary); flex-shrink: 0;"
                                title="Fermer"
                            >
                                { "×" }
                            </button>
                        </div>
                    }
                })
            }
        </div>
    }
}