use crate::services::device::DeviceRequest;
use crate::error::Error;
use yew::{Callback, Component, ComponentLink, Html};
use status_protoc::status::console::task::TaskStatus;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use crate::types::device::{PreDevice, Device};
use status_protoc::status::console::device::DeviceStatus;


pub struct DeviceView {
    dr: DeviceRequest,
    error: Option<Error>,
    request: PreDevice,
    response: Callback<Result<DeviceStatus, Error>>,
    read_response: Callback<Result<Vec<Device>, Error>>,
    task: Option<FetchTask>,
    devices: Vec<Device>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    ReadRequest,
    CreateRequest,
    DeleteRequest,
    UpdateRequest,
    Response(Result<DeviceStatus, Error>),
    ReadResponse(Result<Vec<Device>, Error>),
    Ignore,
}

impl Component for DeviceView {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            dr: DeviceRequest::new(),
            error: None,
            request: PreDevice::default(),
            response: link.callback(Msg::Response),
            read_response: link.callback(Msg::ReadResponse),
            task: None,
            devices: vec![],
            link
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.dr.read(self.read_response.clone()),
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ReadResponse(Ok(ds)) => self.devices = ds,
            _ => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {
            <section class="tpl">
                <div class="container">
                    <h2>
                     { "我的设备 " }
                     <a href="/har/edit" class="btn btn-default btn-xs glyphicon glyphicon-plus" target="_blank"></a>
                    </h2>
                    <table class="table">
                        <thead>
                            <tr>
                                <th>{ "设备" }</th>
                                <th>{ "Token" }</th>
                                <th>{ "任务数" }</th>
                                <th>{ "操作" }</th>
                            </tr>
                      </thead>
                      <tbody>
                          { for self.devices.iter().map(|d| d.get_view()) }
                      </tbody>
                    </table>
                </div>
            </section>
        }
    }
}