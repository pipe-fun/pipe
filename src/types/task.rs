use chrono::{NaiveDateTime, NaiveTime, NaiveDate};
use yew::Html;
use yew::prelude::*;
use crate::routes::console::task::edit::TaskEdit;
use crate::error::Error;
use std::str::FromStr;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewTask {
    pub name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    owner: String,
    pub command: String,
    pub execute_time: NaiveTime,
    pub device_token: String,
    pub active: bool,
}

impl Default for NewTask {
    fn default() -> Self {
        Self {
            name: String::default(),
            succeed_count: 0,
            failed_count: 0,
            last_executed: NaiveDate::from_ymd(1970, 1, 1)
                .and_hms(0, 0, 0),
            owner: String::default(),
            command: String::default(),
            execute_time: NaiveTime::from_hms(0, 0, 0),
            device_token: String::default(),
            active: false,
        }
    }
}

impl NewTask {
    pub fn edit_name(&mut self, name: &str) {
        self.name = name.into();
    }

    pub fn edit_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn edit_command(&mut self, command: &str) {
        self.command = command.into();
    }

    pub fn edit_time(&mut self, time: &str) {
        let t = match NaiveTime::from_str(time) {
            Ok(t) => t,
            Err(_) => NaiveTime::from_hms(0,0,0),
        };
        self.execute_time = t;
    }

    pub fn edit_token(&mut self, token: &str) {
        self.device_token = token.into();
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
            active: task.active,
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