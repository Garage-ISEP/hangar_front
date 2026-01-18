use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::{pages::project_dashboard::render_log_line, services::project_service};

#[derive(Properties, PartialEq)]
pub struct ProjectLogsProps 
{
    pub project_id: i32,
}

#[function_component(ProjectLogs)]
pub fn project_logs(props: &ProjectLogsProps) -> Html 
{
    let (i18n, _) = use_translation();
    let logs = use_state(|| None::<String>);
    let logs_error = use_state(|| None::<String>);
    let are_logs_loading = use_state(|| false);

    let on_fetch_logs = 
    {
        let logs = logs.clone();
        let logs_error = logs_error.clone();
        let are_logs_loading = are_logs_loading.clone();
        let project_id = props.project_id;
        let i18n = i18n.clone();

        Callback::from(move |_| 
        {
            let logs = logs.clone();
            let logs_error = logs_error.clone();
            let are_logs_loading = are_logs_loading.clone();
            let i18n = i18n.clone();
            are_logs_loading.set(true);
            logs_error.set(None);

            wasm_bindgen_futures::spawn_local(async move 
            {
                match project_service::get_project_logs(project_id).await 
                {
                    Ok(log_data) => logs.set(Some(log_data)),
                    Err(e) => 
                    {
                        let error_message = i18n
                            .t("project_dashboard.logs_error")
                            .replace("{error}", &e);
                        logs_error.set(Some(error_message));
                        logs.set(None);
                    }
                }
                are_logs_loading.set(false);
            });
        })
    };

    html! 
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--spacing-md);">
                <h2>{ i18n.t("project_dashboard.card_title_logs") }</h2>
                <button class="button-primary" onclick={on_fetch_logs} disabled={*are_logs_loading}>
                    {
                        if *are_logs_loading 
                        {
                            i18n.t("project_dashboard.fetch_logs_loading")
                        } 
                        else 
                        {
                            i18n.t("project_dashboard.fetch_logs_button")
                        }
                    }
                </button>
            </div>

            <div class="logs-container">
                {
                    if let Some(err_msg) = &*logs_error 
                    {
                        html! { <p class="error">{ err_msg }</p> }
                    } 
                    else if let Some(log_data) = &*logs 
                    {
                        if log_data.is_empty() 
                        {
                            html! { <div class="placeholder">{ i18n.t("project_dashboard.logs_empty") }</div> }
                        } 
                        else 
                        {
                            log_data.lines().map(render_log_line).collect::<Html>()
                        }
                    } 
                    else 
                    {
                        html! { <div class="placeholder">{ i18n.t("project_dashboard.logs_placeholder") }</div> }
                    }
                }
            </div>
        </div>
    }
}