pub mod login;
use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Login,
    #[to = "/logout"]
    Logout,
    #[to = "/console"]
    Console,
}