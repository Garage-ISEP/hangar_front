use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use gloo_timers::callback::Interval;
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::gauge::Gauge,
    contexts::user_context::use_user,
    models::{
        database::DatabaseDetails,
        project::{ProjectDetails, ProjectMetrics, ProjectSourceType, UpdateEnvPayload},
    },
    router::AppRoute,
    services::{
        database_service,
        project_service::{self, ApiError},
    },
};

// ============================================================================
// CONSTANTS
// ============================================================================

const STATUS_POLL_INTERVAL_MS: u32 = 5000;
const METRICS_POLL_INTERVAL_MS: u32 = 3000;
const RELOAD_DELAY_MS: u32 = 1500;

// ============================================================================
// TYPE ALIASES
// ============================================================================

type LocalBoxFutureAction<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// ============================================================================
// PROPS DEFINITIONS
// ============================================================================

#[derive(Properties, PartialEq)]
pub struct ProjectDashboardProps {
    pub project_id: i32,
}

#[derive(Properties, PartialEq)]
struct ParticipantManagerProps {
    project_id: i32,
    participants: Vec<String>,
    on_update: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct ImageUpdateFormProps {
    project_id: i32,
    project_name: String,
    source_type: ProjectSourceType,
    on_update: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct EnvManagerProps {
    project_id: i32,
    current_env_vars: Option<HashMap<String, String>>,
    on_update: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct ProjectStatusProps {
    project_id: i32,
}

#[derive(Properties, PartialEq)]
struct ProjectMetricsDisplayProps {
    project_id: i32,
}

#[derive(Properties, PartialEq)]
struct DatabaseManagerProps {
    project_details: ProjectDetails,
    my_database: Option<DatabaseDetails>,
    has_control_access: bool,
    on_update: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct ProjectControlsProps {
    project_id: i32,
    on_update: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct ProjectLogsProps {
    project_id: i32,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn render_log_line(line: &str) -> Html {
    let (timestamp, message) = parse_log_line(line);
    let log_level_class = determine_log_level(message);

    html! {
        <div class="log-line">
            <span class="log-timestamp">{ format_timestamp(timestamp) }</span>
            <span class={classes!("log-message", log_level_class)}>{ message }</span>
        </div>
    }
}

fn parse_log_line(line: &str) -> (&str, &str) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() == 2 && parts[0].ends_with('Z') {
        (parts[0], parts[1].trim())
    } else {
        ("", line)
    }
}

fn format_timestamp(timestamp: &str) -> &str {
    timestamp.split('.').next().unwrap_or(timestamp)
}

fn determine_log_level(message: &str) -> &'static str {
    let message_upper = message.to_uppercase();
    if message_upper.contains("ERROR") || message_upper.contains("FAILED") {
        "log-error"
    } else if message_upper.contains("WARN") || message_upper.contains("WARNING") {
        "log-warn"
    } else {
        "log-info"
    }
}

fn translate_status(status_str: &str, i18n: &i18nrs::I18n) -> String {
    let key = format!("common.status_{}", status_str);
    let translation = i18n.t(&key);

    if is_translation_missing(&translation) {
        i18n.t("common.status_unknown")
    } else {
        translation
    }
}

fn translate_error(error: &ApiError, i18n: &i18nrs::I18n) -> String {
    let error_key = format!("errors.{}", error.error_code);
    let translation = i18n.t(&error_key);

    if is_translation_missing(&translation) {
        i18n.t("errors.DEFAULT")
    } else {
        translation
    }
}

fn is_translation_missing(translation: &str) -> bool {
    translation.starts_with("Key '") && translation.contains(" not found for language ")
}

// ============================================================================
// STATUS & METRICS COMPONENTS
// ============================================================================

#[function_component(ProjectStatus)]
fn project_status(props: &ProjectStatusProps) -> Html {
    let (i18n, _) = use_translation();
    let status = use_state(|| None::<String>);

    {
        let status = status.clone();
        let project_id = props.project_id;

        use_effect_with(project_id, move |_| {
            let fetch_status = move || {
                let status = status.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(s) = project_service::get_project_status(project_id).await {
                        status.set(s);
                    }
                });
            };

            fetch_status();
            let interval = Interval::new(STATUS_POLL_INTERVAL_MS, fetch_status);

            move || drop(interval)
        });
    }

    let status_class = status
        .as_ref()
        .map(|s| format!("status-{}", s))
        .unwrap_or_else(|| "status-unknown".to_string());

    let status_text = status
        .as_ref()
        .map(|s| translate_status(s, &i18n))
        .unwrap_or_else(|| i18n.t("common.loading"));

    html! {
        <span class={classes!("status-badge", status_class)}>
            { status_text }
        </span>
    }
}

#[function_component(ProjectMetricsDisplay)]
fn project_metrics_display(props: &ProjectMetricsDisplayProps) -> Html {
    let (i18n, _) = use_translation();
    let metrics = use_state(|| None::<ProjectMetrics>);

    {
        let metrics = metrics.clone();
        let project_id = props.project_id;

        use_effect_with(project_id, move |_| {
            let fetch_metrics = move || {
                let metrics = metrics.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match project_service::get_project_metrics(project_id).await {
                        Ok(m) => metrics.set(Some(m)),
                        Err(_) => metrics.set(None),
                    }
                });
            };

            fetch_metrics();
            let interval = Interval::new(METRICS_POLL_INTERVAL_MS, fetch_metrics);

            move || drop(interval)
        });
    }

    html! {
        <div class="metrics-grid">
            {
                if let Some(m) = &*metrics {
                    html! {
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
                } else {
                    html! { <p>{ i18n.t("common.loading") }</p> }
                }
            }
        </div>
    }
}

// ============================================================================
// PROJECT CONTROLS COMPONENT
// ============================================================================

#[function_component(ProjectControls)]
fn project_controls(props: &ProjectControlsProps) -> Html {
    let (i18n, _) = use_translation();
    let is_controlling = use_state(|| false);
    let success_message = use_state(|| None::<String>);

    let create_control_callback = |action: fn(i32) -> LocalBoxFutureAction<Result<(), String>>, message: String| {
        let is_controlling = is_controlling.clone();
        let on_update = props.on_update.clone();
        let project_id = props.project_id;
        let success_message = success_message.clone();

        Callback::from(move |_| {
            let is_controlling = is_controlling.clone();
            let on_update = on_update.clone();
            let success_message = success_message.clone();
            let message = message.clone();
            is_controlling.set(true);
            success_message.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                if action(project_id).await.is_ok() {
                    success_message.set(Some(message));
                    gloo_timers::callback::Timeout::new(RELOAD_DELAY_MS, move || {
                        on_update.emit(());
                    })
                    .forget();
                } else {
                    gloo_console::error!("Control action failed");
                }
                is_controlling.set(false);
            });
        })
    };

    let on_start = create_control_callback(
        |id| Box::pin(project_service::start_project(id)),
        i18n.t("project_dashboard.start_success")
    );
    let on_stop = create_control_callback(
        |id| Box::pin(project_service::stop_project(id)),
        i18n.t("project_dashboard.stop_success")
    );
    let on_restart = create_control_callback(
        |id| Box::pin(project_service::restart_project(id)),
        i18n.t("project_dashboard.restart_success")
    );

    html! {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("project_dashboard.card_title_controls") }</h2>

            if let Some(msg) = &*success_message {
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

// ============================================================================
// PROJECT LOGS COMPONENT
// ============================================================================

#[function_component(ProjectLogs)]
fn project_logs(props: &ProjectLogsProps) -> Html {
    let (i18n, _) = use_translation();
    let logs = use_state(|| None::<String>);
    let logs_error = use_state(|| None::<String>);
    let are_logs_loading = use_state(|| false);

    let on_fetch_logs = {
        let logs = logs.clone();
        let logs_error = logs_error.clone();
        let are_logs_loading = are_logs_loading.clone();
        let project_id = props.project_id;
        let i18n = i18n.clone();

        Callback::from(move |_| {
            let logs = logs.clone();
            let logs_error = logs_error.clone();
            let are_logs_loading = are_logs_loading.clone();
            let i18n = i18n.clone();
            are_logs_loading.set(true);
            logs_error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match project_service::get_project_logs(project_id).await {
                    Ok(log_data) => logs.set(Some(log_data)),
                    Err(e) => {
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

    html! {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--spacing-md);">
                <h2>{ i18n.t("project_dashboard.card_title_logs") }</h2>
                <button class="button-primary" onclick={on_fetch_logs} disabled={*are_logs_loading}>
                    { 
                        if *are_logs_loading { 
                            i18n.t("project_dashboard.fetch_logs_loading") 
                        } else { 
                            i18n.t("project_dashboard.fetch_logs_button") 
                        } 
                    }
                </button>
            </div>

            <div class="logs-container">
                {
                    if let Some(err_msg) = &*logs_error {
                        html! { <p class="error">{ err_msg }</p> }
                    } else if let Some(log_data) = &*logs {
                        if log_data.is_empty() {
                            html! { <div class="placeholder">{ i18n.t("project_dashboard.logs_empty") }</div> }
                        } else {
                            log_data.lines().map(render_log_line).collect::<Html>()
                        }
                    } else {
                        html! { <div class="placeholder">{ i18n.t("project_dashboard.logs_placeholder") }</div> }
                    }
                }
            </div>
        </div>
    }
}

// ============================================================================
// PARTICIPANT MANAGER COMPONENT
// ============================================================================

#[function_component(ParticipantManager)]
fn participant_manager(props: &ParticipantManagerProps) -> Html {
    let (i18n, _) = use_translation();
    let new_participant = use_state(String::new);
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<ApiError>);

    let on_input_change = {
        let new_participant = new_participant.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            new_participant.set(value);
        })
    };

    let on_add = {
        let is_loading = is_loading.clone();
        let error = error.clone();
        let new_participant = new_participant.clone();
        let project_id = props.project_id;
        let on_update = props.on_update.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            is_loading.set(true);
            error.set(None);

            let participant_id = (*new_participant).clone();
            let new_participant = new_participant.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let on_update = on_update.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match project_service::add_participant(project_id, &participant_id).await {
                    Ok(_) => {
                        new_participant.set(String::new());
                        on_update.emit(());
                    }
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
        })
    };

    let render_participant = |p: &String| {
        let participant_id = p.clone();
        let on_remove = {
            let project_id = props.project_id;
            let on_update = props.on_update.clone();
            let i18n = i18n.clone();
            let participant_id = participant_id.clone();

            Callback::from(move |_| {
                let confirm_msg = i18n
                    .t("project_dashboard.confirm_remove_participant")
                    .replace("{name}", &participant_id);

                if web_sys::window()
                    .unwrap()
                    .confirm_with_message(&confirm_msg)
                    .unwrap()
                {
                    let on_update = on_update.clone();
                    let participant_id = participant_id.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        if project_service::remove_participant(project_id, &participant_id)
                            .await
                            .is_ok()
                        {
                            on_update.emit(());
                        } else {
                            gloo_console::error!("Failed to remove participant");
                        }
                    });
                }
            })
        };

        html! {
            <li style="display: flex; justify-content: space-between; align-items: center; padding: var(--spacing-sm) 0; border-bottom: 1px solid var(--color-border);">
                <span>{ p }</span>
                <button class="button-danger" onclick={on_remove}>
                    { i18n.t("project_dashboard.remove_participant_button") }
                </button>
            </li>
        }
    };

    html! {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("project_dashboard.manage_participants_title") }</h2>

            if props.participants.is_empty() {
                <p style="color: var(--color-text-secondary);">
                    { i18n.t("project_dashboard.no_participants") }
                </p>
            } else {
                <ul style="list-style: none; margin-bottom: var(--spacing-lg);">
                    { for props.participants.iter().map(render_participant) }
                </ul>
            }

            <form onsubmit={on_add}>
                <div class="form-group" style="margin-bottom: var(--spacing-sm);">
                    <label for="participant_id">
                        { i18n.t("project_dashboard.add_participant_label") }
                    </label>
                    <input
                        type="text"
                        id="participant_id"
                        class="text-input"
                        placeholder={i18n.t("project_dashboard.add_participant_placeholder")}
                        value={(*new_participant).clone()}
                        onchange={on_input_change}
                        required=true
                    />
                </div>

                if let Some(err) = &*error {
                    <p class="error">{ translate_error(err, &i18n) }</p>
                }

                <button type="submit" class="button-primary" disabled={*is_loading}>
                    {
                        if *is_loading {
                            i18n.t("project_dashboard.add_participant_button_loading")
                        } else {
                            i18n.t("project_dashboard.add_participant_button")
                        }
                    }
                </button>
            </form>
        </div>
    }
}

// ============================================================================
// IMAGE UPDATE FORM COMPONENT (Unified for both GitHub rebuild and Direct image update)
// ============================================================================

#[function_component(ImageUpdateForm)]
fn image_update_form(props: &ImageUpdateFormProps) -> Html {
    let (i18n, _) = use_translation();
    let new_image_url = use_state(String::new);
    let is_updating = use_state(|| false);
    let update_error = use_state(|| None::<ApiError>);

    let is_github = props.source_type == ProjectSourceType::Github;

    let on_input_change = {
        let new_image_url = new_image_url.clone();
        Callback::from(move |e: Event| {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            new_image_url.set(value);
        })
    };

    let on_submit = {
        let project_id = props.project_id;
        let project_name = props.project_name.clone();
        let new_image_url = new_image_url.clone();
        let is_updating = is_updating.clone();
        let update_error = update_error.clone();
        let i18n = i18n.clone();
        let is_github = is_github;
        let on_update = props.on_update.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let confirm_key = if is_github {
                "project_dashboard.confirm_rebuild"
            } else {
                "project_dashboard.confirm_update_image"
            };

            let confirm_message = i18n.t(confirm_key).replace("{name}", &project_name);

            if web_sys::window()
                .unwrap()
                .confirm_with_message(&confirm_message)
                .unwrap()
            {
                let image_url = (*new_image_url).clone();
                let new_image_url = new_image_url.clone();
                let is_updating = is_updating.clone();
                let update_error = update_error.clone();
                let on_update = on_update.clone();

                is_updating.set(true);
                update_error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let result = if is_github {
                        project_service::rebuild_project(project_id).await
                    } else {
                        project_service::update_project_image(project_id, &image_url).await
                    };

                    match result {
                        Ok(_) => {
                            new_image_url.set(String::new());
                            is_updating.set(false);
                            on_update.emit(());
                        }
                        Err(api_error) => {
                            update_error.set(Some(api_error));
                            is_updating.set(false);
                        }
                    }
                });
            }
        })
    };

    let (title_key, description_key, button_key, button_loading_key) = if is_github {
        (
            "project_dashboard.card_title_rebuild",
            "project_dashboard.rebuild_description",
            "project_dashboard.rebuild_button",
            "project_dashboard.rebuild_button_loading",
        )
    } else {
        (
            "project_dashboard.card_title_update_image",
            "project_dashboard.update_image_description",
            "project_dashboard.update_image_button",
            "project_dashboard.update_image_button_loading",
        )
    };

    html! {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t(title_key) }</h2>
            <p style="color: var(--color-text-secondary); margin-bottom: var(--spacing-md);">
                { i18n.t(description_key) }
            </p>

            <form onsubmit={on_submit}>
                if !is_github {
                    <div class="form-group">
                        <label for="new_image_url">
                            { i18n.t("create_project.image_label") }
                        </label>
                        <input
                            type="text"
                            id="new_image_url"
                            class="text-input"
                            placeholder={i18n.t("create_project.image_placeholder")}
                            value={(*new_image_url).clone()}
                            onchange={on_input_change}
                            required=true
                        />
                    </div>
                }

                if let Some(err) = &*update_error {
                    <p class="error">{ translate_error(err, &i18n) }</p>
                }

                <button type="submit" class="button-primary" disabled={*is_updating}>
                    {
                        if *is_updating {
                            i18n.t(button_loading_key)
                        } else {
                            i18n.t(button_key)
                        }
                    }
                </button>
            </form>
        </div>
    }
}

// ============================================================================
// ENV MANAGER COMPONENT
// ============================================================================

#[function_component(EnvManager)]
fn env_manager(props: &EnvManagerProps) -> Html {
    let (i18n, _) = use_translation();

    let initial_vars_str = props.current_env_vars.as_ref().map_or_else(String::new, |vars| {
        vars.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    });
    let env_vars_str = use_state(|| initial_vars_str);

    let is_loading = use_state(|| false);
    let error = use_state(|| None::<ApiError>);
    let success = use_state(|| false);

    let on_change = {
        let env_vars_str = env_vars_str.clone();
        let success = success.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlTextAreaElement>()
                .value();
            env_vars_str.set(value);
            success.set(false);
        })
    };

    let on_submit = {
        let env_vars_str = env_vars_str.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let success = success.clone();
        let on_update = props.on_update.clone();
        let project_id = props.project_id;

        Callback::from(move |e: SubmitEvent| {
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

            wasm_bindgen_futures::spawn_local(async move {
                match project_service::update_env_vars(project_id, &payload).await {
                    Ok(_) => {
                        success.set(true);
                        on_update.emit(());
                    }
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
        })
    };

    html! {
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

                if *success {
                    <p class="success-banner" style="margin-bottom: var(--spacing-md); background-color: rgba(126, 211, 33, 0.2); border-color: #7ED321;">
                        { i18n.t("project_dashboard.env_vars_updated_success") }
                    </p>
                }
                if let Some(err) = &*error {
                    <p class="error">{ translate_error(err, &i18n) }</p>
                }

                <button type="submit" class="button-primary" disabled={*is_loading}>
                    {
                        if *is_loading {
                            i18n.t("project_dashboard.save_and_restart_button_loading")
                        } else {
                            i18n.t("project_dashboard.save_and_restart_button")
                        }
                    }
                </button>
            </form>
        </div>
    }
}

// ============================================================================
// DATABASE MANAGER COMPONENT
// ============================================================================

#[function_component(DatabaseManager)]
fn database_manager(props: &DatabaseManagerProps) -> Html {
    let (i18n, _) = use_translation();
    let project_id = props.project_details.project.id;
    let on_update = props.on_update.clone();

    // Scénario 1: Une DB est déjà liée à ce projet
    if let Some(db) = &props.project_details.database {
        let on_unlink = {
            let on_update = on_update.clone();
            Callback::from(move |_| {
                let on_update = on_update.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if database_service::unlink_database_from_project(project_id)
                        .await
                        .is_ok()
                    {
                        on_update.emit(());
                    }
                });
            })
        };

        let on_delete_db = {
            let on_update = on_update.clone();
            let i18n = i18n.clone();
            Callback::from(move |_| {
                if web_sys::window()
                    .unwrap()
                    .confirm_with_message(&i18n.t("database.confirm_delete"))
                    .unwrap()
                {
                    let on_update = on_update.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if database_service::delete_linked_database(project_id)
                            .await
                            .is_ok()
                        {
                            on_update.emit(());
                        }
                    });
                }
            })
        };

        return html! {
            <div>
                <p><strong>{ i18n.t("database.host") }{": "}</strong> <span class="detail-value">{ &db.host }</span></p>
                <p><strong>{ i18n.t("database.port") }{": "}</strong> <span class="detail-value">{ db.port }</span></p>
                <p><strong>{ i18n.t("database.db_name") }{":"}</strong> <span class="detail-value">{ &db.database_name }</span></p>
                <p><strong>{ i18n.t("database.username") }{":"}</strong> <span class="detail-value">{ &db.username }</span></p>
                <p><strong>{ i18n.t("database.password") }{":"}</strong> <span class="detail-value">{ &db.password }</span></p>

                <div style="margin-top: var(--spacing-md);">
                    <a
                        href="https://phpmyadmin.hangar.garageisep.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="button-primary"
                    >
                        { i18n.t("database.open_phpmyadmin") }
                    </a>
                </div>

                if props.has_control_access {
                    <div style="margin-top: var(--spacing-md); display:flex; gap: var(--spacing-md);">
                        <button class="button-danger" onclick={on_unlink}>
                            { i18n.t("database.unlink_button") }
                        </button>
                        <button class="button-danger" onclick={on_delete_db}>
                            { i18n.t("database.delete_button") }
                        </button>
                    </div>
                }
            </div>
        };
    }

    // Scénario 2: L'utilisateur a une BDD personnelle non liée
    if let Some(my_db) = &props.my_database {
        if my_db.project_id.is_none() {
            let on_link_existing = {
                let db_id = my_db.id;
                let on_update = on_update.clone();
                Callback::from(move |_| {
                    let on_update = on_update.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if database_service::link_database_to_project(project_id, db_id)
                            .await
                            .is_ok()
                        {
                            on_update.emit(());
                        }
                    });
                })
            };

            return html! {
                <div>
                    <p>{ i18n.t("database.unlinked_db_found").replace("{name}", &my_db.database_name) }</p>
                    <button class="button-primary" onclick={on_link_existing}>
                        { i18n.t("database.link_this_db_button") }
                    </button>
                </div>
            };
        }
    }

    // Scénario 3: Pas de BDD liée et pas de BDD personnelle disponible
    let error = use_state(|| None::<ApiError>);
    let is_loading = use_state(|| false);
    let on_create_and_link = {
        let is_loading = is_loading.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let on_update = on_update.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            is_loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match database_service::create_database().await {
                    Ok(db) => {
                        if database_service::link_database_to_project(project_id, db.id)
                            .await
                            .is_ok()
                        {
                            on_update.emit(());
                        } else {
                            error.set(Some(ApiError {
                                error_code: "LINK_FAILED".to_string(),
                                details: None,
                            }));
                        }
                    }
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <div>
            <p>{ i18n.t("database.no_db_linked") }</p>
            if let Some(err) = &*error {
                <p class="error">{ translate_error(err, &i18n) }</p>
            }
            <button class="button-primary" onclick={on_create_and_link} disabled={*is_loading}>
                { i18n.t("database.create_and_link_button") }
            </button>
        </div>
    }
}

// ============================================================================
// PROJECT INFO COMPONENT
// ============================================================================

#[derive(Properties, PartialEq)]
struct ProjectInfoProps 
{
    project_details: ProjectDetails,
}

#[function_component(ProjectInfo)]
fn project_info(props: &ProjectInfoProps) -> Html 
{
    let (i18n, _) = use_translation();
    let p = &props.project_details.project;
    let created_at_formatted = p.created_at.split('T').next().unwrap_or("").to_string();
    let created_on_message = i18n
        .t("common.created_on")
        .replace("{date}", &created_at_formatted);

    let project_url = format!("https://{}.hangar.garageisep.com", p.name);    
    html! 
    {
        <div class="card">
            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: var(--spacing-md); margin-bottom: var(--spacing-md);">
                <h2 style="margin-bottom: 0;">{ i18n.t("project_dashboard.card_title_info") }</h2>
                <a href={project_url} target="_blank" rel="noopener noreferrer" class="button-primary">{ i18n.t("project_dashboard.visit_app_button") }</a>
            </div>
            <p>
                { i18n.t("common.status") }{ ": " }
                <ProjectStatus project_id={p.id} />
            </p>
            <p>{ format!("{}: {}", i18n.t("common.owner"), p.owner) }</p>
            
            if !props.project_details.participants.is_empty() {
                <p>
                    { i18n.t("project_dashboard.participants_list_label") }
                    { " " }
                    { props.project_details.participants.join(", ") }
                </p>
            }
            
            <p>{ created_on_message }</p>
            <p style="word-break: break-all;">
                { format!("{}: {}", i18n.t("common.source_url"), p.source_url) }
            </p>
            
            if let Some(branch) = &p.source_branch {
                <p>{ format!("Branche GitHub: {}", branch) }</p>
            }
            
            if let Some(root_dir) = &p.source_root_dir {
                <p>{ format!("Dossier Racine: {}", root_dir) }</p>
            }
            
            <p style="word-break: break-all;">
                { format!("{}: {}", i18n.t("common.deployed_image"), p.deployed_image_tag) }
            </p>
            
            if let Some(path) = &p.persistent_volume_path {
                <p>
                    { format!("{}: {}", i18n.t("project_dashboard.persistent_volume_label"), path) }
                </p>
            }
        </div>
    }
}

// ============================================================================
// DATABASE INFO CARD COMPONENT
// ============================================================================

#[derive(Properties, PartialEq)]
struct DatabaseInfoCardProps {
    project_details: ProjectDetails,
    my_database: Option<DatabaseDetails>,
    has_control_access: bool,
    on_update: Callback<()>,
}

#[function_component(DatabaseInfoCard)]
fn database_info_card(props: &DatabaseInfoCardProps) -> Html {
    let (i18n, _) = use_translation();

    html! {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("database.title") }</h2>
            {
                if props.has_control_access {
                    html! {
                        <DatabaseManager
                            project_details={props.project_details.clone()}
                            my_database={props.my_database.clone()}
                            has_control_access={props.has_control_access}
                            on_update={props.on_update.clone()}
                        />
                    }
                } else if let Some(db) = &props.project_details.database {
                    html! {
                        <div>
                            <p><strong>{ i18n.t("database.host") }{": "}</strong> <span class="detail-value">{ &db.host }</span></p>
                            <p><strong>{ i18n.t("database.port") }{": "}</strong> <span class="detail-value">{ db.port }</span></p>
                            <p><strong>{ i18n.t("database.db_name") }{":"}</strong> <span class="detail-value">{ &db.database_name }</span></p>
                            <p><strong>{ i18n.t("database.username") }{":"}</strong> <span class="detail-value">{ &db.username }</span></p>
                            <p><strong>{ i18n.t("database.password") }{":"}</strong> <span class="detail-value">{ &db.password }</span></p>

                            <div style="margin-top: var(--spacing-md);">
                                <a
                                    href="https://phpmyadmin.hangar.garageisep.com"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="button-primary"
                                >
                                    { i18n.t("database.open_phpmyadmin") }
                                </a>
                            </div>
                        </div>
                    }
                } else {
                    html! { <p>{ i18n.t("database.no_db_linked") }</p> }
                }
            }
        </div>
    }
}

// ============================================================================
// DANGER ZONE COMPONENT
// ============================================================================

#[derive(Properties, PartialEq)]
struct DangerZoneProps {
    project_id: i32,
    project_name: String,
    has_linked_database: bool,
}

#[function_component(DangerZone)]
fn danger_zone(props: &DangerZoneProps) -> Html {
    let (i18n, _) = use_translation();
    let navigator = use_navigator().unwrap();
    let deletion_error = use_state(|| None::<String>);

    let on_delete = {
        let project_name = props.project_name.clone();
        let has_linked_db = props.has_linked_database;
        let project_id = props.project_id;
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        let deletion_error = deletion_error.clone();

        Callback::from(move |_| {
            let mut confirm_message = i18n
                .t("project_dashboard.confirm_delete")
                .replace("{name}", &project_name);
            
            if has_linked_db {
                confirm_message.push_str(&format!(
                    "\n\n{}",
                    i18n.t("project_dashboard.confirm_delete_db_warning")
                ));
            }

            if web_sys::window()
                .unwrap()
                .confirm_with_message(&confirm_message)
                .unwrap()
            {
                let navigator = navigator.clone();
                let deletion_error = deletion_error.clone();
                let i18n = i18n.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    if project_service::purge_project(project_id).await.is_ok() {
                        navigator.push(&AppRoute::Home);
                    } else {
                        deletion_error.set(Some(i18n.t("errors.DELETE_FAILED")));
                    }
                });
            }
        })
    };

    html! {
        <div class="card" style="margin-top: var(--spacing-lg); border-color: var(--color-danger);">
            <h2>{ i18n.t("project_dashboard.card_title_danger") }</h2>
            
            if let Some(error_msg) = &*deletion_error {
                <p class="error" style="margin-top: var(--spacing-md)">{ error_msg }</p>
            }
            
            <button class="button-danger" onclick={on_delete}>
                { i18n.t("project_dashboard.delete_button") }
            </button>
        </div>
    }
}

// ============================================================================
// MAIN DASHBOARD COMPONENT
// ============================================================================

#[function_component(ProjectDashboard)]
pub fn project_dashboard(props: &ProjectDashboardProps) -> Html {
    let (i18n, _) = use_translation();
    let user_context = use_user();

    let project_details = use_state(|| None::<ProjectDetails>);
    let my_database = use_state(|| None::<Option<DatabaseDetails>>);
    let error = use_state(|| None::<String>);
    let trigger_reload = use_state(|| 0_u32);

    // Fetch project details and database info
    {
        let project_details = project_details.clone();
        let my_database = my_database.clone();
        let error = error.clone();
        let project_id = props.project_id;

        use_effect_with((*trigger_reload, project_id), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match project_service::get_project_details(project_id).await {
                    Ok(pd) => project_details.set(Some(pd)),
                    Err(e) => error.set(Some(e)),
                }
            });

            let my_database = my_database.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match database_service::get_my_database().await {
                    Ok(db) => my_database.set(Some(Some(db))),
                    Err(_) => my_database.set(Some(None)),
                }
            });
            || ()
        });
    }

    let on_update = {
        let trigger_reload = trigger_reload.clone();
        Callback::from(move |_| {
            trigger_reload.set(*trigger_reload + 1);
        })
    };

    // Error state
    if let Some(e) = &*error {
        let error_message = i18n
            .t("project_dashboard.load_error_message")
            .replace("{error}", e);
        return html! {
            <div class="card error">
                <h2>{ i18n.t("project_dashboard.access_error_title") }</h2>
                <p>{ error_message }</p>
                <Link<AppRoute> to={AppRoute::Home} classes="button-primary">
                    { i18n.t("common.back_to_home") }
                </Link<AppRoute>>
            </div>
        };
    }

    // Loading state
    let Some(details) = &*project_details else {
        return html! { <div class="loading-spinner">{ i18n.t("common.loading") }</div> };
    };
    let Some(my_db_option) = &*my_database else {
        return html! { <div class="loading-spinner">{ i18n.t("common.loading") }</div> };
    };

    let p = &details.project;
    let has_strong_access = user_context
        .user
        .as_ref()
        .map_or(false, |u| u.is_admin || u.login == p.owner);

    let has_weak_access = user_context
        .user
        .as_ref()
        .map_or(false, |u| u.is_admin || u.login == p.owner || details.participants.contains(&u.login));

    html! {
        <div>
            <h1>{ i18n.t("project_dashboard.title") }{ format!(": {}", p.name) }</h1>

            <ProjectInfo project_details={details.clone()} />

            <DatabaseInfoCard
                project_details={details.clone()}
                my_database={my_db_option.clone()}
                has_control_access={has_strong_access}
                on_update={on_update.clone()}
            />

            if has_weak_access {
                <ProjectControls project_id={p.id} on_update={on_update.clone()} />
            }

            <ProjectLogs project_id={p.id} />

            <div class="card" style="margin-top: var(--spacing-lg);">
                <h2>{ i18n.t("project_dashboard.card_title_metrics") }</h2>
                <ProjectMetricsDisplay project_id={p.id} />
            </div>

            if has_strong_access 
            {
                <ParticipantManager
                    project_id={p.id}
                    participants={details.participants.clone()}
                    on_update={on_update.clone()}
                />
            }

            if has_weak_access
            {
                <EnvManager
                    project_id={p.id}
                    current_env_vars={p.env_vars.clone()}
                    on_update={on_update.clone()}
                />

                <ImageUpdateForm
                    project_id={p.id}
                    project_name={p.name.clone()}
                    source_type={p.source.clone()}
                    on_update={on_update.clone()}
                />
            }  
                
            if has_strong_access 
            {
                <DangerZone
                    project_id={p.id}
                    project_name={p.name.clone()}
                    has_linked_database={details.database.is_some()}
                />
            }
            
        </div>
    }
}