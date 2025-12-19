use std::collections::{HashMap, HashSet};

use crate::
{
    contexts::user_context::use_user,
    models::project::DeployPayload,
    router::AppRoute,
    services::
    {
        database_service,
        project_service::{self, ApiError},
    },
};
use i18nrs::yew::use_translation;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum DeployMethod
{
    GitHub,
    Direct,
    Database,
}

const GITHUB_APP_NAME: &str = "hangar-app";

fn handle_change_textarea(state: UseStateHandle<String>) -> Callback<Event>
{
    Callback::from(move |e: Event|
    {
        let value = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value();
        state.set(value);
    })
}


#[function_component(CreateProject)]
pub fn create_project() -> Html
{
    let (i18n, _) = use_translation();
    let user_context = use_user();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();

    let project_name = use_state(String::new);
    let participants_str = use_state(String::new);
    let github_repo_url = use_state(String::new);
    let github_branch = use_state(String::new);
    let github_root_dir = use_state(String::new);
    let image_url = use_state(String::new);
    let env_vars_str = use_state(String::new);
    let volume_path_str = use_state(String::new);
    let create_db_with_project = use_state(|| false);

    let active_method = use_state(|| DeployMethod::GitHub);
    let is_loading = use_state(|| false);
    let error = use_state(|| None::<ApiError>);
    let show_success_banner = use_state(|| false);

    {
        let show_success_banner = show_success_banner.clone();
        use_effect_with((), move |_|
        {
            if location.query_str().contains("github_connected=true")
            {
                show_success_banner.set(true);
            }
            || ()
        });
    }

    let on_submit =
    {
        let project_name = project_name.clone();
        let participants_str = participants_str.clone();
        let github_repo_url = github_repo_url.clone();
        let github_branch = github_branch.clone();
        let github_root_dir = github_root_dir.clone();
        let image_url = image_url.clone();
        let active_method = active_method.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        let user_login = user_context.user.as_ref().map(|u| u.login.clone());
        let env_vars_str = env_vars_str.clone();
        let volume_path_str = volume_path_str.clone();
        let create_db_with_project = create_db_with_project.clone();

        Callback::from(move |e: SubmitEvent|
        {
            e.prevent_default();
            is_loading.set(true);
            error.set(None);

            let project_name = project_name.clone();
            let participants_str = participants_str.clone();
            let github_repo_url = github_repo_url.clone();
            let github_branch = github_branch.clone();
            let github_root_dir = github_root_dir.clone();
            let image_url = image_url.clone();
            let active_method = active_method.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let navigator = navigator.clone();
            let user_login = user_login.clone();
            let env_vars_str = env_vars_str.clone();
            let volume_path_str = volume_path_str.clone();
            let create_db_with_project = create_db_with_project.clone();

            wasm_bindgen_futures::spawn_local(async move
            {
                if *active_method == DeployMethod::Database
                {
                    match database_service::create_database().await
                    {
                        Ok(db) => navigator.push(&AppRoute::DatabaseDashboard { id: db.id }),
                        Err(e) => error.set(Some(e)),
                    }
                    is_loading.set(false);
                    return;
                }

                let participants_set: HashSet<String> = (*participants_str)
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if let Some(login) = &user_login
                {
                    if participants_set.contains(login)
                    {
                        error.set(Some(ApiError
                        {
                            error_code: "OWNER_CANNOT_BE_PARTICIPANT".to_string(),
                            details: None,
                        }));
                        is_loading.set(false);
                        return;
                    }
                }
                let participants: Vec<String> = participants_set.into_iter().collect();

                let env_vars: HashMap<String, String> = (*env_vars_str)
                    .lines()
                    .filter_map(|line| line.trim().split_once('='))
                    .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                    .filter(|(k, _)| !k.is_empty())
                    .collect();

                let mut payload = DeployPayload
                {
                    project_name: (*project_name).clone(),
                    participants,
                    env_vars: if env_vars.is_empty() { None } else { Some(env_vars) },
                    persistent_volume_path: if (*volume_path_str).trim().is_empty() { None } else { Some((*volume_path_str).trim().to_string()) },
                    create_database: if *create_db_with_project { Some(true) } else { None },
                    ..Default::default()
                };

                match *active_method
                {
                    DeployMethod::GitHub =>
                    {
                        payload.github_repo_url = Some((*github_repo_url).clone());
                        payload.github_branch = if (*github_branch).trim().is_empty() { None } else { Some((*github_branch).trim().to_string()) };
                        payload.github_root_dir = if (*github_root_dir).trim().is_empty() { None } else { Some((*github_root_dir).trim().to_string()) };
                    }
                    DeployMethod::Direct =>
                    {
                        payload.image_url = Some((*image_url).clone());
                    }
                    DeployMethod::Database => {}
                }

                let result = project_service::deploy_project(payload).await;
                is_loading.set(false);

                match result
                {
                    Ok(project) =>
                    {
                        navigator.push(&AppRoute::ProjectDashboard { id: project.project.id });
                    }
                    Err(api_error) =>
                    {
                        error.set(Some(api_error));
                    }
                }
            });
        })
    };

    let handle_change = |state: UseStateHandle<String>|
    {
        Callback::from(move |e: Event|
        {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            state.set(value);
        })
    };
    
    let handle_checkbox_change = 
    {
        let create_db_with_project = create_db_with_project.clone();
        Callback::from(move |e: Event| 
        {
            let checked = e.target_unchecked_into::<web_sys::HtmlInputElement>().checked();
            create_db_with_project.set(checked);
        })
    };

    let select_method = |method: DeployMethod|
    {
        let active_method = active_method.clone();
        Callback::from(move |_|
        {
            active_method.set(method);
        })
    };

    let on_close_banner =
    {
        let show_success_banner = show_success_banner.clone();
        Callback::from(move |_|
        {
            show_success_banner.set(false);
        })
    };

    let render_error = |err: &ApiError|
    {
        let error_key = format!("errors.{}", err.error_code);
        let main_message = i18n.t(&error_key);
        let display_message =
            if main_message.starts_with("Key '") && main_message.contains(" not found for language ")
            {
                i18n.t("errors.DEFAULT")
            }
            else
            {
                main_message
            };

        html!
        {
            <div class="error">
                <p>{ display_message }</p>
                {
                    if err.error_code == "GITHUB_ACCOUNT_NOT_LINKED"
                    {
                        let github_app_install_url = format!(
                            "https://github.com/apps/{}/installations/new",
                            GITHUB_APP_NAME
                        );
                        html!
                        {
                            <div style="margin-top: var(--spacing-md)">
                                <p>{ i18n.t("create_project.link_github_prompt") }</p>
                                <a href={github_app_install_url} target="_blank" class="button-primary">
                                    { i18n.t("create_project.link_github_button") }
                                </a>
                            </div>
                        }
                    }
                    else if err.error_code == "GITHUB_REPO_NOT_ACCESSIBLE"
                    {
                        let github_installations_url = "https://github.com/settings/installations";
                        html!
                        {
                             <div style="margin-top: var(--spacing-md)">
                                <a href={github_installations_url} target="_blank" class="button-primary">
                                    { i18n.t("create_project.link_github_button") }
                                </a>
                            </div>
                        }
                    }
                    else if err.error_code == "IMAGE_SCAN_FAILED"
                    {
                        if let Some(details) = &err.details
                        {
                            html!
                            {
                                <div class="error-details-box">
                                    <strong>{ "Grype Security Report:" }</strong>
                                    <pre><code>{ details.clone() }</code></pre>
                                </div>
                            }
                        } else { html! {} }
                    }
                    else { html! {} }
                }
            </div>
        }
    };

    let tab_class = |method: DeployMethod|
    {
        if *active_method == method
        {
            "tab active"
        }
        else
        {
            "tab"
        }
    };

    html!
    {
        <div class="create-project-page" style="max-width: 700px; margin: auto;">

            if *show_success_banner
            {
                <div class="success-banner">
                    <p>{ i18n.t("create_project.github_connected_success") }</p>
                    <button onclick={on_close_banner}>{"✖"}</button>
                </div>
            }

            <h1>{ i18n.t("create_project.title") }</h1>
            <p>{ i18n.t("create_project.documentation")}</p>

            <div class="tabs-container">
                <button class={tab_class(DeployMethod::GitHub)} onclick={select_method(DeployMethod::GitHub)}>
                    { i18n.t("create_project.github_tab") }
                </button>
                <button class={tab_class(DeployMethod::Direct)} onclick={select_method(DeployMethod::Direct)}>
                    { i18n.t("create_project.direct_tab") }
                </button>
                <button class={tab_class(DeployMethod::Database)} onclick={select_method(DeployMethod::Database)}>
                    { i18n.t("create_project.database_tab") }
                </button>
            </div>

            <form onsubmit={on_submit} class="card" style="border-top-left-radius: 0;">
                <p style="color: var(--color-text-secondary); margin-bottom: var(--spacing-xl);">
                {
                    match *active_method
                    {
                        DeployMethod::GitHub => i18n.t("create_project.description_github"),
                        DeployMethod::Direct => i18n.t("create_project.description_direct"),
                        DeployMethod::Database => i18n.t("create_project.description_database"),
                    }
                }
                </p>

                {
                    if *active_method != DeployMethod::Database
                    {
                        html!
                        {
                            <>
                                <div class="form-group">
                                    <label for="project_name">{ i18n.t("create_project.name_label") }</label>
                                    <input type="text" id="project_name" class="text-input"
                                        placeholder={i18n.t("create_project.name_placeholder")}
                                        value={(*project_name).clone()}
                                        onchange={handle_change(project_name.clone())}
                                        required=true />
                                    <small style="color: var(--color-text-secondary)">{ i18n.t("create_project.name_help") }</small>
                                </div>

                                {
                                    if *active_method == DeployMethod::GitHub
                                    {
                                        html!
                                        {
                                            <>
                                                <div class="form-group">
                                                    <label for="github_repo_url">{ i18n.t("create_project.github_repo_url_label") }</label>
                                                    <input type="text" id="github_repo_url" class="text-input"
                                                        placeholder={i18n.t("create_project.github_repo_url_placeholder")}
                                                        value={(*github_repo_url).clone()}
                                                        onchange={handle_change(github_repo_url.clone())}
                                                        required=true />
                                                </div>
                                                <div class="form-group">
                                                    <label for="github_branch">{ i18n.t("create_project.github_branch_label") }</label>
                                                    <input type="text" id="github_branch" class="text-input"
                                                        placeholder="main"
                                                        value={(*github_branch).clone()}
                                                        onchange={handle_change(github_branch.clone())} />
                                                    <small style="color: var(--color-text-secondary)">{ i18n.t("create_project.github_branch_help") }</small>
                                                </div>
                                                <div class="form-group">
                                                    <label for="github_root_dir">{ i18n.t("create_project.github_root_dir_label") }</label>
                                                    <input type="text" id="github_root_dir" class="text-input"
                                                        placeholder="/"
                                                        value={(*github_root_dir).clone()}
                                                        onchange={handle_change(github_root_dir.clone())} />
                                                    <small style="color: var(--color-text-secondary)">{ i18n.t("create_project.github_root_dir_help") }</small>
                                                </div>
                                            </>
                                        }
                                    }
                                    else
                                    {
                                        html!
                                        {
                                            <>
                                                <div class="form-group">
                                                    <label for="image_url">{ i18n.t("create_project.image_label") }</label>
                                                    <input type="text" id="image_url" class="text-input"
                                                        placeholder={i18n.t("create_project.image_placeholder")}
                                                        value={(*image_url).clone()}
                                                        onchange={handle_change(image_url.clone())}
                                                        required=true />
                                                </div>

                                                <div class="form-group">
                                                    <label for="volume_path">{ i18n.t("create_project.volume_path_label") }</label>
                                                    <input type="text" id="volume_path" class="text-input"
                                                        placeholder="/data/uploads"
                                                        value={(*volume_path_str).clone()}
                                                        onchange={handle_change(volume_path_str.clone())} />
                                                    <small style="color: var(--color-text-secondary)">
                                                        { i18n.t("create_project.volume_path_help") }
                                                    </small>
                                                </div>
                                            </>
                                        }
                                    }
                                }

                                <div class="form-group">
                                    <label for="participants">{ i18n.t("create_project.participants_label") }</label>
                                    <input type="text" id="participants" class="text-input"
                                        placeholder={i18n.t("create_project.participants_placeholder")}
                                        value={(*participants_str).clone()}
                                        onchange={handle_change(participants_str.clone())} />
                                    <small style="color: var(--color-text-secondary)">{ i18n.t("create_project.participants_help") }</small>
                                </div>

                                <div class="form-group">
                                    <label for="env_vars">{ i18n.t("create_project.env_vars_label") }</label>
                                    <textarea id="env_vars" class="text-input"
                                        placeholder="API_KEY=your_secret_key"
                                        value={(*env_vars_str).clone()}
                                        onchange={handle_change_textarea(env_vars_str.clone())}
                                        rows="4">
                                    </textarea>
                                    <small style="color: var(--color-text-secondary)">{ i18n.t("create_project.env_vars_help") }</small>
                                </div>
                                
                                <div class="form-group">
                                    <label class="checkbox-label" for="create_db" style="display: flex; align-items: center; gap: var(--spacing-sm);">
                                        <input type="checkbox" id="create_db"
                                            checked={*create_db_with_project}
                                            onchange={handle_checkbox_change}
                                        />
                                        { i18n.t("create_project.create_db_checkbox") }
                                    </label>
                                </div>
                            </>
                        }
                    }
                    else
                    {
                        html!{}
                    }
                }

                { if let Some(err) = &*error { render_error(err) } else { html! {} } }

                <button type="submit" class="button-primary" disabled={*is_loading}>
                {
                    if *is_loading
                    {
                        i18n.t("create_project.submit_button_loading")
                    }
                    else if *active_method == DeployMethod::Database
                    {
                         i18n.t("database.create_button")
                    }
                    else
                    {
                        i18n.t("create_project.submit_button")
                    }
                }
                </button>
            </form>
        </div>
    }
}