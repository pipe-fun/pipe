use yew::{
    ComponentLink,
    Component,
    Html,
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use status_protoc::status::user::active::{ActiveStatus, _ActiveStatus};
use status_protoc::my_trait::StatusTrait;
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use crate::error::Error;
use crate::services::auth::Auth;
use crate::routes::AppRoute;
use crate::components::footer::Footer;
use crate::routes::from_js::{active_btn_disable, active_btn_enable};

pub struct Active {
    auth: Auth,
    code: String,
    task: Option<FetchTask>,
    response: Callback<Result<ActiveStatus, Error>>,
    tip: Html,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>
}

pub enum Msg {
    Request,
    Response(Result<ActiveStatus, Error>),
    UpdateCode(String),
    Ignore
}

impl Component for Active {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            auth: Auth::new(),
            code: String::default(),
            task: None,
            response: link.callback(Msg::Response),
            tip: html! { <p class="alert alert-info">{ "激活账户, 激活码请查看注册邮箱，激活成功将会跳转到登录界面" }</p> },
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(response)) => {
                match response.status() {
                    _ActiveStatus::InvalidCode => {
                        self.tip = html! { <p class="alert alert-danger">{ "激活码错误" }</p> };
                    }
                    _ActiveStatus::DbAPIError => {
                        self.tip = html! { <p class="alert alert-danger">{ "数据库错误，请联系管理员" }</p> };
                    }
                    _ActiveStatus::ActiveSuccessfully => {
                        self.router_agent.send(ChangeRoute(AppRoute::Login.into()))
                    }
                }
                active_btn_enable();
            },
            Msg::Request => {
                active_btn_disable();
                self.task = Some(self.auth.active(self.code.clone(), self.response.clone()));
            }
            Msg::UpdateCode(code) => self.code = code,
            Msg::Ignore => (),
            Msg::Response(Err(_)) => (),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
        });

        let oninput_code = self
            .link
            .callback(|ev: InputData| Msg::UpdateCode(ev.value));

        html! {
            <>
                <link href="register.css" rel="stylesheet" type="text/css"/>
                <div class="container">
                    <form onsubmit=onsubmit>
                        <h1>{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                        { self.tip.clone() }
                        <div class="form-group ">
                            <label class="control-label" for="code">{ "激活码" }</label>
                            <input type="text" class="form-control"
                            name="code"
                            id="code"
                            placeholder="请输入 激活码"
                            required=true
                            value=&self.code
                            oninput=oninput_code
                            />
                        </div>

                        <div class="text-right">
                            <button type="submit" id="active_btn" class="btn btn-default">{ "激活" }</button>
                        </div>
                    </form>
                </div>
                <Footer />
            </>
        }
    }
}
