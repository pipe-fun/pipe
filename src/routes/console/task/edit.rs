use crate::services::task::TaskRequest;
use crate::error::Error;
use crate::types::task::{NewTask, Task};
use yew::{Callback, Component, ComponentLink, Html};
use status_protoc::status::console::task::TaskStatus;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use log::debug;

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
            }
            Msg::UpdateRequest => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let delete = self.link.callback(|_| Msg::DeleteRequest);
        html! {
            <div class="modal-dialog">
                <div id="edit" class="modal-content">
                    <form>
                        <h1 class="text-center">{ "Pipe" }<sup>{ "alpha" }</sup></h1>
                        <div id="run-result"></div>
                        <div class="form-group">
                            <label class="control-label">
                              <a id="siteurl" href="" target="_blank" style="color:black;">{ "任务名" }</a>
                            </label>
                            <input type="text" class="form-control" name="cookie" placeholder="请输入 cookie"/>
                        </div>

                        <div id="variables">
                            <div class="form-group">
                                <label class="control-label" for="input-cookie"><span>{ "激活" }</span></label>
                                <input type="text" class="form-control" name="cookie" placeholder="请输入 cookie"/>
                            </div>
                        </div>

                       <div class="text-right">
                            <a class="btn btn-default" onclick=delete data-confirm="是否要删除任务?">{ "删除" }</a>
                            <button type="submit" data-loading-text="loading..." class="btn btn-primary">{ "提交" }</button>
                       </div>
                    </form>
                </div>
            </div>
        }
    }
}