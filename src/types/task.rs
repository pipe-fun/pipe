use chrono::{NaiveDateTime, NaiveTime, NaiveDate};
use yew::Html;
use yew::prelude::*;
use crate::routes::console::task::edit::TaskEdit;
use crate::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub succeed_count: i32,
    pub failed_count: i32,
    pub last_executed: NaiveDateTime,
    pub owner: String,
    pub command: String,
    pub execute_time: NaiveTime,
    pub device_token: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewTask {
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    owner: String,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            succeed_count: 0,
            failed_count: 0,
            last_executed: NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
            owner: "".to_string(),
            command: "".to_string(),
            execute_time: NaiveTime::from_hms(0,0,0),
            device_token: "".to_string(),
            active: false
        }
    }
}
impl NewTask {
    pub fn edit_name(self, name: &str) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }

    pub fn edit_active(self, active: bool) -> Self {
        Self {
            active,
            ..self
        }
    }

    pub fn from(task: &Task) -> Self {
        Self {
            name: task.name.clone(),
            succeed_count: task.succeed_count,
            failed_count: task.failed_count,
            last_executed: task.last_executed,
            owner: task.owner.clone(),
            command: task.command.clone(),
            execute_time: task.execute_time,
            device_token: task.device_token.clone(),
            active: task.active
        }
    }
}

impl Task {
    pub fn get_view(&self) -> Html {
        html! {
            <tr>
                <td>
                    <span data-toggle="popover"
                    title={ self.id }
                    data-content={ self.name.clone() }>{ self.name.clone() }</span>
                </td>

                <td>{ self.succeed_count }</td>
                <td>{ self.failed_count }</td>
                <td>{ self.last_executed }</td>
                <td>{ "正常" }</td>
                <td>{ "10 小时后" }</td>

                <td>
                    <a class="modal_load">{ "修改 " }</a>
                    <a class="modal_load">{ "立即执行" }</a>
                </td>
            </tr>
        }
    }
}