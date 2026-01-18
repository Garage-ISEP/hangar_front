use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::sse_context::{use_sse_deployment, SseProvider};
use crate::contexts::user_context::use_user;
use crate::models::database::DatabaseDetails;
use crate::models::project::ProjectDetails;
use crate::router::AppRoute;
use crate::services::project_service::ApiError;
use crate::services::sse_service::{ContainerStatus, DeploymentStage};
use crate::services::{database_service, project_service};

use crate::components::{
    database_card::DatabaseCard,
    danger_zone::DangerZone,
    env_manager::EnvManager,
    image_update_form::ImageUpdateForm,
    participant_manager::ParticipantManager,
    project_controls::ProjectControls,
    project_info::ProjectInfo,
    project_logs::ProjectLogs,
    project_metrics::ProjectMetrics,
};

const RELOAD_DELAY_MS: u32 = 1500;

#[derive(Properties, PartialEq)]
pub struct ProjectDashboardProps
{
    pub project_id: i32,
}

#[function_component(ProjectDashboard)]
pub fn project_dashboard(props: &ProjectDashboardProps) -> Html
{
    html!
    {
        <SseProvider project_id={props.project_id}>
            <ProjectDashboardInner project_id={props.project_id} />
        </SseProvider>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectDashboardInnerProps
{
    project_id: i32,
}

#[function_component(ProjectDashboardInner)]
fn project_dashboard_inner(props: &ProjectDashboardInnerProps) -> Html
{
    let (i18n, _) = use_translation();
    let user_context = use_user();
    
    let project_details = use_state(|| None::<ProjectDetails>);
    let my_database = use_state(|| None::<Option<DatabaseDetails>>);
    let error = use_state(|| None::<String>);
    let trigger_reload = use_state(|| 0_u32);

    {
        let trigger_reload = trigger_reload.clone();
        let deployment_stage = use_sse_deployment();

        use_effect_with(deployment_stage, move |stage|
        {
            if matches!(stage, Some(DeploymentStage::Completed { .. }))
            {
                gloo_timers::callback::Timeout::new(RELOAD_DELAY_MS, move ||
                {
                    trigger_reload.set(*trigger_reload + 1);
                })
                .forget();
            }
            || ()
        });
    }

    {
        let project_details = project_details.clone();
        let my_database = my_database.clone();
        let error = error.clone();
        let project_id = props.project_id;

        use_effect_with((*trigger_reload, project_id), move |_|
        {
            wasm_bindgen_futures::spawn_local(async move
            {
                match project_service::get_project_details(project_id).await
                {
                    Ok(pd) => project_details.set(Some(pd)),
                    Err(e) => error.set(Some(e)),
                }
            });

            let my_database = my_database.clone();
            wasm_bindgen_futures::spawn_local(async move
            {
                match database_service::get_my_database().await
                {
                    Ok(db) => my_database.set(Some(Some(db))),
                    Err(_) => my_database.set(Some(None)),
                }
            });
            || ()
        });
    }

    let on_update =
    {
        let trigger_reload = trigger_reload.clone();
        Callback::from(move |_|
        {
            trigger_reload.set(*trigger_reload + 1);
        })
    };

    if let Some(e) = &*error
    {
        let error_message = i18n
            .t("project_dashboard.load_error_message")
            .replace("{error}", e);
        return html!
        {
            <div class="card error">
                <h2>{ i18n.t("project_dashboard.access_error_title") }</h2>
                <p>{ error_message }</p>
                <Link<AppRoute> to={AppRoute::Home} classes="button-primary">
                    { i18n.t("common.back_to_home") }
                </Link<AppRoute>>
            </div>
        };
    }

    let Some(details) = &*project_details
    else
    {
        return html! { <div class="loading-spinner">{ i18n.t("common.loading") }</div> };
    };
    
    let Some(my_db_option) = &*my_database
    else
    {
        return html! { <div class="loading-spinner">{ i18n.t("common.loading") }</div> };
    };

    let p = &details.project;
    let has_strong_access = user_context
        .user
        .as_ref()
        .map_or(false, |u| u.is_admin || u.login == p.owner);

    let has_weak_access = user_context.user.as_ref().map_or(false, |u|
    {
        u.is_admin || u.login == p.owner || details.participants.contains(&u.login)
    });

    html!
    {
        <div>
            <h1>{ i18n.t("project_dashboard.title") }{ format!(": {}", p.name) }</h1>

            <ProjectInfo project_details={details.clone()} />

            <DatabaseCard
                project_details={details.clone()}
                my_database={my_db_option.clone()}
                has_control_access={has_strong_access}
                on_update={on_update.clone()}
            />

            if has_weak_access
            {
                <ProjectControls
                    project_id={p.id}
                    on_update={on_update.clone()}
                />
            }

            <ProjectLogs project_id={p.id} />

            <ProjectMetrics />

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


pub fn render_log_line(line: &str) -> Html 
{
    let (timestamp, message) = parse_log_line(line);
    let log_level_class = determine_log_level(message);

    html! 
    {
        <div class="log-line">
            <span class="log-timestamp">{ format_timestamp(timestamp) }</span>
            <span class={classes!("log-message", log_level_class)}>{ message }</span>
        </div>
    }
}

fn parse_log_line(line: &str) -> (&str, &str) 
{
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() == 2 && parts[0].ends_with('Z') 
    {
        (parts[0], parts[1].trim())
    } 
    else 
    {
        ("", line)
    }
}

fn format_timestamp(timestamp: &str) -> &str 
{
    timestamp.split('.').next().unwrap_or(timestamp)
}

fn determine_log_level(message: &str) -> &'static str 
{
    let message_upper = message.to_uppercase();
    if message_upper.contains("ERROR") || message_upper.contains("FAILED") 
    {
        "log-error"
    } 
    else if message_upper.contains("WARN") || message_upper.contains("WARNING") 
    {
        "log-warn"
    } 
    else 
    {
        "log-info"
    }
}

pub fn translate_status(status: &ContainerStatus, i18n: &i18nrs::I18n) -> String 
{
    let key = format!("common.{}", get_status_class(status));
    let translation = i18n.t(&key);

    if is_translation_missing(&translation) 
    {
        i18n.t("common.status_unknown")
    } 
    else 
    {
        translation
    }
}

pub fn translate_error(error: &ApiError, i18n: &i18nrs::I18n) -> String 
{
    let error_key = format!("errors.{}", error.error_code);
    let translation = i18n.t(&error_key);

    if is_translation_missing(&translation) 
    {
        i18n.t("errors.DEFAULT")
    } 
    else
    {
        translation
    }
}

fn is_translation_missing(translation: &str) -> bool 
{
    translation.starts_with("Key '") && translation.contains(" not found for language ")
}

pub fn get_status_class(status: &ContainerStatus) -> &'static str 
{
    match status 
    {
        ContainerStatus::Running => "status_running",
        ContainerStatus::Created => "status_created",
        ContainerStatus::Restarting => "status_restarting",
        ContainerStatus::Paused => "status_paused",
        ContainerStatus::Exited => "status_exited",
        ContainerStatus::Dead => "status_dead",
        ContainerStatus::Removing => "status_removing",
        ContainerStatus::Unknown => "status_unknown",
    }
}
