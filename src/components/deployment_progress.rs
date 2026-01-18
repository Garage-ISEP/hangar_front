use yew::prelude::*;
use i18nrs::I18n;
use crate::services::sse_service::DeploymentStage;

#[derive(Properties, PartialEq)]
pub struct DeploymentProgressProps
{
    pub stage: DeploymentStage,
    #[prop_or_default]
    pub context: ProgressContext,
}

#[derive(PartialEq, Clone, Copy, Default)]
pub enum ProgressContext
{
    #[default]
    Creation,
    Update,
}

#[function_component(DeploymentProgress)]
pub fn deployment_progress(props: &DeploymentProgressProps) -> Html
{
    let i18n = use_context::<I18n>().expect("I18n context not found");
    
    let is_failed = matches!(&props.stage, DeploymentStage::Failed { .. });
    let is_completed = matches!(&props.stage, DeploymentStage::Completed { .. });
    
    let progress_percent = calculate_progress(&props.stage);
    let stage_key = get_stage_translation_key(&props.stage);
    let current_stage_label = i18n.t(stage_key);
    
    let status_class = if is_failed 
    {
        "deployment-status-failed"
    } 
    else if is_completed 
    {
        "deployment-status-success"
    } 
    else 
    {
        "deployment-status-active"
    };

    let title_key = match props.context
    {
        ProgressContext::Creation => "create_project.deployment_in_progress",
        ProgressContext::Update => "project_dashboard.deployment_updating",
    };

    let completion_message_key = match props.context
    {
        ProgressContext::Creation => "create_project.deployment_redirecting",
        ProgressContext::Update => "project_dashboard.deployment_complete",
    };

    html! 
    {
        <div class="deployment-progress-container" style="margin-bottom: var(--spacing-lg);">
            <div class="card" style="background-color: rgba(33, 150, 243, 0.1); border-color: #2196F3;">
                <div class="deployment-header">
                    <span class="deployment-title">
                        { i18n.t(title_key) }
                    </span>
                    <span class={classes!("deployment-status", status_class)}>
                        { current_stage_label.clone() }
                    </span>
                </div>

                <div class="deployment-progress-bar">
                    <div 
                        class={classes!("deployment-progress-fill", status_class)}
                        style={format!("width: {}%", progress_percent)}
                    >
                        <span class="deployment-progress-text">
                            { format!("{}%", progress_percent) }
                        </span>
                    </div>
                </div>

                {
                    if is_completed 
                    {
                        html! 
                        {
                            <div class="deployment-success-message">
                                { i18n.t(completion_message_key) }
                            </div>
                        }
                    }
                    else if is_failed 
                    {
                        if let DeploymentStage::Failed { error, .. } = &props.stage 
                        {
                            html! 
                            {
                                <div class="deployment-error-message">
                                    { error }
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
            </div>
        </div>
    }
}

fn get_stage_translation_key(stage: &DeploymentStage) -> &'static str
{
    match stage
    {
        DeploymentStage::Started => "create_project.deployment_stage_started",
        DeploymentStage::ValidatingInput => "create_project.deployment_stage_validating",
        DeploymentStage::PullingImage { .. } => "create_project.deployment_stage_pulling",
        DeploymentStage::ImagePulled => "create_project.deployment_stage_pulled",
        DeploymentStage::ScanningImage => "create_project.deployment_stage_scanning",
        DeploymentStage::ImageScanned => "create_project.deployment_stage_scanned",
        DeploymentStage::CloningRepository { .. } => "create_project.deployment_stage_cloning",
        DeploymentStage::RepositoryCloned => "create_project.deployment_stage_cloned",
        DeploymentStage::BuildingImage => "create_project.deployment_stage_building",
        DeploymentStage::ImageBuilt => "create_project.deployment_stage_built",
        DeploymentStage::GettingImageDigest => "create_project.deployment_stage_digest",
        DeploymentStage::CreatingContainer => "create_project.deployment_stage_creating",
        DeploymentStage::ContainerCreated => "create_project.deployment_stage_created",
        DeploymentStage::WaitingHealthCheck => "create_project.deployment_stage_health",
        DeploymentStage::HealthCheckPassed => "create_project.deployment_stage_healthy",
        DeploymentStage::ProvisioningDatabase => "create_project.deployment_stage_db_provision",
        DeploymentStage::DatabaseProvisioned => "create_project.deployment_stage_db_provisioned",
        DeploymentStage::LinkingDatabase => "create_project.deployment_stage_db_linking",
        DeploymentStage::DatabaseLinked => "create_project.deployment_stage_db_linked",
        DeploymentStage::CleaningUp => "create_project.deployment_stage_cleanup",
        DeploymentStage::Completed { .. } => "create_project.deployment_stage_completed",
        DeploymentStage::Failed { .. } => "create_project.deployment_stage_failed",
    }
}

fn calculate_progress(stage: &DeploymentStage) -> u8
{
    match stage
    {
        DeploymentStage::Started => 5,
        DeploymentStage::ValidatingInput => 10,
        DeploymentStage::PullingImage { .. } => 20,
        DeploymentStage::ImagePulled => 30,
        DeploymentStage::ScanningImage => 35,
        DeploymentStage::ImageScanned => 40,
        DeploymentStage::CloningRepository { .. } => 25,
        DeploymentStage::RepositoryCloned => 35,
        DeploymentStage::BuildingImage => 50,
        DeploymentStage::ImageBuilt => 60,
        DeploymentStage::GettingImageDigest => 65,
        DeploymentStage::CreatingContainer => 70,
        DeploymentStage::ContainerCreated => 80,
        DeploymentStage::WaitingHealthCheck => 85,
        DeploymentStage::HealthCheckPassed => 90,
        DeploymentStage::ProvisioningDatabase => 75,
        DeploymentStage::DatabaseProvisioned => 85,
        DeploymentStage::LinkingDatabase => 90,
        DeploymentStage::DatabaseLinked => 95,
        DeploymentStage::CleaningUp => 98,
        DeploymentStage::Completed { .. } => 100,
        DeploymentStage::Failed { .. } => 100,
    }
}