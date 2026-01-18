use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::models::database::DatabaseDetails;
use crate::models::project::ProjectDetails;
use crate::pages::project_dashboard::translate_error;
use crate::services::database_service;
use crate::services::project_service::ApiError;

#[derive(Properties, PartialEq)]
pub struct DatabaseCardProps
{
    pub project_details: ProjectDetails,
    pub my_database: Option<DatabaseDetails>,
    pub has_control_access: bool,
    pub on_update: Callback<()>,
}

#[function_component(DatabaseCard)]
pub fn database_card(props: &DatabaseCardProps) -> Html
{
    let (i18n, _) = use_translation();

    html! 
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("database.title") }</h2>
            {
                if props.has_control_access 
                {
                    html! 
                    {
                        <DatabaseManager
                            project_details={props.project_details.clone()}
                            my_database={props.my_database.clone()}
                            on_update={props.on_update.clone()}
                        />
                    }
                } 
                else if let Some(db) = &props.project_details.database 
                {
                    html! 
                    {
                        <DatabaseDisplay database={db.clone()} />
                    }
                } 
                else 
                {
                    html! { <p>{ i18n.t("database.no_db_linked") }</p> }
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DatabaseDisplayProps
{
    database: DatabaseDetails,
}

#[function_component(DatabaseDisplay)]
fn database_display(props: &DatabaseDisplayProps) -> Html
{
    let (i18n, _) = use_translation();
    let db = &props.database;

    html! 
    {
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
}

#[derive(Properties, PartialEq)]
struct DatabaseManagerProps
{
    project_details: ProjectDetails,
    my_database: Option<DatabaseDetails>,
    on_update: Callback<()>,
}

#[function_component(DatabaseManager)]
fn database_manager(props: &DatabaseManagerProps) -> Html
{
    let (i18n, _) = use_translation();
    let project_id = props.project_details.project.id;
    let on_update = props.on_update.clone();

    // Scénario 1: Une DB est déjà liée à ce projet
    if let Some(db) = &props.project_details.database
    {
        let on_unlink = 
        {
            let on_update = on_update.clone();
            Callback::from(move |_| 
            {
                let on_update = on_update.clone();
                wasm_bindgen_futures::spawn_local(async move 
                {
                    if database_service::unlink_database_from_project(project_id)
                        .await
                        .is_ok()
                    {
                        on_update.emit(());
                    }
                });
            })
        };

        let on_delete_db = 
        {
            let on_update = on_update.clone();
            let i18n = i18n.clone();
            Callback::from(move |_| 
            {
                if web_sys::window()
                    .unwrap()
                    .confirm_with_message(&i18n.t("database.confirm_delete"))
                    .unwrap()
                {
                    let on_update = on_update.clone();
                    wasm_bindgen_futures::spawn_local(async move 
                    {
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

        return html! 
        {
            <div>
                <DatabaseDisplay database={db.clone()} />

                <div style="margin-top: var(--spacing-md); display:flex; gap: var(--spacing-md);">
                    <button class="button-danger" onclick={on_unlink}>
                        { i18n.t("database.unlink_button") }
                    </button>
                    <button class="button-danger" onclick={on_delete_db}>
                        { i18n.t("database.delete_button") }
                    </button>
                </div>
            </div>
        };
    }

    // Scénario 2: L'utilisateur a une BDD personnelle non liée
    if let Some(my_db) = &props.my_database
    {
        if my_db.project_id.is_none()
        {
            let on_link_existing = 
            {
                let db_id = my_db.id;
                let on_update = on_update.clone();
                Callback::from(move |_| 
                {
                    let on_update = on_update.clone();
                    wasm_bindgen_futures::spawn_local(async move 
                    {
                        if database_service::link_database_to_project(project_id, db_id)
                            .await
                            .is_ok()
                        {
                            on_update.emit(());
                        }
                    });
                })
            };

            return html! 
            {
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
    let on_create_and_link = 
    {
        let is_loading = is_loading.clone();
        let error = error.clone();
        Callback::from(move |_| 
        {
            let on_update = on_update.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            is_loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move 
            {
                match database_service::create_database().await
                {
                    Ok(db) =>
                    {
                        if database_service::link_database_to_project(project_id, db.id)
                            .await
                            .is_ok()
                        {
                            on_update.emit(());
                        }
                        else
                        {
                            error.set(Some(ApiError 
                            {
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

    html! 
    {
        <div>
            <p>{ i18n.t("database.no_db_linked") }</p>
            if let Some(err) = &*error 
            {
                <p class="error">{ translate_error(err, &i18n) }</p>
            }
            <button class="button-primary" onclick={on_create_and_link} disabled={*is_loading}>
                { i18n.t("database.create_and_link_button") }
            </button>
        </div>
    }
}
