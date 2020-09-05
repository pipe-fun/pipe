use yew::{Callback,
          Component,
          ComponentLink,
          Html,
};

use super::{
    edit::DeviceEdit,
    new::CreateDevice,
};

use crate::routes::console::from_js::{
    unShow,
    deleteBackDrop,
    show,
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use crate::types::device::Device;
use crate::routes::console::view::Route;
use crate::services::device::DeviceRequest;
use crate::error::Error;

pub struct DeviceView {
    dr: DeviceRequest,
    response: Callback<Result<Vec<Device>, Error>>,
    task: Option<FetchTask>,
    devices: Vec<Device>,
    route: Route,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Response(Result<Vec<Device>, Error>),
    Edit(Device),
    New,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<Route>,
}

impl Component for DeviceView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            dr: DeviceRequest::new(),
            response: link.callback(Msg::Response),
            task: None,
            devices: vec![],
            route: Route::None,
            props,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.dr.read(self.response.clone()),
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(ds)) => {
                unShow();
                deleteBackDrop();
                self.devices = ds;
                self.route = Route::None;
            }
            Msg::Edit(d) => {
                let callback = self.link.callback(Msg::Response);
                let html = html! { <DeviceEdit device=d callback=callback.clone() /> };
                self.props.callback.emit(Route::Edit(html));
                show();
            }
            Msg::New => {
                let callback = self.link.callback(Msg::Response);
                let html = html! { <CreateDevice callback=callback.clone() /> };
                self.props.callback.emit(Route::New(html));
                show();
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let tbody = self.devices.iter().map(|d| {
            let d_c = d.clone();
            html! {
                <tr>
                    <td>
                        <span data-toggle="popover"
                        title={ d.name.clone() }
                        data-content={ d.owner.clone() }>{ d.name.clone() }</span>
                    </td>

                    <td>{ d.token.clone() }</td>

                    <td>
                        <button class="my_button" onclick=self.link.callback(move |_| Msg::Edit(d_c.clone()))>
                        { "编辑" }</button>
                    </td>
                </tr>
            }
        }).collect::<Html>();

        html! {
            <section class="tpl">
                <div class="container">
                    <h2>
                     { "我的设备 " }
                     <a onclick=self.link.callback(|_| Msg::New) class="btn btn-default btn-xs modal_load glyphicon glyphicon-plus"></a>
                    </h2>
                    <table class="table">
                        <thead>
                            <tr>
                                <th>{ "设备" }</th>
                                <th>{ "Token" }</th>
                                <th>{ "操作" }</th>
                            </tr>
                      </thead>
                      <tbody>
                          { tbody }
                      </tbody>
                    </table>
                </div>
            </section>
        }
    }
}