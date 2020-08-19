use chrono::{NaiveDateTime, NaiveTime};

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: i32,
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

#[derive(Serialize, Deserialize)]
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