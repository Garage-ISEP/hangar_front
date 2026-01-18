use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::services::project_service::{self, ApiError};

use crate::pages::project_dashboard::translate_error;

#[derive(Properties, PartialEq)]
pub struct ParticipantManagerProps
{
    pub project_id: i32,
    pub participants: Vec<String>,
    pub on_update: Callback<()>,
}

#[function_component(ParticipantManager)]
pub fn participant_manager(props: &ParticipantManagerProps) -> Html
{
    let (i18n, _) = use_translation();
    
    let new_participant = use_state(String::new);
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<ApiError>);

    let on_input_change =
    {
        let new_participant = new_participant.clone();
        Callback::from(move |e: Event|
        {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            new_participant.set(value);
        })
    };

    let on_add =
    {
        let is_loading = is_loading.clone();
        let error = error.clone();
        let new_participant = new_participant.clone();
        let project_id = props.project_id;
        let on_update = props.on_update.clone();

        Callback::from(move |e: SubmitEvent|
        {
            e.prevent_default();
            is_loading.set(true);
            error.set(None);

            let participant_id = (*new_participant).clone();
            let new_participant = new_participant.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let on_update = on_update.clone();

            wasm_bindgen_futures::spawn_local(async move
            {
                match project_service::add_participant(project_id, &participant_id).await
                {
                    Ok(_) =>
                    {
                        new_participant.set(String::new());
                        on_update.emit(());
                    }
                    Err(e) => error.set(Some(e)),
                }
                is_loading.set(false);
            });
        })
    };

    let render_participant = |p: &String|
    {
        let participant_id = p.clone();
        let on_remove =
        {
            let project_id = props.project_id;
            let on_update = props.on_update.clone();
            let i18n = i18n.clone();
            let participant_id = participant_id.clone();

            Callback::from(move |_|
            {
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

                    wasm_bindgen_futures::spawn_local(async move
                    {
                        if project_service::remove_participant(project_id, &participant_id)
                            .await
                            .is_ok()
                        {
                            on_update.emit(());
                        }
                        else
                        {
                            gloo_console::error!("Failed to remove participant");
                        }
                    });
                }
            })
        };

        html!
        {
            <li style="display: flex; justify-content: space-between; align-items: center; padding: var(--spacing-sm) 0; border-bottom: 1px solid var(--color-border);">
                <span>{ p }</span>
                <button class="button-danger" onclick={on_remove}>
                    { i18n.t("project_dashboard.remove_participant_button") }
                </button>
            </li>
        }
    };

    html!
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t("project_dashboard.manage_participants_title") }</h2>

            if props.participants.is_empty()
            {
                <p style="color: var(--color-text-secondary);">
                    { i18n.t("project_dashboard.no_participants") }
                </p>
            }
            else
            {
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

                if let Some(err) = &*error
                {
                    <p class="error">{ translate_error(err, &i18n) }</p>
                }

                <button type="submit" class="button-primary" disabled={*is_loading}>
                    {
                        if *is_loading
                        {
                            i18n.t("project_dashboard.add_participant_button_loading")
                        }
                        else
                        {
                            i18n.t("project_dashboard.add_participant_button")
                        }
                    }
                </button>
            </form>
        </div>
    }
}