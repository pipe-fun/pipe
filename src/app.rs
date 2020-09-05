//! The root app contains initial authentication and url routes
use yew::{
    agent::Bridged,
    html,
    Bridge,
    Callback,
    Component,
    ComponentLink,
    Html,
    ShouldRender,
};

use crate::routes::{
    console::view::Console,
    login::Login,
    AppRoute,
    fix_fragment_routes,
};

use yew::services::fetch::FetchTask;
use yew_router::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
use log::debug;
use crate::services::auth::Auth;
use crate::types::auth::UserInfo;
use crate::error::Error;

/// The root app component
pub struct App {
    auth: Auth,
    current_route: Option<AppRoute>,
    current_user: Option<UserInfo>,
    current_user_response: Callback<Result<UserInfo, Error>>,
    current_user_task: Option<FetchTask>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    CurrentUserResponse(Result<UserInfo, Error>),
    Route(Route),
    Authenticated(UserInfo),
    Logout,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();
        fix_fragment_routes(&mut route);
        debug!("Route {:?}", route);

        App {
            auth: Auth::new(),
            current_route: AppRoute::switch(route),
            router_agent,
            current_user: None,
            current_user_response: link.callback(Msg::CurrentUserResponse),
            current_user_task: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let task = self.auth.authorize(self.current_user_response.clone());
            self.current_user_task = Some(task);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CurrentUserResponse(Ok(info)) => {
                if info.authorized {
                    self.current_user = Some(info);
                    self.current_user_task = None;
                }
                self.redirect();
            }
            Msg::CurrentUserResponse(Err(_)) => {
                self.current_user_task = None;
                self.redirect();
            }
            Msg::Route(mut route) => {
                fix_fragment_routes(&mut route);
                self.current_route = AppRoute::switch(route);
                self.redirect();
            }
            Msg::Authenticated(info) => {
                self.current_user = Some(info);
            }
            Msg::Logout => {
                self.current_user = None;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let callback_login = self.link.callback(Msg::Authenticated);

        html! {
            <>
                {
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login callback=callback_login />},
                            AppRoute::Console => html!{<Console />},
                            AppRoute::Logout => html!{<Console />},
                        }
                    } else {
                        html! { "No found" }
                    }
                }
            </>
        }
    }
}

impl App {
    fn redirect(&mut self) {
        if let Some(route) = &self.current_route {
            match route {
                AppRoute::Login if self.current_user.is_some() =>
                    self.router_agent.send(ChangeRoute(AppRoute::Console.into())),
                AppRoute::Console if self.current_user.is_none() =>
                    self.router_agent.send(ChangeRoute(AppRoute::Login.into())),
                AppRoute::Logout if self.current_user.is_none() =>
                    self.router_agent.send(ChangeRoute(AppRoute::Login.into())),
                AppRoute::Login => {}
                AppRoute::Console => {}
                AppRoute::Logout => {}
            }
        }
    }
}