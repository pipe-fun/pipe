use yew::{
    ComponentLink,
    Component,
    Html,
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use status_protoc::status::user::register::{RegisterStatus, _RegisterStatus};
use status_protoc::my_trait::StatusTrait;
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use crate::error::Error;
use crate::services::auth::Auth;
use crate::components::footer::Footer;
use crate::types::auth::RegisterInfo;
use crate::routes::AppRoute;
use crate::routes::from_js::{register_btn_enable, register_btn_disable};

pub struct Register {
    auth: Auth,
    request: RegisterInfo,
    response: Callback<Result<RegisterStatus, Error>>,
    task: Option<FetchTask>,
    tip: Html,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    Response(Result<RegisterStatus, Error>),
    UpdateUserName(String),
    UpdateEmail(String),
    UpdatePassword(String),
    Ignore,
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            auth: Auth::new(),
            request: RegisterInfo::default(),
            response: link.callback(Msg::Response),
            task: None,
            tip: html! { <p class="alert alert-info">{ "注册Pipe (pipe.unsafe.me)" }</p> },
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(response)) => {
                match response.status() {
                    _RegisterStatus::UserNameHasExisted => {
                        self.tip = html! { <p class="alert alert-danger">{ "用户名已存在" }</p> };
                    }
                    _RegisterStatus::EmailHasExisted => {
                        self.tip = html! { <p class="alert alert-danger">{ "邮件已存在" }</p> };
                    }
                    _RegisterStatus::InvalidEmailAddress => {
                        self.tip = html! { <p class="alert alert-danger">{ "邮件格式错误" }</p> };
                    }
                    _RegisterStatus::UserNameTooShort => {
                        self.tip = html! { <p class="alert alert-danger">{ "用户名太短" }</p> };
                    }
                    _RegisterStatus::PasswordTooShort => {
                        self.tip = html! { <p class="alert alert-danger">{ "密码太短" }</p> };
                    }
                    _RegisterStatus::DbAPIError => {
                        self.tip = html! { <p class="alert alert-danger">{ "数据库错误，请联系管理员" }</p> };
                    }
                    _RegisterStatus::SendEmailError => {
                        self.tip = html! { <p class="alert alert-danger">{ "邮件发送错误，请联系管理员" }</p> };
                    }
                    _RegisterStatus::RegisterSuccessfully => self.router_agent.send(ChangeRoute(AppRoute::Active.into()))
                }
                register_btn_enable();
            }
            Msg::Response(Err(_)) => (),
            Msg::UpdateUserName(name) => self.request.user_name = name,
            Msg::UpdateEmail(email) => self.request.user_email = email,
            Msg::UpdatePassword(password) => self.request.user_password = password,
            Msg::Request => {
                register_btn_disable();
                self.task = Some(self.auth.register(self.request.clone(), self.response.clone()))
            },
            Msg::Ignore => ()
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
        });

        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateUserName(ev.value));
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));

        html! {
            <>
                <link href="register.css" rel="stylesheet" type="text/css"/>
                <div class="container">
                    <form onsubmit=onsubmit>
                        <h1>{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                        { self.tip.clone() }
                        <div class="form-group ">
                            <label class="control-label" for="name">{ "用户名" }</label>
                            <input type="name" class="form-control"
                            name="name"
                            id="name"
                            placeholder="请输入 用户名"
                            required=true
                            value=&self.request.user_name
                            oninput=oninput_name
                            />
                        </div>

                        <div class="form-group ">
                            <label class="control-label" for="email">{ "邮箱" }</label>
                            <input type="email" class="form-control"
                            name="email"
                            id="email"
                            placeholder="请输入 邮箱"
                            required=true
                            value=&self.request.user_email
                            oninput=oninput_email
                            />
                        </div>

                        <div class="form-group ">
                            <label for="password">{ "密码" }</label>
                            <input type="password" class="form-control"
                            name="password"
                            id="password"
                            placeholder="请输入 密码"
                            required=true
                            value=&self.request.user_password
                            oninput=oninput_password
                            />
                        </div>

                        <div class="text-right">
                            <button type="submit" id="register_btn" class="btn btn-default">{ "注册" }</button>
                        </div>
                    </form>
                </div>
                <Footer />
            </>
        }
    }
}