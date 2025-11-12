use yew::prelude::*;
use yew_router::prelude::*;
use i18nrs::yew::use_translation;
use crate::
{
    models::database::DatabaseDetails,
    services::{database_service, project_service},
    router::AppRoute,
    services::project_service::ApiError
};

#[derive(Properties, PartialEq)]
pub struct DatabaseDashboardProps
{
    pub db_id: i32,
}

#[function_component(DatabaseDashboard)]
pub fn database_dashboard(props: &DatabaseDashboardProps) -> Html
{
    let (i18n, _) = use_translation();
    let navigator = use_navigator().unwrap();
    
    let db_details = use_state(|| None::<DatabaseDetails>);
    let projects = use_state(|| vec![]);
    let error = use_state(|| None::<ApiError>);
    let selected_project_to_link = use_state(String::new);

    {
        let db_details = db_details.clone();
        let projects = projects.clone();
        let error = error.clone();

        use_effect_with((), move |_|
        {
            wasm_bindgen_futures::spawn_local(async move 
            {
                match database_service::get_my_database().await
                {
                    Ok(db) => db_details.set(Some(db)),
                    Err(e) => error.set(Some(e)),
                }
                match project_service::get_owned_projects().await
                {
                    Ok(projs) => projects.set(projs),
                    Err(_) => {}, // Ignore error, linking will just not be possible
                }
            });
            || ()
        });
    }

    let on_delete =
    {
        let navigator = navigator.clone();
        let db_id = props.db_id;
        let i18n = i18n.clone();

        Callback::from(move |_| 
        {
            if web_sys::window().unwrap().confirm_with_message(&i18n.t("database.confirm_delete")).unwrap()
            {
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move 
                {
                    if database_service::delete_database(db_id).await.is_ok()
                    {
                        navigator.push(&AppRoute::Home);
                    }
                });
            }
        })
    };

    let on_project_select = 
    {
        let selected_project_to_link = selected_project_to_link.clone();
        Callback::from(move |e: Event| 
        {
            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
            selected_project_to_link.set(value);
        })
    };

    let on_link = 
    {
        let selected_project_to_link = selected_project_to_link.clone();
        let navigator = navigator.clone();
        let db_id = props.db_id;

        Callback::from(move |e: SubmitEvent| 
        {
            e.prevent_default();
            let project_id_str = &*selected_project_to_link;
            if let Ok(project_id) = project_id_str.parse::<i32>()
            {
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move 
                {
                    if database_service::link_database_to_project(project_id, db_id).await.is_ok()
                    {
                        navigator.push(&AppRoute::ProjectDashboard { id: project_id });
                    }
                });
            }
        })
    };


    if let Some(db) = &*db_details
    {
        html! 
        {
            <div>
                <h1>{ i18n.t("database.dashboard_title") }</h1>

                <div class="card">
                    <h2>{ i18n.t("database.connection_info_title") }</h2>
                    <p><strong>{ "Host:" }</strong> <span class="detail-value">{ &db.host }</span></p>
                    <p><strong>{ "Port:" }</strong> <span class="detail-value">{ db.port }</span></p>
                    <p><strong>{ i18n.t("database.db_name") }{":"}</strong> <span class="detail-value">{ &db.database_name }</span></p>
                    <p><strong>{ i18n.t("database.username") }{":"}</strong> <span class="detail-value">{ &db.username }</span></p>
                    <p><strong>{ i18n.t("database.password") }{":"}</strong> <span class="detail-value">{ &db.password }</span></p>

                    <div style="margin-top: var(--spacing-lg);">
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

                <div class="card" style="margin-top: var(--spacing-lg);">
                    <h2>{ i18n.t("database.link_to_project_title") }</h2>
                    if projects.is_empty()
                    {
                        <p>{ i18n.t("database.no_projects_to_link") }</p>
                    }
                    else
                    {
                        <form onsubmit={on_link}>
                            <select class="text-input" onchange={on_project_select} required=true>
                                <option value="" disabled=true selected=true>{ i18n.t("database.select_project") }</option>
                                { for projects.iter().map(|p| html!{ <option value={p.id.to_string()}>{ &p.name }</option> }) }
                            </select>
                            <button type="submit" class="button-primary" style="margin-top: var(--spacing-md);">{ i18n.t("database.link_button") }</button>
                        </form>
                    }
                </div>

                <div class="card" style="margin-top: var(--spacing-lg); border-color: var(--color-danger);">
                    <h2>{ i18n.t("project_dashboard.card_title_danger") }</h2>
                    <button class="button-danger" onclick={on_delete}>{ i18n.t("database.delete_button") }</button>
                </div>
            </div>
        }
    }
    else if let Some(e) = &*error
    {
        html!{ <p class="error">{ format!("Error loading database: {}", e.error_code) }</p> }
    }
    else
    {
        html!{ <p>{ i18n.t("common.loading") }</p> }
    }
}