use crate::{components::protected_route::{AdminRoute, ProtectedRoute}, pages::{self, admin, create_project, database_dashboard, project_dashboard}};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute 
{
    #[at("/")]
    Home,
    #[at("/auth/callback")]
    AuthCallback,
    #[at("/projects/create")]
    CreateProject,
    #[at("/projects/:id")]
    ProjectDashboard { id: i32 },
    #[at("/databases/:id")]
    DatabaseDashboard { id: i32 },
    #[at("/admin")]
    Admin,
    #[at("/about")]
    About,
    #[at("/terms")]
    Terms,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html 
{
    match route 
    {
        AppRoute::Home => html! { <pages::home::Home /> },
        AppRoute::AuthCallback => html! { <pages::auth_callback::AuthCallback /> },
        AppRoute::CreateProject => html! 
        {
            <ProtectedRoute>
                <create_project::CreateProject />
            </ProtectedRoute>
        },
        AppRoute::ProjectDashboard { id } => html! 
        {
            <ProtectedRoute>
                <project_dashboard::ProjectDashboard project_id={id} />
            </ProtectedRoute>
        },
        AppRoute::DatabaseDashboard { id } => html!
        {
            <ProtectedRoute>
                <database_dashboard::DatabaseDashboard db_id={id} />
            </ProtectedRoute>
        },
        AppRoute::Admin => html!
        {
            <AdminRoute>
                <admin::Admin />
            </AdminRoute>
        },
        AppRoute::About => html! { <pages::about::About /> },
        AppRoute::Terms => html! { <pages::terms::Terms /> },
        AppRoute::Contact => html! { <pages::contact::Contact /> },
        AppRoute::NotFound => html! { <pages::not_found::NotFound /> },
    }
}
