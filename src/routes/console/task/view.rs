use crate::services::task::TaskRequest;
use crate::error::Error;
use crate::types::task::Task;
use yew::{Callback, Component, ComponentLink, Html};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use super::{
    edit::TaskEdit,
    new::CreateTask,
};

use crate::routes::console::from_js::{unShow, deleteBackDrop, show};
use crate::routes::console::view::Route;

pub struct TaskView {
    tr: TaskRequest,
    response: Callback<Result<Vec<Task>, Error>>,
    task: Option<FetchTask>,
    tasks: Vec<Task>,
    route: Route,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    Response(Result<Vec<Task>, Error>),
    Edit(Task),
    New,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<Route>,
}

impl Component for TaskView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            tr: TaskRequest::new(),
            response: link.callback(Msg::Response),
            task: None,
            tasks: vec![],
            route: Route::None,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Ok(ts)) => {
                unShow();
                deleteBackDrop();
                self.tasks = ts;
                self.route = Route::None;
            }
            Msg::Edit(t) => {
                let callback = self.link.callback(Msg::Response);
                let html = html! { <TaskEdit task=t callback=callback.clone() /> };
                self.props.callback.emit(Route::Edit(html));
                show();
            }
            Msg::New => {
                let callback = self.link.callback(Msg::Response);
                let html = html! { <CreateTask callback=callback.clone() /> };
                self.props.callback.emit(Route::New(html));
                show();
            }
            _ => ()
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(
                self.tr
                    .read(self.response.clone()),
            );
        }
    }

    fn view(&self) -> Html {
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
                        <a class="modal_load" onclick=self.link.callback(move |_| Msg::Edit(t_c.clone()))>
                        { "编辑 " }</a>
                        <a class="modal_load">{ "立即执行" }</a>
                    </td>
                </tr>
            }
        }).collect::<Html>();

        html! {
            <section class="task">
                <div class="container">
                    <h2>
                    { "我的任务 " }
                    <a onclick=self.link.callback(|_| Msg::New) class="btn btn-default btn-xs modal_load glyphicon glyphicon-plus"></a>
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
        }
    }
}