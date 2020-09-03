use yew::{
    Callback,
    Component,
    ComponentLink,
    Html,
};

use super::{
    edit::TaskEdit,
    new::CreateTask,
};

use crate::routes::console::from_js::{
    unShow,
    deleteBackDrop,
    show,
};

use yew::prelude::*;
use yew::services::fetch::FetchTask;
use web2core::protoc::OpResult;
use crate::routes::console::view::Route;
use crate::services::task::TaskRequest;
use crate::error::Error;
use crate::types::task::Task;

pub struct TaskView {
    tr: TaskRequest,
    response_first: Callback<Result<Vec<Task>, Error>>,
    execute_response: Callback<Result<OpResult, Error>>,
    reload_response: Callback<Result<OpResult, Error>>,
    task: Option<FetchTask>,
    tasks: Vec<Task>,
    route: Route,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Response((Vec<Task>, String)),
    ResponseFirst(Result<Vec<Task>, Error>),
    ExecuteResponse(Result<OpResult, Error>),
    ReloadResponse(Result<OpResult, Error>),
    Edit(Task),
    Execute(Task),
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
            response_first: link.callback(Msg::ResponseFirst),
            execute_response: link.callback(Msg::ExecuteResponse),
            reload_response: link.callback(Msg::ReloadResponse),
            task: None,
            tasks: vec![],
            route: Route::None,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(info) => {
                unShow();
                deleteBackDrop();
                self.tasks = info.0;
                self.route = Route::None;
                self.task = Some(self.tr.reload(&info.1, self.reload_response.clone()));
            },
            Msg::ResponseFirst(Ok(ts)) => {
                self.tasks = ts;
                self.route = Route::None;
            },
            Msg::ExecuteResponse(Ok(result)) => {
                self.task = None;
                let html = match result {
                    OpResult::Ok => html! { <h1 class="alert alert-success text-center"> { "执行成功" }</h1> },
                    OpResult::CoreOffline => html! { <h1 class="alert alert-danger text-center">{ "未与核心连接" }</h1> },
                    OpResult::DeviceOffline => html! { <h1 class="alert alert-danger text-center">{ "设备离线" }</h1> },
                };
                self.props.callback.emit(Route::Execute(html));
                show();
            },
            Msg::ReloadResponse(_) => {
                self.task = None;
            },
            Msg::Edit(t) => {
                let callback = self.link.callback(Msg::Response);
                let html = html! { <TaskEdit task=t callback=callback.clone() /> };
                self.props.callback.emit(Route::Edit(html));
                show();
            },
            Msg::Execute(t) => {
                self.task = Some(self.tr.execute(t, self.execute_response.clone()));
            },
            Msg::New => {
                let callback = self.link.callback(Msg::Response);
                let html = html! { <CreateTask callback=callback.clone() /> };
                self.props.callback.emit(Route::New(html));
                show();
            },
            _ => {}
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
                    .read(self.response_first.clone()),
            );
        }
    }

    fn view(&self) -> Html {
        let tbody = self.tasks.iter().map(|t| {
            let t_c1 = t.clone();
            let t_c2 = t.clone();
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
                        <button class="my_button" onclick=self.link.callback(move |_| Msg::Edit(t_c1.clone()))>
                        { "编辑" }</button>
                        <button class="my_button my_button_offset" onclick=self.link.callback(move |_| Msg::Execute(t_c2.clone()))>
                        { "立即执行" }</button>
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