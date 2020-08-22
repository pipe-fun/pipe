use yew::Html;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PreDevice {
    name: String,
    owner: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    token: String,
    name: String,
    owner: String
}

impl Device {
    pub fn get_view(&self) -> Html {
        html! {
            <tr>
                <td>
                    <span data-toggle="popover"
                    title={ self.name.clone() }
                    data-content={ self.owner.clone() }>{ self.name.clone() }</span>
                </td>
                <td>{ self.token.clone() }</td>
                <td>{ 2 }</td>
                <td>
                    <a href="/tpl/4625/edit" target="_blank">{ "编辑 " }</a>
                    <a data-method="POST" href="/tpl/4625/del" data-confirm="是否要删除模板?">{ "删除 " }</a>
                    <a class="modal_load" href="/task/new?tplid=4625">{ "新建任务" }</a>
                </td>
            </tr>
        }
    }
}