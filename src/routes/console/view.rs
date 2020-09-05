use crate::components::{
    header::Header,
    footer::Footer,
};

use crate::routes::console::{
    task::view::TaskView,
    device::view::DeviceView,
};

use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use crate::routes::console::from_js::addEditEvent;

pub enum Route {
    None,
    Edit(Html),
    New(Html),
    Execute(Html),
}

pub struct Console {
    route: Route,
    link: ComponentLink<Self>,
}

pub enum Msg {
    UpdateRoute(Route)
}

impl Component for Console {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            route: Route::None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute(route) => self.route = route,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            addEditEvent();
        }
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(Msg::UpdateRoute);

        let route = match &self.route {
            Route::None => html! {},
            Route::Edit(html) => html.clone(),
            Route::New(html) => html.clone(),
            Route::Execute(html) => html.clone(),
        };

        let part = html! {
            <div class="modal fade" id="modal_load"
            tabindex="-1" role="dialog" aria-hidden="true" style="display: none;">
                <div class="modal-dialog">
                    <div id="insert" class="modal-content">
                    { route }
                    </div>
                </div>
            </div>
        };

        html! {
            <>
                <link href="my.css" rel="stylesheet" type="text/css"/>
                <Header />
                { part }
                <TaskView callback=callback.clone()/>
                <DeviceView callback=callback/>
                <Footer />
            </>
        }
    }
}