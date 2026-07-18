use crate::
{
    contexts::user_context::use_user,
    models::{database::DatabaseDetails, project::{Project, ProjectSourceType}},
    router::AppRoute,
    services::{database_service, project_service},
};
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

const CAS_LOGIN_URL: &str = "https://portail-ovh.isep.fr/cas/login";

#[function_component(Home)]
pub fn home() -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();

    if user_context.user.is_some() 
    {
        html! { <Dashboard /> }
    } 
    else 
    {
        let callback_url = format!(
            "{}/auth/callback",
            web_sys::window().unwrap().location().origin().unwrap()
        );
        let login_url = format!("{}?service={}", CAS_LOGIN_URL, callback_url);
        html! 
        {
            <div class="home-page" style="text-align: center; margin-top: 10vh; display: flex; flex-direction: column; align-items: center; gap: var(--spacing-lg);">
                <img src="/assets/logo-garageisep-white.svg" alt="GarageIsep Logo" style="height: 80px;" />
                <h1>{ i18n.t("home.title") }</h1>
                <p style="max-width: 500px;">{ i18n.t("home.description") }</p>
                <a href={login_url} class="button-gradient">{ i18n.t("home.login_button") }</a>
            </div>
        }
    }
}

#[function_component(Dashboard)]
fn dashboard() -> Html 
{
    let (i18n, _) = use_translation();
    let user_context = use_user();
    let owned_projects = use_state(|| None::<Vec<Project>>);
    let participating_projects = use_state(|| None::<Vec<Project>>);
    let unlinked_db = use_state(|| None::<DatabaseDetails>);

    {
        let owned_projects = owned_projects.clone();
        let participating_projects = participating_projects.clone();
        let unlinked_db = unlinked_db.clone();

        use_effect_with((), move |_| 
        {
            wasm_bindgen_futures::spawn_local(async move 
            {
                match project_service::get_owned_projects().await 
                {
                    Ok(projects) => owned_projects.set(Some(projects)),
                    Err(e) => 
                    {
                        gloo_console::error!("Failed to fetch owned projects:", e);
                        owned_projects.set(Some(vec![]));
                    }
                }
                match project_service::get_participating_projects().await 
                {
                    Ok(projects) => participating_projects.set(Some(projects)),
                    Err(e) => 
                    {
                        gloo_console::error!("Failed to fetch participating projects:", e);
                        participating_projects.set(Some(vec![]));
                    }
                }
                let unlinked_db = unlinked_db.clone();
                wasm_bindgen_futures::spawn_local(async move 
                {
                    if let Ok(db) = database_service::get_my_database().await
                        && db.project_id.is_none() 
                        {
                            unlinked_db.set(Some(db));
                        }
                });

            });
            || ()
        });
    }

    let welcome_message = i18n
        .t("dashboard.welcome")
        .replace("{name}", &user_context.user.as_ref().unwrap().name);

    html! 
    {
        <div class="dashboard-home">
            <div class="dashboard-header">
                <h1>{ welcome_message }</h1>
                <Link<AppRoute> to={AppRoute::CreateProject} classes="button-primary">
                    { i18n.t("dashboard.create_project_button") }
                </Link<AppRoute>>
            </div>
            <p>{ i18n.t("dashboard.description") }</p>

            <section class="projects-section">
                <h2>{ i18n.t("dashboard.owned_projects_title") }</h2>
                <ProjectGrid projects={(*owned_projects).clone()} unlinked_db={(*unlinked_db).clone()} empty_message={i18n.t("dashboard.empty_state_owned")} />
            </section>

            <section class="projects-section" style="margin-top: var(--spacing-xxl)">
                <h2>{ i18n.t("dashboard.participating_projects_title") }</h2>
                <ProjectGrid projects={(*participating_projects).clone()} empty_message={i18n.t("dashboard.empty_state_participating")} />
            </section>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectGridProps 
{
    projects: Option<Vec<Project>>,
    #[prop_or_default] 
    unlinked_db: Option<DatabaseDetails>,
    empty_message: String,
}

#[function_component(ProjectGrid)]
fn project_grid(props: &ProjectGridProps) -> Html 
{
    let (i18n, _) = use_translation();

    match (&props.projects, &props.unlinked_db)
    {
        (Some(projects), db) if projects.is_empty() && db.is_none() => html! 
        {
             <div class="empty-state">
                <p>{ &props.empty_message }</p>
            </div>
        },
        (Some(projects), db) => html! 
        {
            <div class="project-grid">
                { for projects.iter().map(|p| project_card(p, &i18n)) }
                {
                    if let Some(db_details) = db 
                    {
                        database_card(db_details, &i18n)
                    } 
                    else 
                    {
                        html!{}
                    }
                }
            </div>
        },
        (None, _) => html! { <div class="loading-spinner">{ i18n.t("common.loading") }</div> },
    }
}

fn project_card(project: &Project, i18n: &i18nrs::I18n) -> Html 
{
    let (source_icon, source_title) = match project.source 
    {
        ProjectSourceType::Github => ("/assets/github-mark-white.svg", "GitHub"),
        ProjectSourceType::Direct => ("/assets/docker-logo-white.svg", "Direct Image"),
    };

    html! 
    {
        <Link<AppRoute> to={AppRoute::ProjectDashboard { id: project.id }} classes="card-link">
            <div class="card project-card">
                <div class="project-header">
                    <h3>{ &project.name }</h3>
                    <img src={source_icon} title={source_title} alt={source_title} style="height: 24px; width: 24px;" />
                </div>
                <div class="project-details">
                    <span>{ i18n.t("common.owner") }</span>
                    <span class="detail-value">{ &project.owner }</span>
                </div>
                 <div class="project-details">
                    <span>{ i18n.t("common.source_url") }</span>
                    <span class="detail-value" style="word-break: break-all;">{ &project.source_url }</span>
                </div>
            </div>
        </Link<AppRoute>>
    }
}

fn database_card(db: &DatabaseDetails, i18n: &i18nrs::I18n) -> Html
{
    html!
    {
        <Link<AppRoute> to={AppRoute::DatabaseDashboard { id: db.id }} classes="card-link">
            <div class="card project-card">
                <div class="project-header">
                    <h3>{ &db.database_name }</h3>
                    <img src="/assets/database.svg" title="Database" alt="Database" style="height: 24px; width: 24px;" />
                </div>
                 <div class="project-details">
                    <span>{ i18n.t("database.db_name") }</span>
                    <span class="detail-value" style="word-break: break-all;">{ &db.database_name }</span>
                </div>
                <div class="project-details">
                    <span>{ i18n.t("database.username") }</span>
                    <span class="detail-value">{ &db.username }</span>
                </div>
            </div>
        </Link<AppRoute>>
    }
}