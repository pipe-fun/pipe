use crate::types::auth::LoginInfo;
use yew::{ComponentLink, Component, Html};
use yew::format::Json;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use status_protoc::status::user::login::LoginStatus;
use crate::error::Error;
use crate::services::auth::Auth;

pub struct Login {
    auth: Auth,
    error: Option<Error>,
    request: LoginInfo,
    response: Callback<Result<LoginStatus, Error>>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
    test: String
}

pub enum Msg {
    Request,
    Response(Result<LoginStatus, Error>),
    Ignore,
    UpdateUserName(String),
    UpdatePassword(String),
}

#[derive(PartialEq, Properties, Clone, Default)]
pub struct Props {
    /// Callback when user is logged in successfully
    pub callback: Callback<LoginStatus>,
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
            link,
            test: String::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Request => {
                let request = self.request.clone();
                self.task = Some(self.auth.login(request, self.response.clone()))
            },
            Msg::Response(Ok(response)) => {
                self.test = serde_json::to_string_pretty(&response).unwrap();

                self.props.callback.emit(response);
                self.error = None;
                self.task = None;
                // Route to home page after logged in
                // self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            },
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            },
            Msg::UpdateUserName(value) => self.request.user_name = value,
            Msg::UpdatePassword(value) => self.request.user_password = value,
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Request
        });
        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateUserName(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));

        html! {
         <div class="auth-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-6 offset-md-3 col-xs-12">
                            <h1 class="text-xs-center">{ "Sign In" }</h1>
                            <form onsubmit=onsubmit>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="name"
                                            placeholder="Name"
                                            value=&self.request.user_name
                                            oninput=oninput_name
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            value=&self.request.user_password
                                            oninput=oninput_password
                                            />
                                    </fieldset>
                                    <button
                                        class="btn btn-lg btn-primary pull-xs-right"
                                        type="submit"
                                        disabled=false>
                                        { "Sign in" }
                                    </button>
                                </fieldset>
                            </form>
                            <p> { self.test.clone() } </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}