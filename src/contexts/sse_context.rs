use std::rc::Rc;
use yew::prelude::*;

use crate::hooks::use_sse::use_sse_project;
use crate::models::project::ProjectMetrics;
use crate::services::sse_service::{ContainerStatus, DeploymentStage, SseEvent, SystemEvent};

#[derive(Clone, PartialEq)]
pub struct SseStatusContext
{
    pub status: Option<ContainerStatus>,
}

#[derive(Clone, PartialEq)]
pub struct SseMetricsContext
{
    pub metrics: Option<ProjectMetrics>,
}

#[derive(Clone, PartialEq)]
pub struct SseDeploymentContext
{
    pub stage: Option<DeploymentStage>,
}

#[derive(Clone, PartialEq)]
pub struct SseSystemContext
{
    pub events: Vec<SystemEvent>,
}

#[derive(Properties, PartialEq)]
pub struct SseProviderProps
{
    pub project_id: i32,
    pub children: Children,
}

#[function_component(SseProvider)]
pub fn sse_provider(props: &SseProviderProps) -> Html
{
    let sse_state = use_sse_project(props.project_id);
    
    let current_status = use_state(|| None::<ContainerStatus>);
    let current_metrics = use_state(|| None::<ProjectMetrics>);
    let deployment_stage = use_state(|| None::<DeploymentStage>);
    let system_events = use_state(Vec::<SystemEvent>::new);

    {
        let current_status = current_status.clone();
        let current_metrics = current_metrics.clone();
        let deployment_stage = deployment_stage.clone();
        let system_events = system_events.clone();

        use_effect_with(sse_state.events.clone(), move |events|
        {
            for event in events.iter()
            {
                match event
                {
                    SseEvent::ContainerStatus(status_event) =>
                    {
                        current_status.set(Some(status_event.status.clone()));
                    }
                    SseEvent::Metrics(metrics_event) =>
                    {
                        current_metrics.set(Some(metrics_event.metrics.clone()));
                    }
                    SseEvent::Deployment(deploy_event) =>
                    {
                        deployment_stage.set(Some(deploy_event.stage.clone()));
                    }
                    SseEvent::System(system_event) => 
                    { 
                        let mut events = (*system_events).clone();
                        events.push(system_event.clone());
                        
                        if events.len() > 5
                        {
                            events.remove(0);
                        }
                        
                        system_events.set(events);
                    }
                }
            }
            || ()
        });
    }

    let status_context = SseStatusContext
    {
        status: (*current_status).clone(),
    };

    let metrics_context = SseMetricsContext
    {
        metrics: (*current_metrics).clone(),
    };

    let deployment_context = SseDeploymentContext
    {
        stage: (*deployment_stage).clone(),
    };

    let system_context = SseSystemContext
    {
        events: (*system_events).clone(),
    };

    html!
    {
        <ContextProvider<Rc<SseStatusContext>> context={Rc::new(status_context)}>
            <ContextProvider<Rc<SseMetricsContext>> context={Rc::new(metrics_context)}>
                <ContextProvider<Rc<SseDeploymentContext>> context={Rc::new(deployment_context)}>
                    <ContextProvider<Rc<SseSystemContext>> context={Rc::new(system_context)}>
                        { for props.children.iter() }
                    </ContextProvider<Rc<SseSystemContext>>>
                </ContextProvider<Rc<SseDeploymentContext>>>
            </ContextProvider<Rc<SseMetricsContext>>>
        </ContextProvider<Rc<SseStatusContext>>>
    }
}

#[hook]
pub fn use_sse_status() -> Option<ContainerStatus>
{
    use_context::<Rc<SseStatusContext>>()
        .expect("SseStatusContext not found")
        .status
        .clone()
}

#[hook]
pub fn use_sse_metrics() -> Option<ProjectMetrics>
{
    use_context::<Rc<SseMetricsContext>>()
        .expect("SseMetricsContext not found")
        .metrics
        .clone()
}

#[hook]
pub fn use_sse_deployment() -> Option<DeploymentStage>
{
    use_context::<Rc<SseDeploymentContext>>()
        .expect("SseDeploymentContext not found")
        .stage
        .clone()
}

#[hook]
pub fn use_sse_system_events() -> Vec<SystemEvent>
{
    use_context::<Rc<SseSystemContext>>()
        .expect("SseSystemContext not found")
        .events
        .clone()
}