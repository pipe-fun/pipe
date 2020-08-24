use crate::error::Error;
use yew::{Callback, Component, ComponentLink, Html};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use crate::types::device::Device;
use crate::services::device::DeviceRequest;
use log::debug;
use status_protoc::status::console::device::DeviceStatus;

pub struct DeviceEdit {
    dr: DeviceRequest,
    error: Option<Error>,
    request: Device,
    response: Callback<Result<DeviceStatus, Error>>,
    read_device_response: Callback<Result<Vec<Device>, Error>>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    DeleteRequest,
    UpdateRequest,
    Response(Result<DeviceStatus, Error>),
    DeviceReadResponse(Result<Vec<Device>, Error>),
    UpdateDeviceName(String),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub device: Device,
    pub callback: Callback<Result<Vec<Device>, Error>>,
}

impl Component for DeviceEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            dr: DeviceRequest::new(),
            error: None,
            request: props.device.clone(),
            response: link.callback(Msg::Response),
            read_device_response: link.callback(Msg::DeviceReadResponse),
            task: None,
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(_)) => {
                self.task = None;
                self.task = Some(self.dr.read(self.read_device_response.clone()));
            },
            Msg::Response(Err(e)) => self.error = Some(e),
            Msg::DeviceReadResponse(ds) => {
                self.task = None;
                self.props.callback.emit(ds);
            },
            Msg::DeleteRequest => {
                self.task = Some(self.dr.delete(&self.props.device.token, self.response.clone()));
            },
            Msg::UpdateRequest => {
                self.task = Some(self.dr.update(self.request.clone(), self.response.clone()));
            },
            Msg::UpdateDeviceName(n) => self.request.edit_name(&n),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.request = props.device.clone();
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let delete = self.link.callback(|_| Msg::DeleteRequest);

        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::UpdateRequest
        });

        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateDeviceName(ev.value));

        html! {
            <form onsubmit=onsubmit>
                <h1 class="text-center">{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                <div id="run-result"></div>
                <div class="form-group">
                    <label class="control-label"><span>{ "设备名" }</span></label>
                    <input type="text" class="form-control" placeholder="请输入 设备名"
                    value=&self.request.name
                    oninput=oninput_name
                    required=true
                    />
                </div>

               <div class="text-right">
                    <a class="btn btn-default" onclick=delete>{ "删除" }</a>
                    <button type="submit" class="btn btn-primary">{ "提交" }</button>
               </div>
            </form>
        }
    }
}