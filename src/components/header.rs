use yew_router::prelude::*;
use yew::{html, Component, ComponentLink, Html};
use crate::routes::AppRoute;

pub struct Header;

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self { Self }

    fn update(&mut self, _msg: Self::Message) -> bool { true }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {
            <header>
                <div class="container">
                    <h1 class="text-center">{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                        <div class="buttons">
                            <RouterAnchor<AppRoute> route=AppRoute::Console
                            classes="btn btn-default pull-left">
                            { "首页" }
                            </RouterAnchor<AppRoute>>

                            <RouterAnchor<AppRoute> route=AppRoute::Logout
                            classes="btn btn-default pull-right">
                            { "登出" }
                            </RouterAnchor<AppRoute>>
                        </div>
                 </div>
             </header>
        }
    }
}
