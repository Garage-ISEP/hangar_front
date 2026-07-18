use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use crate::models::project::ProjectMetrics;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SseEvent
{
    Deployment(DeploymentEvent),
    ContainerStatus(ContainerStatusEvent),
    Metrics(MetricsEvent),
    System(SystemEvent),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DeploymentEvent
{
    pub project_id: i32,
    pub project_name: String,
    pub stage: DeploymentStage,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentStage
{
    Started,
    ValidatingInput,
    PullingImage
    {
        image_url: String,
    },
    ImagePulled,
    ScanningImage,
    ImageScanned,
    CloningRepository
    {
        repo_url: String,
    },
    RepositoryCloned,
    BuildingImage,
    ImageBuilt,
    GettingImageDigest,
    CreatingContainer,
    ContainerCreated,
    WaitingHealthCheck,
    HealthCheckPassed,
    ProvisioningDatabase,
    DatabaseProvisioned,
    LinkingDatabase,
    DatabaseLinked,
    CleaningUp,
    Completed
    {
        container_name: String,
    },
    Failed
    {
        error: String,
        stage: String,
    },
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ContainerStatusEvent
{
    pub project_id: i32,
    pub project_name: String,
    pub container_name: String,
    pub status: ContainerStatus,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus
{
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Stopping,
    Exited,
    Dead,
    Unknown,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MetricsEvent
{
    pub project_id: i32,
    pub project_name: String,
    pub metrics: ProjectMetrics,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SystemEvent
{
    pub level: SystemEventLevel,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SystemEventLevel
{
    Info,
    Warning,
    Error,
}

pub struct SseManager
{
    _phantom: std::marker::PhantomData<()>,
}

impl SseManager
{
    fn subscribe_to_event(
        event_source: &mut EventSource,
        event_type: &str,
        on_message: Callback<SseEvent>,
        on_error: Callback<String>,
    )
    {
        let event_type_owned = event_type.to_string();

        match event_source.subscribe(event_type)
        {
            Ok(mut subscription) =>
            {
                spawn_local(async move 
                {
                    while let Some(message_result) = subscription.next().await
                    {
                        match message_result
                        {
                            Ok((_event_type, message)) =>
                            {
                                if let Some(data_str) = message.data().as_string()
                                {
                                    match serde_json::from_str::<SseEvent>(&data_str)
                                    {
                                        Ok(event) => on_message.emit(event),
                                        Err(e) =>
                                        {
                                            gloo_console::error!(
                                                "Failed to parse SSE event",
                                                &event_type_owned,
                                                format!("{:?}", e)
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) =>
                            {
                                gloo_console::error!(
                                    "Error in SSE stream",
                                    &event_type_owned,
                                    format!("{:?}", e)
                                );
                                on_error.emit(format!("SSE stream error ({})", event_type_owned));
                                break;
                            }
                        }
                    }
                    gloo_console::log!("SSE subscription ended", &event_type_owned);
                });
            }
            Err(e) =>
            {
                gloo_console::error!("Failed to subscribe", event_type, format!("{:?}", e));
            }
        }
    }
}


pub fn connect_sse(
    url: &str,
    on_message: Callback<SseEvent>,
    on_error: Callback<String>,
) -> Result<EventSource, String>
{
    let event_source = gloo_net::eventsource::futures::EventSource::new(url)
        .map_err(|e| format!("Failed to create EventSource: {:?}", e))?;
    
    let mut event_source = event_source;

    let event_types = [
        "deployment",
        "container_status",
        "metrics",
        "system",
    ];

    for event_type in &event_types
    {
        SseManager::subscribe_to_event(&mut event_source, event_type, on_message.clone(), on_error.clone());
    }

    Ok(event_source)
}

pub fn connect_to_project(
    project_id: i32,
    on_message: Callback<SseEvent>,
    on_error: Callback<String>,
) -> Result<EventSource, String>
{
    let url = format!("/api/sse/projects/{}", project_id);
    connect_sse(&url, on_message, on_error)
}

pub fn connect_to_creation(
    on_message: Callback<SseEvent>,
    on_error: Callback<String>,
) -> Result<EventSource, String>
{
    connect_sse("/api/sse/creation", on_message, on_error)
}