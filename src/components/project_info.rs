use i18nrs::yew::use_translation;
use yew::prelude::*;

use crate::contexts::sse_context::use_sse_status;
use crate::models::project::ProjectDetails;

use crate::pages::project_dashboard::{get_status_class, translate_status};

#[derive(Properties, PartialEq)]
pub struct ProjectInfoProps
{
    pub project_details: ProjectDetails,
}

#[function_component(ProjectInfo)]
pub fn project_info(props: &ProjectInfoProps) -> Html
{
    let (i18n, _) = use_translation();
    let current_status = use_sse_status();
    let p = &props.project_details.project;

    let created_at_formatted = p.created_at.split('T').next().unwrap_or("").to_string();
    let created_on_message = i18n
        .t("common.created_on")
        .replace("{date}", &created_at_formatted);

    let project_url = format!("https://{}.hangar.garageisep.com", p.name);
    
    let (status_class, status_text) = if let Some(status) = &current_status
    {
        (
            get_status_class(status),
            translate_status(status, &i18n),
        )
    }
    else
    {
        ("status_unknown", i18n.t("common.loading"))
    };

    html!
    {
        <div class="card">
            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: var(--spacing-md); margin-bottom: var(--spacing-md);">
                <h2 style="margin-bottom: 0;">{ i18n.t("project_dashboard.card_title_info") }</h2>
                <a href={project_url} target="_blank" rel="noopener noreferrer" class="button-primary">
                    { i18n.t("project_dashboard.visit_app_button") }
                </a>
            </div>
            <p>
                { i18n.t("common.status") }{ ": " }
                <span class={classes!("status-badge", status_class)}>
                    { status_text }
                </span>
            </p>
            <p>{ format!("{}: {}", i18n.t("common.owner"), p.owner) }</p>

            if !props.project_details.participants.is_empty()
            {
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

            if let Some(branch) = &p.source_branch
            {
                <p>{ format!("Branche GitHub: {}", branch) }</p>
            }

            if let Some(root_dir) = &p.source_root_dir
            {
                <p>{ format!("Dossier Racine: {}", root_dir) }</p>
            }

            <p style="word-break: break-all;">
                { format!("{}: {}", i18n.t("common.deployed_image"), p.deployed_image_tag) }
            </p>

            if let Some(path) = &p.persistent_volume_path
            {
                <p>
                    { format!("{}: {}", i18n.t("project_dashboard.persistent_volume_label"), path) }
                </p>
            }
        </div>
    }
}