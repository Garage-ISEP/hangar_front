use crate::models::database::AdminDatabaseInfo;
use crate::models::project::{DownProjectInfo, GlobalMetrics, Project};
use crate::router::AppRoute;
use crate::services::project_service;
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::Link;

fn format_downtime(seconds: i64) -> String 
{
    if seconds < 60 
    {
        format!("{}s", seconds)
    } 
    else if seconds < 3600 
    {
        format!("{}m", seconds / 60)
    } 
    else if seconds < 86400 
    {
        format!("{}h", seconds / 3600)
    } 
    else 
    {
        format!("{}d", seconds / 86400)
    }
}

#[function_component(Admin)]
pub fn admin() -> Html 
{
    let (i18n, _) = use_translation();
    
    let metrics = use_state(|| None::<GlobalMetrics>);
    let down_projects = use_state(|| None::<Vec<DownProjectInfo>>);
    let all_projects = use_state(|| None::<Vec<Project>>);
    let all_databases = use_state(|| None::<Vec<AdminDatabaseInfo>>);

    {
        let metrics = metrics.clone();
        let down_projects = down_projects.clone();
        let all_projects = all_projects.clone();
        let all_databases = all_databases.clone();

        use_effect_with((), move |_| 
        {
            // Fetch Global Metrics
            wasm_bindgen_futures::spawn_local(async move 
            {
                if let Ok(m) = project_service::get_global_metrics_admin().await 
                {
                    metrics.set(Some(m));
                }
            });
            // Fetch Down Projects
            wasm_bindgen_futures::spawn_local(async move 
            {
                if let Ok(p) = project_service::get_down_projects_admin().await 
                {
                    down_projects.set(Some(p));
                }
            });
            // Fetch All Projects
            wasm_bindgen_futures::spawn_local(async move 
            {
                if let Ok(p) = project_service::get_all_projects_admin().await
                {
                    all_projects.set(Some(p));
                }
            });
            // Fetch All Databases
            wasm_bindgen_futures::spawn_local(async move
            {
                if let Ok(d) = project_service::get_all_databases_admin().await
                {
                    all_databases.set(Some(d));
                }
            });
            || ()
        });
    }

    html! 
    {
        <div>
            <h1>{ i18n.t("admin.title") }</h1>
            
            // Section Métriques
            <div class="card" style="margin-bottom: var(--spacing-lg)">
                <h2>{ i18n.t("admin.global_metrics_title") }</h2>
                {
                    if let Some(m) = &*metrics 
                    {
                        html! 
                        {
                            <div class="metrics-grid" style="justify-items: start;">
                                <p><strong>{ "Total Projects:" }</strong><span class="detail-value">{ m.total_projects }</span></p>
                                <p><strong>{ "Running Containers:" }</strong><span class="detail-value">{ m.running_containers }</span></p>
                                <p><strong>{ "Total CPU Usage:" }</strong><span class="detail-value">{ format!("{:.2}%", m.total_cpu_usage) }</span></p>
                                <p><strong>{ "Total Memory Usage:" }</strong><span class="detail-value">{ format!("{:.2} MB", m.total_memory_usage_mb) }</span></p>
                            </div>
                        }
                    } 
                    else 
                    {
                        html! { <p>{ i18n.t("common.loading") }</p> }
                    }
                }
            </div>

            // Section Alertes (Projets Down)
            <div class="card" style="margin-bottom: var(--spacing-lg); border-color: var(--color-danger)">
                <h2>{ "Alerts: Down Projects" }</h2>
                {
                    if let Some(projects) = &*down_projects 
                    {
                        if projects.is_empty() 
                        {
                            html! { <p style="color: var(--color-success)">{ "All projects are up and running!" }</p> }
                        } 
                        else 
                        {
                            html! 
                            {
                                <ul style="list-style: none;">
                                    {
                                        for projects.iter().map(|p| html! 
                                        {
                                            <li style="display: flex; justify-content: space-between; align-items: center; padding: var(--spacing-sm) 0; border-bottom: 1px solid var(--color-border);">
                                                <div>
                                                    <Link<AppRoute> to={AppRoute::ProjectDashboard { id: p.project.id }}>
                                                        <strong>{ &p.project.name }</strong>
                                                    </Link<AppRoute>>
                                                    <span style="color: var(--color-text-secondary);">{ format!(" (Owner: {})", p.project.owner) }</span>
                                                </div>
                                                <span class="status-badge status-stopped">
                                                    { format!("Down for {}", format_downtime(p.downtime_seconds)) }
                                                </span>
                                            </li>
                                        }) 
                                    }
                                </ul>
                            }
                        }
                    } 
                    else 
                    {
                        html! { <p>{ i18n.t("common.loading") }</p> }
                    }
                }
            </div>

            // Section Tous les projets
            <div class="card">
                <h2>{ i18n.t("admin.all_projects_title") }</h2>
                {
                    if let Some(projects) = &*all_projects 
                    {
                        html! 
                        {
                            <ul style="list-style: none;">
                                { 
                                    for projects.iter().map(|p| html! 
                                    {
                                        <li style="display: flex; justify-content: space-between; align-items: center; padding: var(--spacing-sm) 0; border-bottom: 1px solid var(--color-border);">
                                            <Link<AppRoute> to={AppRoute::ProjectDashboard { id: p.id }}>
                                                <strong>{ &p.name }</strong>
                                            </Link<AppRoute>>
                                            <span class="detail-value">{ &p.owner }</span>
                                        </li>
                                    })
                                }
                            </ul>
                        }
                    } 
                    else
                    {
                        html!{ <p>{ i18n.t("common.loading") }</p> }
                    }
                }
            </div>

            // Section Toutes les bases de données
            <div class="card" style="margin-top: var(--spacing-lg)">
                <h2>{ i18n.t("admin.all_databases_title") }</h2>
                {
                    if let Some(databases) = &*all_databases
                    {
                        if databases.is_empty()
                        {
                            html! { <p>{ i18n.t("admin.no_databases") }</p> }
                        }
                        else
                        {
                            html!
                            {
                                <ul style="list-style: none;">
                                    {
                                        for databases.iter().map(|d| html!
                                        {
                                            <li style="display: flex; justify-content: space-between; align-items: center; padding: var(--spacing-sm) 0; border-bottom: 1px solid var(--color-border);">
                                                <div>
                                                    <strong>{ &d.database_name }</strong>
                                                    <span style="color: var(--color-text-secondary);">{ format!(" (Owner: {})", d.owner_login) }</span>
                                                </div>
                                                {
                                                    match d.project_id
                                                    {
                                                        Some(pid) => html!
                                                        {
                                                            <Link<AppRoute> to={AppRoute::ProjectDashboard { id: pid }}>
                                                                <span class="status-badge status_running">
                                                                    { format!("Linked to project #{}", pid) }
                                                                </span>
                                                            </Link<AppRoute>>
                                                        },
                                                        None => html!
                                                        {
                                                            <span class="status-badge status_restarting">
                                                                { "Standalone" }
                                                            </span>
                                                        },
                                                    }
                                                }
                                            </li>
                                        })
                                    }
                                </ul>
                            }
                        }
                    }
                    else
                    {
                        html!{ <p>{ i18n.t("common.loading") }</p> }
                    }
                }
            </div>
        </div>
    }
}