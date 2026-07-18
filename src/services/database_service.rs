use crate::models::database::{CreateDatabaseResponse, DatabaseDetails, DatabaseDetailsResponse};
use crate::services::project_service::{parse_detailed_error_response, ApiError};
use gloo_net::http::Request;

const API_ROOT: &str = "/api";

pub async fn get_my_database() -> Result<DatabaseDetails, ApiError>
{
    let response = Request::get(&format!("{}/databases/mine", API_ROOT))
        .send()
        .await
        .map_err(|e| ApiError 
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;

    if response.status() == 404
    {
        return Err(ApiError 
        {
            error_code: "NOT_FOUND".to_string(),
            details: None,
        });
    }

    if !response.ok()
    {
        return Err(parse_detailed_error_response(response).await);
    }

    response
        .json::<DatabaseDetailsResponse>()
        .await
        .map(|r| r.database)
        .map_err(|e| ApiError {
            error_code: "RESPONSE_PARSE_ERROR".to_string(),
            details: Some(e.to_string()),
        })
}

pub async fn get_database(db_id: i32) -> Result<DatabaseDetails, ApiError>
{
    let response = Request::get(&format!("{}/databases/{}", API_ROOT, db_id))
        .send()
        .await
        .map_err(|e| ApiError
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;

    if response.status() == 404
    {
        return Err(ApiError
        {
            error_code: "NOT_FOUND".to_string(),
            details: None,
        });
    }

    if !response.ok()
    {
        return Err(parse_detailed_error_response(response).await);
    }

    response
        .json::<DatabaseDetailsResponse>()
        .await
        .map(|r| r.database)
        .map_err(|e| ApiError {
            error_code: "RESPONSE_PARSE_ERROR".to_string(),
            details: Some(e.to_string()),
        })
}

pub async fn create_database() -> Result<DatabaseDetails, ApiError>
{
    let response = Request::post(&format!("{}/databases", API_ROOT))
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
        .json::<CreateDatabaseResponse>()
        .await
        .map(|r| r.database)
        .map_err(|e| ApiError 
        {
            error_code: "RESPONSE_PARSE_ERROR".to_string(),
            details: Some(e.to_string()),
        })
}

pub async fn delete_database(db_id: i32) -> Result<(), ApiError>
{
    let response = Request::delete(&format!("{}/databases/{}", API_ROOT, db_id))
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

pub async fn link_database_to_project(project_id: i32, db_id: i32) -> Result<(), ApiError>
{
    let response = Request::put(&format!("{}/projects/{}/database/{}", API_ROOT, project_id, db_id))
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

pub async fn unlink_database_from_project(project_id: i32) -> Result<(), ApiError>
{
    let response = Request::delete(&format!("{}/projects/{}/database", API_ROOT, project_id))
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

pub async fn delete_linked_database(project_id: i32) -> Result<(), ApiError> 
{
    let response = Request::delete(&format!("{}/projects/{}/database/delete", API_ROOT, project_id))
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