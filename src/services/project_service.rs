use crate::models::database::{AdminDatabaseInfo, AdminDatabasesResponse};
use crate::models::project::{
    DeployPayload, DownProjectInfo, DownProjectsResponse, GlobalMetrics, Project, ProjectDetails, ProjectDetailsResponse, ProjectsResponse, UpdateEnvPayload
};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_ROOT: &str = "/api";

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct ApiError 
{
    pub error_code: String,
    pub details: Option<String>,
}
#[derive(Deserialize)]
pub struct LogsResponse 
{
    pub logs: String,
}

#[derive(Serialize)]
struct UpdateImagePayload 
{
    new_image_url: String,
}

#[derive(Serialize)]
struct ParticipantPayload 
{
    participant_id: String,
}

async fn parse_simple_error_response(response: gloo_net::http::Response) -> String 
{
    #[derive(Deserialize)]
    struct SimpleErrorResponse 
    {
        error_code: String,
    }

    if let Ok(error_body) = response.json::<SimpleErrorResponse>().await 
    {
        error_body.error_code
    } 
    else 
    {
        format!("HTTP_ERROR_{}", response.status())
    }
}

pub async fn parse_detailed_error_response(response: gloo_net::http::Response) -> ApiError 
{
    if let Ok(error_body) = response.json::<ApiError>().await 
    {
        error_body
    } 
    else 
    {
        ApiError 
        {
            error_code: format!("HTTP_ERROR_{}", response.status()),
            details: None,
        }
    }
}

pub async fn get_owned_projects() -> Result<Vec<Project>, String> 
{
    let response = Request::get(&format!("{}/projects/owned", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }

    response
        .json::<ProjectsResponse>()
        .await
        .map(|r| r.projects)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_participating_projects() -> Result<Vec<Project>, String> 
{
    let response = Request::get(&format!("{}/projects/participations", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }

    response
        .json::<ProjectsResponse>()
        .await
        .map(|r| r.projects)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn deploy_project(payload: DeployPayload) -> Result<ProjectDetails, ApiError> 
{
    let response = Request::post(&format!("{}/projects/deploy", API_ROOT))
        .json(&payload)
        .map_err(|_| ApiError 
        {
            error_code: "CLIENT_SERIALIZATION_ERROR".to_string(),
            details: None,
        })?
        .send()
        .await
        .map_err(|e| ApiError 
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;

    if !response.ok() 
    {
        return Err(parse_detailed_error_response(response).await);
    }

    response
        .json::<ProjectDetailsResponse>()
        .await
        .map(|pr| pr.project)
        .map_err(|e| ApiError 
        {
            error_code: "RESPONSE_PARSE_ERROR".to_string(),
            details: Some(e.to_string()),
        })
}

pub async fn purge_project(project_id: i32) -> Result<(), String> 
{
    let response = Request::delete(&format!("{}/projects/{}", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }

    Ok(())
}

pub async fn get_project_details(project_id: i32) -> Result<ProjectDetails, String> 
{
    let response = Request::get(&format!("{}/projects/{}", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }

    response
        .json::<ProjectDetailsResponse>()
        .await
        .map(|r| r.project)
        .map_err(|e| format!("Failed to parse response: {}", e))
}


pub async fn start_project(project_id: i32) -> Result<(), String> 
{
    let response = Request::post(&format!("{}/projects/{}/start", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }
    Ok(())
}

pub async fn stop_project(project_id: i32) -> Result<(), String> 
{
    let response = Request::post(&format!("{}/projects/{}/stop", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }
    Ok(())
}

pub async fn restart_project(project_id: i32) -> Result<(), String> 
{
    let response = Request::post(&format!("{}/projects/{}/restart", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }
    Ok(())
}

pub async fn get_project_logs(project_id: i32) -> Result<String, String> 
{
    let response = Request::get(&format!("{}/projects/{}/logs", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }

    response
        .json::<LogsResponse>()
        .await
        .map(|r| r.logs)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn update_project_image(project_id: i32, new_image_url: &str) -> Result<(), ApiError> 
{
    let payload = UpdateImagePayload 
    {
        new_image_url: new_image_url.to_string(),
    };

    let response = Request::put(&format!("{}/projects/{}/image", API_ROOT, project_id))
        .json(&payload)
        .map_err(|_| ApiError 
        {
            error_code: "CLIENT_SERIALIZATION_ERROR".to_string(),
            details: None,
        })?
        .send()
        .await
        .map_err(|e| ApiError 
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;

    if !response.ok() 
    {
        return Err(parse_detailed_error_response(response).await);
    }

    Ok(())
}

pub async fn rebuild_project(project_id: i32) -> Result<(), ApiError>
{
    let response = Request::put(&format!("{}/projects/{}/rebuild", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| ApiError 
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;

    if !response.ok()
    {
        return Err(parse_detailed_error_response(response).await);
    }

    Ok(())
}

pub async fn add_participant(project_id: i32, participant_id: &str) -> Result<(), ApiError> 
{
    let payload = ParticipantPayload 
    {
        participant_id: participant_id.to_string(),
    };
    let response = Request::post(&format!("{}/projects/{}/participants", API_ROOT, project_id))
        .json(&payload)
        .map_err(|_| ApiError 
        {
            error_code: "CLIENT_ERROR".to_string(),
            details: None,
        })?
        .send()
        .await
        .map_err(|e| ApiError 
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;

    if !response.ok() 
    {
        return Err(parse_detailed_error_response(response).await);
    }
    Ok(())
}

pub async fn remove_participant(project_id: i32, participant_id: &str) -> Result<(), String> 
{
    let response = Request::delete(&format!(
        "{}/projects/{}/participants/{}",
        API_ROOT, project_id, participant_id
    ))
    .send()
    .await
    .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }
    Ok(())
}

pub async fn get_all_projects_admin() -> Result<Vec<Project>, String> 
{
    let response = Request::get(&format!("{}/admin/projects", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }

    response
        .json::<ProjectsResponse>()
        .await
        .map(|r| r.projects)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_all_databases_admin() -> Result<Vec<AdminDatabaseInfo>, String>
{
    let response = Request::get(&format!("{}/admin/databases", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok()
    {
        return Err(parse_simple_error_response(response).await);
    }

    response
        .json::<AdminDatabasesResponse>()
        .await
        .map(|r| r.databases)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_global_metrics_admin() -> Result<GlobalMetrics, String>
{
    Request::get(&format!("{}/admin/metrics", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json::<GlobalMetrics>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_down_projects_admin() -> Result<Vec<DownProjectInfo>, String> 
{
    Request::get(&format!("{}/admin/projects/down", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json::<DownProjectsResponse>()
        .await
        .map(|r| r.down_projects)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn update_env_vars(project_id: i32, payload: &UpdateEnvPayload) -> Result<(), ApiError>
{
    let response = Request::put(&format!("{}/projects/{}/env", API_ROOT, project_id))
        .json(payload)
        .map_err(|_| ApiError 
        {
            error_code: "CLIENT_ERROR".to_string(),
            details: None,
        })?
        .send()
        .await
        .map_err(|e| ApiError 
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;

    if !response.ok()
    {
        return Err(parse_detailed_error_response(response).await);
    }

    Ok(())
}