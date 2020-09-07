pub mod login;
pub mod logout;
pub mod register;
pub mod active;
pub mod console;
pub mod from_js;

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/#/logout"]
    Logout,
    #[to = "/#/console"]
    Console,
    #[to = "/#/register"]
    Register,
    #[to = "/#/active"]
    Active,
    #[to = "/#"]
    Login,
}

/// Fix fragment handling problem for yew_router
pub fn fix_fragment_routes(route: &mut Route) {
    let r = route.route.as_str();
    if let None = r.find('#') {
        route.route = "/#".to_string();
    }
}
