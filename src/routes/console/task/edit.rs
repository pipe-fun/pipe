use crate::services::task::TaskRequest;
use crate::error::Error;
use crate::types::task::{NewTask, Task};
use yew::{Callback, Component, ComponentLink, Html};
use status_protoc::status::console::task::TaskStatus;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use log::debug;
use std::str::FromStr;

pub struct TaskEdit {
    tr: TaskRequest,
    error: Option<Error>,
    request: NewTask,
    response: Callback<Result<TaskStatus, Error>>,
    read_response: Callback<Result<Vec<Task>, Error>>,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    DeleteRequest,
    UpdateRequest,
    Response(Result<TaskStatus, Error>),
    ReadResponse(Result<Vec<Task>, Error>),
    UpdateTaskName(String),
    UpdateActive(ChangeData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub task: Task,
    pub callback: Callback<Result<Vec<Task>, Error>>,
}

impl Component for TaskEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            tr: TaskRequest::new(),
            error: None,
            request: NewTask::from(&props.task),
            response: link.callback(Msg::Response),
            read_response: link.callback(Msg::ReadResponse),
            task: None,
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(_)) => {
                self.task = None;
                self.task = Some(self.tr.read(self.read_response.clone()));
            },
            Msg::Response(Err(e)) => self.error = Some(e),
            Msg::ReadResponse(ts) => self.props.callback.emit(ts),
            Msg::DeleteRequest => {
                self.task = Some(self.tr.delete(self.props.task.id, self.response.clone()));
            },
            Msg::UpdateRequest => {
                self.task = Some(self.tr.put(self.props.task.id
                                             , self.request.clone()
                                             , self.response.clone()));
            },
            Msg::UpdateTaskName(n) => self.request.edit_name(&n),
            Msg::UpdateActive(select) => {
                if let ChangeData::Select(select) = select {
                    debug!("{:?}", select);
                    self.request.edit_active(bool::from_str(&select.value()).unwrap())
                }
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let delete = self.link.callback(|_| Msg::DeleteRequest);
        let onsubmit = self.link.callback(|_| Msg::UpdateRequest);
        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateTaskName(ev.value));
        let onchange_active = self
            .link
            .callback(|ev: ChangeData| Msg::UpdateActive(ev));

        html! {
            <div class="modal-dialog">
                <div id="edit" class="modal-content" onsubmit=onsubmit>
                    <form>
                        <h1 class="text-center">{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                        <div id="run-result"></div>
                        <div class="form-group">
                            <label class="control-label"><span>{ "任务名" }</span></label>
                            <input type="text" class="form-control" placeholder="请输入 任务名"
                            value=&self.request.name
                            oninput=oninput_name
                            />
                        </div>

                        <div id="variables">
                            <div class="form-group">
                                <label class="control-label"><span>{ "状态" }</span></label>
                                <select class="form-control" onchange=onchange_active>
                                    <option value=&self.request.active>{ "不修改" }</option>
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
                </div>
            </div>
        }
    }
}