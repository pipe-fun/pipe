use yew::{
    Callback,
    Component,
    ComponentLink,
    Html,
};

use crate::types::task::{
    NewTask,
    Task
};

use status_protoc::status::console::task::TaskStatus;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use std::str::FromStr;
use crate::types::device::Device;
use crate::services::device::DeviceRequest;
use crate::services::task::TaskRequest;
use crate::error::Error;

pub struct TaskEdit {
    tr: TaskRequest,
    dr: DeviceRequest,
    error: Option<Error>,
    request: NewTask,
    response: Callback<Result<TaskStatus, Error>>,
    read_task_response: Callback<Result<Vec<Task>, Error>>,
    read_device_response: Callback<Result<Vec<Device>, Error>>,
    task: Option<FetchTask>,
    devices: Vec<Device>,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    DeleteRequest,
    UpdateRequest,
    Response(Result<TaskStatus, Error>),
    TaskReadResponse(Result<Vec<Task>, Error>),
    DeviceReadResponse(Result<Vec<Device>, Error>),
    UpdateTaskName(String),
    UpdateActive(ChangeData),
    UpdateExecuteTime(String),
    UpdateToken(ChangeData),
    UpdateCommand(String),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Task,
    pub callback: Callback<(Vec<Task>, String)>,
}

impl Component for TaskEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            tr: TaskRequest::new(),
            dr: DeviceRequest::new(),
            error: None,
            request: NewTask::from(&props.task),
            response: link.callback(Msg::Response),
            read_task_response: link.callback(Msg::TaskReadResponse),
            read_device_response: link.callback(Msg::DeviceReadResponse),
            task: None,
            devices: vec![],
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(_)) => {
                self.task = None;
                self.task = Some(self.tr.read(self.read_task_response.clone()));
            }
            Msg::Response(Err(e)) => self.error = Some(e),
            Msg::TaskReadResponse(Ok(ts)) => {
                self.task = None;
                self.props.callback.emit((ts, self.props.task.device_token.clone()));
            }
            Msg::DeviceReadResponse(Ok(ds)) => {
                self.task = None;
                self.devices = ds;
            }
            Msg::DeleteRequest => {
                self.task = Some(self.tr.delete(self.props.task.id, self.response.clone()));
            }
            Msg::UpdateRequest => {
                self.task = Some(self.tr.update(self.props.task.id
                                                , self.request.clone()
                                                , self.response.clone()));
            }
            Msg::UpdateTaskName(n) => self.request.edit_name(&n),
            Msg::UpdateActive(select) => {
                if let ChangeData::Select(select) = select {
                    self.request.edit_active(bool::from_str(&select.value()).unwrap())
                }
            }
            Msg::UpdateExecuteTime(t) => self.request.edit_time(&t),
            Msg::UpdateCommand(c) => self.request.edit_command(&c),
            Msg::UpdateToken(select) => {
                if let ChangeData::Select(select) = select {
                    self.request.edit_token(&select.value())
                }
            }
            Msg::DeviceReadResponse(Err(_)) => {}
            _ => {}
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
        self.request = NewTask::from(&self.props.task);
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

        let device_options = self
            .devices
            .iter()
            .map(|d| if d.token.eq(&self.request.device_token) {
                html! { <option value=d.token.clone() selected=true>{ d.name.clone() }</option> }
            } else {
                html! { <option value=d.token.clone()>{ d.name.clone() }</option> }
            }).collect::<Html>();

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
                        { device_options }
                     </select>
                </div>

                <div id="variables">
                    <div class="form-group">
                        <label class="control-label"><span>{ "状态" }</span></label>
                        <select class="form-control" onchange=onchange_active required=true>
                            <option value=&self.request.active selected=true>{ "不修改" }</option>
                            <option value="true">{ "激活" }</option>
                            <option value="false">{ "禁用" }</option>
                         </select>
                    </div>
                </div>

               <div class="text-right">
                    <a class="btn btn-default" onclick=delete>{ "删除" }</a>
                    <button type="submit" class="btn btn-primary">{ "提交" }</button>
               </div>
            </form>
        }
    }
}