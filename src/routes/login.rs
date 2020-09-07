use yew::{
    ComponentLink,
    Component,
    Html,
};

use crate::types::auth::{
    LoginInfo,
    UserInfo,
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use status_protoc::status::user::login::{LoginStatus, _LoginStatus};
use status_protoc::my_trait::StatusTrait;
use yew_router::agent::RouteAgent;
use yew_router::agent::RouteRequest::ChangeRoute;
use crate::error::Error;
use crate::services::auth::Auth;
use crate::routes::AppRoute;
use crate::components::footer::Footer;
use crate::routes::from_js::{login_btn_disable, login_btn_enable};

pub struct Login {
    auth: Auth,
    error: Option<Error>,
    request: LoginInfo,
    response: Callback<Result<LoginStatus, Error>>,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    tip: Html,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    Response(Result<LoginStatus, Error>),
    Ignore,
    UpdateUserName(String),
    UpdatePassword(String),
    Register,
}

#[derive(PartialEq, Properties, Clone, Default)]
pub struct Props {
    /// Callback when user is logged in successfully
    pub callback: Callback<UserInfo>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            auth: Auth::new(),
            error: None,
            request: LoginInfo::default(),
            response: link.callback(Msg::Response),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            tip: html! { <></> },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Request => {
                login_btn_disable();
                let request = self.request.clone();
                self.task = Some(self.auth.login(request, self.response.clone()))
            }
            Msg::Response(Ok(response)) => {
                self.error = None;
                self.task = None;
                match response.status() {
                    _LoginStatus::LoginSuccessfully => {
                        let info = UserInfo::new(&response.get_user_name());
                        self.props.callback.emit(info);
                        self.router_agent.send(ChangeRoute(AppRoute::Console.into()));
                    }
                    _LoginStatus::UserNameOrPasswordWrongOrNoActive => {
                        self.tip = html! { <p class="alert alert-danger">{ "账号密码错误，或者未激活" }</p> };
                    }
                    _LoginStatus::DbAPIError => {
                        self.tip = html! { <p class="alert alert-danger">{ "数据库错误，请联系管理员" }</p> };
                    }
                }
                login_btn_enable();
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
                login_btn_enable();
            }
            Msg::UpdateUserName(value) => self.request.user_name = value,
            Msg::UpdatePassword(value) => self.request.user_password = value,
            Msg::Ignore => {}
            Msg::Register => self.router_agent.send(ChangeRoute(AppRoute::Register.into())),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
        });

        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateUserName(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));

        let register = self.link.callback(|_| Msg::Register);

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
                            <button type="submit" id="login_btn" class="btn btn-default">{ "登录" }</button>
                            <button type="submit" class="btn btn-default" onclick=register>{ "注册" }</button>
                        </div>
                    </form>
                </div>
                <Footer />
            </>
        }
    }
}