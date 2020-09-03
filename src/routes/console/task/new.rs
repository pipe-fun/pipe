use yew::{
    Callback,
    Component,
    ComponentLink,
    Html
};

use crate::types::task::{
    NewTask,
    Task
};

use status_protoc::status::console::task::TaskStatus;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use log::debug;
use std::str::FromStr;
use crate::services::device::DeviceRequest;
use crate::types::device::Device;
use crate::services::task::TaskRequest;
use crate::error::Error;

pub struct CreateTask {
    tr: TaskRequest,
    dr: DeviceRequest,
    response: Callback<Result<TaskStatus, Error>>,
    read_task_response: Callback<Result<Vec<Task>, Error>>,
    read_device_response: Callback<Result<Vec<Device>, Error>>,
    request: NewTask,
    task: Option<FetchTask>,
    devices: Vec<Device>,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Response(Result<TaskStatus, Error>),
    TaskReadResponse(Result<Vec<Task>, Error>),
    DeviceReadResponse(Result<Vec<Device>, Error>),
    Request,
    UpdateTaskName(String),
    UpdateActive(ChangeData),
    UpdateExecuteTime(String),
    UpdateToken(ChangeData),
    UpdateCommand(String),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<(Vec<Task>, String)>,
}

impl Component for CreateTask {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            tr: TaskRequest::new(),
            dr: DeviceRequest::new(),
            response: link.callback(Msg::Response),
            read_task_response: link.callback(Msg::TaskReadResponse),
            read_device_response: link.callback(Msg::DeviceReadResponse),
            request: NewTask::default(),
            task: None,
            devices: vec![],
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::TaskReadResponse(Ok(ts)) => {
                self.task = None;
                self.props.callback.emit((ts, self.request.device_token.clone()));
            },
            Msg::DeviceReadResponse(Ok(ds)) => {
                self.task = None;
                self.devices = ds;
            },
            Msg::Response(_) => {
                self.task = None;
                self.task = Some(self.tr.read(self.read_task_response.clone()));
            },
            Msg::Request => {
                debug!("{:?}", self.request);
                self.task = Some(self.tr.create(self.request.clone(), self.response.clone()))
            },
            Msg::UpdateTaskName(n) => self.request.edit_name(&n),
            Msg::UpdateActive(select) => {
                if let ChangeData::Select(select) = select {
                    self.request.edit_active(bool::from_str(&select.value()).unwrap())
                }
            },
            Msg::UpdateExecuteTime(t) => self.request.edit_time(&t),
            Msg::UpdateCommand(c) => self.request.edit_command(&c),
            Msg::UpdateToken(select) => {
                if let ChangeData::Select(select) = select {
                    self.request.edit_token(&select.value())
                }
            },
            _ => ()
        }
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.dr
                    .read(self.read_device_response.clone()),
            );
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        self.request = NewTask::default();
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Request
        });

        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateTaskName(ev.value));

        let oninput_command = self
            .link
            .callback(|ev: InputData| Msg::UpdateCommand(ev.value));

        let oninput_time = self
            .link
            .callback(|ev: InputData| Msg::UpdateExecuteTime(ev.value));

        let onchange_active = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateActive(ev));

        let onchange_device = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateToken(ev));

        html! {
            <form onsubmit=onsubmit>
                <h1 class="text-center">{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                <div id="run-result"></div>
                <div class="form-group">
                    <label class="control-label"><span>{ "任务名" }</span></label>
                    <input type="text" class="form-control" placeholder="请输入 任务名"
                    value=&self.request.name
                    oninput=oninput_name
                    required=true
                    />
                </div>

                <div class="form-group">
                    <label class="control-label"><span>{ "命令" }</span></label>
                    <input type="text" class="form-control" placeholder="请输入 命令"
                    value=&self.request.command
                    oninput=oninput_command
                    required=true
                    />
                </div>

                <div class="form-group">
                    <label class="control-label"><span>{ "执行时间" }</span></label>
                    <input  type="time" class="form-control"
                    min="00:00" max="23:59" step="1" required=true
                    value=&self.request.execute_time
                    oninput=oninput_time
                    />
                </div>

                <div class="form-group">
                    <label class="control-label"><span>{ "绑定设备" }</span></label>
                    <select class="form-control" onchange=onchange_device required=true>
                        <option value="" selected=true hidden=true>{ "请选择 设备" }</option>
                        { for self.devices
                        .iter()
                        .map(|d| { html! { <option value=d.token.clone()>{ d.name.clone() }</option> } })}
                    </select>
                </div>

                <div id="variables">
                    <div class="form-group">
                        <label class="control-label"><span>{ "状态" }</span></label>
                        <select class="form-control" onchange=onchange_active required=true>
                            <option value="true">{ "激活" }</option>
                            <option value="false" selected=true>{ "禁用" }</option>
                         </select>
                    </div>
                </div>

               <div class="text-right">
                    <button type="submit" class="btn btn-primary">{ "提交" }</button>
               </div>
            </form>
        }
    }
}