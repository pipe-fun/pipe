use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use crate::components::{
    header::Header,
    footer::Footer,
};

use crate::routes::console::{
    task::view::TaskView,
    device::view::DeviceView
};
use crate::services::task::TaskRequest;
use crate::error::Error;
use crate::types::task::Task;

pub struct Console;
pub enum Msg {}

impl Component for Console {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self { Self }

    fn update(&mut self, _msg: Self::Message) -> bool { true }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <link href="my.css" rel="stylesheet" type="text/css"/>
                <Header />
                <TaskView />
                <DeviceView />
                <Footer />
            </>
        }
    }
}