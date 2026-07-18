// src/pages/project_dashboard/components/image_update_form.rs

use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::components::deployment_progress::{DeploymentProgress, ProgressContext};
use crate::contexts::sse_context::use_sse_deployment;
use crate::models::project::ProjectSourceType;
use crate::services::project_service::{self, ApiError};
use crate::services::sse_service::DeploymentStage;

use crate::pages::project_dashboard::translate_error;

const PROGRESS_HIDE_DELAY_MS: u32 = 20000;

#[derive(Properties, PartialEq)]
pub struct ImageUpdateFormProps
{
    pub project_id: i32,
    pub project_name: String,
    pub source_type: ProjectSourceType,
    pub on_update: Callback<()>,
}

#[function_component(ImageUpdateForm)]
pub fn image_update_form(props: &ImageUpdateFormProps) -> Html
{
    let (i18n, _) = use_translation();
    let deployment_stage = use_sse_deployment();
    
    let new_image_url = use_state(String::new);
    let is_updating = use_state(|| false);
    let update_error = use_state(|| None::<ApiError>);
    let hide_progress = use_state(|| false);

    let is_github = props.source_type == ProjectSourceType::Github;

    // Gérer l'affichage/masquage du progress
    {
        let hide_progress = hide_progress.clone();
        let is_updating = is_updating.clone();

        use_effect_with(deployment_stage.clone(), move |stage|
        {
            match stage
            {
                Some(DeploymentStage::Started)
                | Some(DeploymentStage::ValidatingInput)
                | Some(DeploymentStage::PullingImage { .. })
                | Some(DeploymentStage::ImagePulled)
                | Some(DeploymentStage::ScanningImage)
                | Some(DeploymentStage::ImageScanned)
                | Some(DeploymentStage::CloningRepository { .. })
                | Some(DeploymentStage::RepositoryCloned)
                | Some(DeploymentStage::BuildingImage)
                | Some(DeploymentStage::ImageBuilt)
                | Some(DeploymentStage::GettingImageDigest)
                | Some(DeploymentStage::CreatingContainer)
                | Some(DeploymentStage::ContainerCreated)
                | Some(DeploymentStage::WaitingHealthCheck)
                | Some(DeploymentStage::HealthCheckPassed)
                | Some(DeploymentStage::ProvisioningDatabase)
                | Some(DeploymentStage::DatabaseProvisioned)
                | Some(DeploymentStage::LinkingDatabase)
                | Some(DeploymentStage::DatabaseLinked)
                | Some(DeploymentStage::CleaningUp) =>
                {
                    is_updating.set(true);
                    hide_progress.set(false);
                }
                Some(DeploymentStage::Completed { .. }) =>
                {
                    is_updating.set(false);
                    let hide_progress = hide_progress.clone();
                    gloo_timers::callback::Timeout::new(PROGRESS_HIDE_DELAY_MS, move ||
                    {
                        hide_progress.set(true);
                    })
                    .forget();
                }
                Some(DeploymentStage::Failed { .. }) =>
                {
                    is_updating.set(false);
                }
                None => {}
            }
            || ()
        });
    }

    let is_deploying = deployment_stage.is_some()
        && !matches!(&deployment_stage, Some(DeploymentStage::Failed { .. }));

    let on_input_change =
    {
        let new_image_url = new_image_url.clone();
        Callback::from(move |e: Event|
        {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            new_image_url.set(value);
        })
    };

    let on_close_progress =
    {
        let hide_progress = hide_progress.clone();
        Callback::from(move |_|
        {
            hide_progress.set(true);
        })
    };

    let on_submit =
    {
        let project_id = props.project_id;
        let project_name = props.project_name.clone();
        let new_image_url = new_image_url.clone();
        let is_updating = is_updating.clone();
        let update_error = update_error.clone();
        let hide_progress = hide_progress.clone();
        let i18n = i18n.clone();
        let on_update = props.on_update.clone();

        Callback::from(move |e: SubmitEvent|
        {
            e.prevent_default();

            let confirm_key = if is_github
            {
                "project_dashboard.confirm_rebuild"
            }
            else
            {
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
                let hide_progress = hide_progress.clone();
                let on_update = on_update.clone();

                is_updating.set(true);
                update_error.set(None);
                hide_progress.set(false);

                wasm_bindgen_futures::spawn_local(async move
                {
                    let result = if is_github
                    {
                        project_service::rebuild_project(project_id).await
                    }
                    else
                    {
                        project_service::update_project_image(project_id, &image_url).await
                    };

                    match result
                    {
                        Ok(_) =>
                        {
                            new_image_url.set(String::new());
                            is_updating.set(false);
                            on_update.emit(());
                        }
                        Err(api_error) =>
                        {
                            update_error.set(Some(api_error));
                            is_updating.set(false);
                        }
                    }
                });
            }
        })
    };

    let (title_key, description_key, button_key, button_loading_key) = if is_github
    {
        (
            "project_dashboard.card_title_rebuild",
            "project_dashboard.rebuild_description",
            "project_dashboard.rebuild_button",
            "project_dashboard.rebuild_button_loading",
        )
    }
    else
    {
        (
            "project_dashboard.card_title_update_image",
            "project_dashboard.update_image_description",
            "project_dashboard.update_image_button",
            "project_dashboard.update_image_button_loading",
        )
    };

    html!
    {
        <div class="card" style="margin-top: var(--spacing-lg);">
            <h2>{ i18n.t(title_key) }</h2>
            <p style="color: var(--color-text-secondary); margin-bottom: var(--spacing-md);">
                { i18n.t(description_key) }
            </p>

            {
                if let Some(stage) = &deployment_stage
                {
                    if !*hide_progress
                    {
                        html!
                        {
                            <div style="position: relative; margin-bottom: var(--spacing-md);">
                                <button
                                    onclick={on_close_progress}
                                    style="position: absolute; top: 8px; right: 8px; background: transparent; border: none; cursor: pointer; font-size: 20px; line-height: 1; padding: 4px 8px; color: var(--color-text-secondary); z-index: 10;"
                                    title="Fermer"
                                >
                                    { "×" }
                                </button>
                                <DeploymentProgress
                                    stage={stage.clone()}
                                    context={ProgressContext::Update}
                                />
                            </div>
                        }
                    }
                    else
                    {
                        html! {}
                    }
                }
                else
                {
                    html! {}
                }
            }

            <form onsubmit={on_submit}>
                if !is_github
                {
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
                            disabled={is_deploying}
                        />
                    </div>
                }

                if let Some(err) = &*update_error
                {
                    <p class="error">{ translate_error(err, &i18n) }</p>
                }

                <button type="submit" class="button-primary" disabled={*is_updating}>
                    {
                        if is_deploying
                        {
                            i18n.t(button_loading_key)
                        }
                        else
                        {
                            i18n.t(button_key)
                        }
                    }
                </button>
            </form>
        </div>
    }
}