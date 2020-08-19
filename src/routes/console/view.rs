use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use crate::components::{
    header::Header,
    footer::Footer
};

pub struct Console;
pub enum Msg {}

impl Component for Console {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <link href="my.css" rel="stylesheet" type="text/css"/>
                <section class="task">
                    <div class="container">
                        <h2>
                        { "我的任务 " }
                        <a href="/task/new" class="btn btn-default btn-xs modal_load glyphicon glyphicon-plus"></a>
                        </h2>
                        <table class="table">
                            <thead>
                                <tr>
                                    <th>{ "设备" }</th>
                                    <th>{ "成功执行次数" }</th>
                                    <th>{ "失败次数" }</th>
                                    <th>{ "上次成功时间" }</th>
                                    <th>{ "状态" }</th>
                                    <th>{ "预计下次执行时间" }</th>
                                    <th>{ "操作" }</th>
                                </tr>
                            </thead>
                        </table>
                    </div>
                </section>
                <Footer />
            </>
        }
    }
}