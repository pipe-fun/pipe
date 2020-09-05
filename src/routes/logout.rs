use yew::{
    ComponentLink,
    Component,
    Html,
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use crate::error::Error;
use crate::services::auth::Auth;
use crate::routes::AppRoute;

pub struct Logout {
    auth: Auth,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Prop,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Response(Result<(), Error>),
    Ignore,
}

#[derive(PartialEq, Properties, Clone, Default)]
pub struct Prop {
    pub callback: Callback<()>,
}

impl Component for Logout {
    type Message = Msg;
    type Properties = Prop;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            auth: Auth::new(),
            task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Ignore => (),
            Msg::Response(_) => {
                self.props.callback.emit(());
                self.router_agent.send(ChangeRoute(AppRoute::Login.into()))
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <></>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(self.auth.logout(self.link.callback(Msg::Response)));
        }
    }
}