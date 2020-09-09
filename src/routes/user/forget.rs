use yew::{
    ComponentLink,
    Component,
    Html,
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use status_protoc::my_trait::StatusTrait;
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use status_protoc::status::user::check::{CheckStatus, _CheckStatus};
use status_protoc::status::user::change::{ChangeStatus, _ChangeStatus};
use crate::error::Error;
use crate::services::auth::Auth;
use crate::routes::AppRoute;
use crate::components::footer::Footer;
use crate::types::auth::NewPassword;
use crate::routes::from_js::{change_btn_disable, change_btn_enable, send_btn_enable, send_btn_disable};

pub struct Forget {
    auth: Auth,
    request: NewPassword,
    response_change: Callback<Result<ChangeStatus, Error>>,
    response_send: Callback<Result<CheckStatus, Error>>,
    ready: bool,
    email: String,
    tip: Html,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ResponseChange(Result<ChangeStatus, Error>),
    ResponseSend(Result<CheckStatus, Error>),
    RequestChange,
    RequestSend,
    UpdateCode(String),
    UpdatePassword(String),
    UpdateEmail(String),
    Ignore,
}

impl Component for Forget {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            auth: Auth::new(),
            request: NewPassword::default(),
            response_change: link.callback(Msg::ResponseChange),
            response_send: link.callback(Msg::ResponseSend),
            ready: false,
            email: String::default(),
            tip: html! {<p class="alert alert-info">{ "密码重置, 重置成功将会跳转到登录界面" }</p>},
            task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ResponseChange(Ok(response)) => {
                self.task = None;
                match response.status() {
                    _ChangeStatus::InvalidCode => {
                        self.tip = html! { <p class="alert alert-danger">{ "验证码错误" }</p> };
                    }
                    _ChangeStatus::PasswordTooShort => {
                        self.tip = html! { <p class="alert alert-danger">{ "密码太短" }</p> };
                    }
                    _ChangeStatus::DbAPIError => {
                        self.tip = html! { <p class="alert alert-danger">{ "数据库错误，请联系管理员" }</p> };
                    }
                    _ChangeStatus::ChangeSuccessfully => {
                        self.router_agent.send(ChangeRoute(AppRoute::Login.into()))
                    }
                }
                change_btn_enable();
            }
            Msg::ResponseSend(Ok(response)) => {
                self.task = None;
                match response.status() {
                    _CheckStatus::SendSuccessfully => {
                        self.ready = true;
                        self.tip = html! { <p class="alert alert-success">{ "验证码发送成功" }</p> };
                    }
                    _CheckStatus::SendEmailError => {
                        self.tip = html! { <p class="alert alert-danger">{ "邮件发送错误，请联系管理员" }</p> };
                    }
                    _CheckStatus::InvalidEmailAddress => {
                        self.tip = html! { <p class="alert alert-danger">{ "邮件格式错误" }</p> };
                    }
                    _CheckStatus::DbAPIError => {
                        self.tip = html! { <p class="alert alert-danger">{ "数据库错误，请联系管理员" }</p> };
                    }
                }
                send_btn_enable();
            }
            Msg::RequestChange => {
                self.task = Some(self.auth.reset_password(self.request.clone(), self.response_change.clone()));
                change_btn_disable();
            },
            Msg::RequestSend => {
                self.task = Some(self.auth.send_check_code(self.email.clone(), self.response_send.clone()));
                send_btn_disable();
            }
            Msg::UpdateCode(code) => self.request.code = code,
            Msg::UpdatePassword(pd) => self.request.new_password = pd,
            Msg::UpdateEmail(e) => self.email = e,
            Msg::Ignore => (),
            Msg::ResponseChange(Err(_)) => (),
            Msg::ResponseSend(Err(_)) => ()
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let inner = if !self.ready {
            let oninput_email = self
                .link
                .callback(|ev: InputData| Msg::UpdateEmail(ev.value));

            let onsubmit = self.link.callback(|ev: FocusEvent| {
                ev.prevent_default();
                Msg::RequestSend
            });

            html! {
                <div class="container">
                    <form onsubmit=onsubmit>
                        <h1>{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                        { self.tip.clone() }

                        <div class="form-group">
                            <label class="control-label" for="email">{ "邮箱" }</label>
                            <input type="email" class="form-control"
                            name="email"
                            id="email"
                            placeholder="请输入 邮箱"
                            required=true
                            value=&self.email
                            oninput=oninput_email
                            />
                        </div>

                        <p class="help-block"> { "如果用户存在，会将发送密码重置验证码到您的邮箱，请注意查收。" }</p>
                        <div class="text-right">
                            <button type="submit" id="send_btn" class="btn btn-default">{ "发送" }</button>
                        </div>
                    </form>
                </div>
            }
        } else {
            let oninput_code = self
                .link
                .callback(|ev: InputData| Msg::UpdateCode(ev.value));

            let oninput_password = self
                .link
                .callback(|ev: InputData| Msg::UpdatePassword(ev.value));

            let onsubmit = self.link.callback(|ev: FocusEvent| {
                ev.prevent_default();
                Msg::RequestChange
            });

            html! {
                <div class="container">
                    <form onsubmit=onsubmit>
                        <h1>{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                        { self.tip.clone() }
                        <div class="form-group">
                            <label class="control-label" for="code">{ "验证码" }</label>
                            <input type="text" class="form-control"
                            name="code"
                            id="code"
                            placeholder="请输入 验证码"
                            required=true
                            value=&self.request.code
                            oninput=oninput_code
                            />
                        </div>

                        <div class="form-group">
                            <label class="control-label" for="password">{ "密码" }</label>
                            <input type="password" class="form-control"
                            name="password"
                            id="password"
                            placeholder="请输入 新密码"
                            required=true
                            value=&self.request.new_password
                            oninput=oninput_password
                            />
                        </div>

                        <div class="text-right">
                            <button type="submit" id="change_btn" class="btn btn-default">{ "修改" }</button>
                        </div>
                    </form>
                </div>
            }
        };

        html! {
            <>
                <link href="register.css" rel="stylesheet" type="text/css"/>
                { inner }
                <Footer />
            </>
        }
    }
}