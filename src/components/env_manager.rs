// src/pages/project_dashboard/components/env_manager.rs

use std::collections::HashMap;

use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::models::project::UpdateEnvPayload;
use crate::services::project_service::{self, ApiError};

use crate::pages::project_dashboard::translate_error;

#[derive(Properties, PartialEq)]
pub struct EnvManagerProps
{
    pub project_id: i32,
    pub current_env_vars: Option<HashMap<String, String>>,
    pub on_update: Callback<()>,
}

#[function_component(EnvManager)]
pub fn env_manager(props: &EnvManagerProps) -> Html
{
    let (i18n, _) = use_translation();

    let env_vars_str = use_state(String::new);
    let is_initialized = use_state(|| false);

    let is_loading = use_state(|| false);
    let error = use_state(|| None::<ApiError>);
    let success = use_state(|| false);

    {
        let env_vars_str = env_vars_str.clone();
        let is_initialized = is_initialized.clone();
        let initial_vars = props.current_env_vars.clone();
        
        use_effect_with((), move |_|
        {
            if !*is_initialized
            {
                if let Some(vars) = initial_vars
                {
                    let initial_str = vars
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join("\n");
                    env_vars_str.set(initial_str);
                }
                is_initialized.set(true);
            }
            || ()
        });
    }

    let on_change =
    {
        let env_vars_str = env_vars_str.clone();
        let success = success.clone();
        Callback::from(move |e: Event|
        {
            let value = e
                .target_unchecked_into::<web_sys::HtmlTextAreaElement>()
                .value();
            env_vars_str.set(value);
            success.set(false);
        })
    };

    let on_submit =
    {
        let env_vars_str = env_vars_str.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let success = success.clone();
        let on_update = props.on_update.clone();
        let project_id = props.project_id;

        Callback::from(move |e: SubmitEvent|
        {
            e.prevent_default();
            is_loading.set(true);
            error.set(None);
            success.set(false);

            let env_vars: HashMap<String, String> = (*env_vars_str)
                .lines()
                .filter_map(|line| line.trim().split_once('='))
                .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                .filter(|(k, _)| !k.is_empty())
                .collect();

            let payload = UpdateEnvPayload { env_vars };
            let is_loading = is_loading.clone();
            let error = error.clone();
            let success = success.clone();
            let on_update = on_update.clone();

            wasm_bindgen_futures::spawn_local(async move
            {
                match project_service::update_env_vars(project_id, &payload).await
                {
                    Ok(_) =>
                    {
                        success.set(true);
                        on_update.emit(());
                    }
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
        })
    };

    if !*is_initialized
    {
        return html!
        {
            <div class="card" style="margin-top: var(--spacing-lg);">
                <h2>{ i18n.t("project_dashboard.card_title_env_vars") }</h2>
                <p>{ i18n.t("common.loading") }</p>
            </div>
        };
    }

    html!
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("project_dashboard.card_title_env_vars") }</h2>
            <p style="color: var(--color-text-secondary); margin-bottom: var(--spacing-md);">
                { i18n.t("project_dashboard.env_vars_description") }
            </p>
            <form onsubmit={on_submit}>
                <div class="form-group">
                    <textarea
                        class="text-input"
                        value={(*env_vars_str).clone()}
                        onchange={on_change}
                        rows="8"
                        placeholder="KEY=VALUE"
                    />
                </div>

                if *success
                {
                    <p class="success-banner" style="margin-bottom: var(--spacing-md); background-color: rgba(126, 211, 33, 0.2); border-color: #7ED321;">
                        { i18n.t("project_dashboard.env_vars_updated_success") }
                    </p>
                }
                if let Some(err) = &*error
                {
                    <p class="error">{ translate_error(err, &i18n) }</p>
                }

                <button type="submit" class="button-primary" disabled={*is_loading}>
                    {
                        if *is_loading
                        {
                            i18n.t("project_dashboard.save_and_restart_button_loading")
                        }
                        else
                        {
                            i18n.t("project_dashboard.save_and_restart_button")
                        }
                    }
                </button>
            </form>
        </div>
    }
}