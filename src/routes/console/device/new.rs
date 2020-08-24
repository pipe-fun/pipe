use yew::{
    Callback,
    Component,
    ComponentLink,
    Html
};

use crate::types::device::{
    Device,
    NewDevice
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use status_protoc::status::console::device::DeviceStatus;
use crate::error::Error;
use crate::services::device::DeviceRequest;

pub struct CreateDevice {
    dr: DeviceRequest,
    response: Callback<Result<DeviceStatus, Error>>,
    read_device_response: Callback<Result<Vec<Device>, Error>>,
    request: NewDevice,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Response(Result<DeviceStatus, Error>),
    DeviceReadResponse(Result<Vec<Device>, Error>),
    Request,
    UpdateDeviceName(String),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<Result<Vec<Device>, Error>>,
}

impl Component for CreateDevice {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            dr: DeviceRequest::new(),
            response: link.callback(Msg::Response),
            read_device_response: link.callback(Msg::DeviceReadResponse),
            request: NewDevice::default(),
            task: None,
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DeviceReadResponse(ds) => {
                self.task = None;
                self.props.callback.emit(ds);
            },
            Msg::Response(_) => {
                self.task = None;
                self.task = Some(self.dr.read(self.read_device_response.clone()));
            },
            Msg::Request => {
                self.task = Some(self.dr.create(self.request.clone(), self.response.clone()))
            },
            Msg::UpdateDeviceName(n) => self.request.edit_name(&n),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        self.request = NewDevice::default();
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
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
                    <button type="submit" class="btn btn-primary">{ "提交" }</button>
               </div>
            </form>
        }
    }
}