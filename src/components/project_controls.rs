use std::future::Future;
use std::pin::Pin;

use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::services::project_service;

const RELOAD_DELAY_MS: u32 = 1500;

type LocalBoxFutureAction<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

#[derive(Properties, PartialEq)]
pub struct ProjectControlsProps 
{
    pub project_id: i32,
    pub on_update: Callback<()>,
}

#[function_component(ProjectControls)]
pub fn project_controls(props: &ProjectControlsProps) -> Html 
{
    let (i18n, _) = use_translation();
    let is_controlling = use_state(|| false);
    let success_message = use_state(|| None::<String>);

    let create_control_callback =
        |action: fn(i32) -> LocalBoxFutureAction<Result<(), String>>, message: String| 
        {
            let is_controlling = is_controlling.clone();
            let on_update = props.on_update.clone();
            let project_id = props.project_id;
            let success_message = success_message.clone();

            Callback::from(move |_| 
            {
                let is_controlling = is_controlling.clone();
                let on_update = on_update.clone();
                let success_message = success_message.clone();
                let message = message.clone();
                is_controlling.set(true);
                success_message.set(None);

                wasm_bindgen_futures::spawn_local(async move 
                {
                    if action(project_id).await.is_ok() 
                    {
                        success_message.set(Some(message));
                        gloo_timers::callback::Timeout::new(RELOAD_DELAY_MS, move || {
                            on_update.emit(());
                        })
                        .forget();
                    } 
                    else 
                    {
                        gloo_console::error!("Control action failed");
                    }
                    is_controlling.set(false);
                });
            })
        };

    let on_start = create_control_callback(
        |id| Box::pin(project_service::start_project(id)),
        i18n.t("project_dashboard.start_success"),
    );
    let on_stop = create_control_callback(
        |id| Box::pin(project_service::stop_project(id)),
        i18n.t("project_dashboard.stop_success"),
    );
    let on_restart = create_control_callback(
        |id| Box::pin(project_service::restart_project(id)),
        i18n.t("project_dashboard.restart_success"),
    );

    html! 
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("project_dashboard.card_title_controls") }</h2>

            if let Some(msg) = &*success_message 
            {
                <div class="success-banner" style="margin-bottom: var(--spacing-md);">
                    <p>{ msg }</p>
                </div>
            }

            <div style="display: flex; gap: var(--spacing-md);">
                <button class="button-primary" onclick={on_start} disabled={*is_controlling}>
                    { i18n.t("project_dashboard.start_button") }
                </button>
                <button class="button-danger" onclick={on_stop} disabled={*is_controlling}>
                    { i18n.t("project_dashboard.stop_button") }
                </button>
                <button class="button-primary" onclick={on_restart} disabled={*is_controlling}>
                    { i18n.t("project_dashboard.restart_button") }
                </button>
            </div>
        </div>
    }
}