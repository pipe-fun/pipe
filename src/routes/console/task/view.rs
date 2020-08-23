use crate::services::task::TaskRequest;
use crate::error::Error;
use crate::types::task::{NewTask, Task};
use yew::{Callback, Component, ComponentLink, Html};
use status_protoc::status::console::task::TaskStatus;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use log::debug;
use super::edit::TaskEdit;
use wasm_bindgen::{prelude::wasm_bindgen};

pub struct TaskView {
    tr: TaskRequest,
    error: Option<Error>,
    response: Callback<Result<Vec<Task>, Error>>,
    task: Option<FetchTask>,
    tasks: Vec<Task>,
    this_task: Option<Task>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    Response(Result<Vec<Task>, Error>),
    Edit(Option<Task>),
}

impl Component for TaskView {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            tr: TaskRequest::new(),
            error: None,
            response: link.callback(Msg::Response),
            task: None,
            tasks: vec![],
            this_task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(ts)) => {
                unShow();
                deleteBackDrop();
                self.this_task = None;
                self.tasks = ts;
                true
            },
            Msg::Edit(t) => {
                self.this_task = t;
                show();
                true
            }
            _ => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.tr
                    .read(self.response.clone()),
            );
            addEditEvent();
        }
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(Msg::Response);
        let task = self.this_task.clone();

        let tbody = self.tasks.iter().map(|t| {
            let t_c = t.clone();
            html! {
                <tr>
                    <td>
                        <span data-toggle="popover"
                        title={ t.id }
                        data-content={ t.name.clone() }>{ t.name.clone() }</span>
                    </td>

                    <td>{ t.succeed_count }</td>
                    <td>{ t.failed_count }</td>
                    <td>{ t.last_executed }</td>
                    <td>{ if t.active { "激活" } else { "禁用" } }</td>
                    <td>{ "10 小时后" }</td>

                    <td>
                        <a class="modal_load" onclick=self.link.callback(move |_| Msg::Edit(Some(t_c.clone())))>
                        { "修改 " }</a>
                        <a class="modal_load">{ "立即执行" }</a>
                    </td>
                </tr>
            }
        }).collect::<Html>();

        html! {
            <>
                <div class="modal fade" id="modal_load"
                tabindex="-1" role="dialog" aria-hidden="true" style="display: none;">
                    {
                        if task.is_some() {
                            html! { <TaskEdit task=task.unwrap() callback=callback.clone() /> }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <section class="task">
                    <div class="container">
                        <h2>
                        { "我的任务 " }
                        <a href="/task/new" class="btn btn-default btn-xs modal_load glyphicon glyphicon-plus"></a>
                        </h2>
                        <table class="table">
                            <thead>
                                <tr>
                                    <th>{ "任务" }</th>
                                    <th>{ "成功执行次数" }</th>
                                    <th>{ "失败次数" }</th>
                                    <th>{ "上次成功时间" }</th>
                                    <th>{ "状态" }</th>
                                    <th>{ "预计下次执行时间" }</th>
                                    <th>{ "操作" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { tbody }
                            </tbody>
                        </table>
                    </div>
                </section>
            </>
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn addEditEvent();
    fn show();
    fn unShow();
    fn deleteBackDrop();
}