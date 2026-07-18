use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DatabaseDetails
{
    pub id: i32,
    pub database_name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub project_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct DatabaseDetailsResponse
{
    pub database: DatabaseDetails,
}

#[derive(Deserialize)]
pub struct CreateDatabaseResponse
{
    pub database: DatabaseDetails,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdminDatabaseInfo
{
    pub id: i32,
    pub owner_login: String,
    pub database_name: String,
    pub username: String,
    pub project_id: Option<i32>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct AdminDatabasesResponse
{
    pub databases: Vec<AdminDatabaseInfo>,
}