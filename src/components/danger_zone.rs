use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::AppRoute;
use crate::services::project_service;

#[derive(Properties, PartialEq)]
pub struct DangerZoneProps
{
    pub project_id: i32,
    pub project_name: String,
    pub has_linked_database: bool,
}

#[function_component(DangerZone)]
pub fn danger_zone(props: &DangerZoneProps) -> Html
{
    let (i18n, _) = use_translation();
    let navigator = use_navigator().unwrap();
    let deletion_error = use_state(|| None::<String>);

    let on_delete = 
    {
        let project_name = props.project_name.clone();
        let has_linked_db = props.has_linked_database;
        let project_id = props.project_id;
        let navigator = navigator.clone();
        let i18n = i18n.clone();
        let deletion_error = deletion_error.clone();

        Callback::from(move |_| 
        {
            let mut confirm_message = i18n
                .t("project_dashboard.confirm_delete")
                .replace("{name}", &project_name);

            if has_linked_db
            {
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

                wasm_bindgen_futures::spawn_local(async move 
                {
                    if project_service::purge_project(project_id).await.is_ok()
                    {
                        navigator.push(&AppRoute::Home);
                    }
                    else
                    {
                        deletion_error.set(Some(i18n.t("errors.DELETE_FAILED")));
                    }
                });
            }
        })
    };

    html! 
    {
        <div class="card" style="margin-top: var(--spacing-lg); border-color: var(--color-danger);">
            <h2>{ i18n.t("project_dashboard.card_title_danger") }</h2>

            if let Some(error_msg) = &*deletion_error 
            {
                <p class="error" style="margin-top: var(--spacing-md)">{ error_msg }</p>
            }

            <button class="button-danger" onclick={on_delete}>
                { i18n.t("project_dashboard.delete_button") }
            </button>
        </div>
    }
}
