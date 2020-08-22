use crate::services::requests::Requests;
use crate::error::Error;
use yew::Callback;
use crate::types::task::Task;
use yew::services::fetch::FetchTask;
use status_protoc::status::console::task::TaskStatus;

#[derive(Default, Debug)]
pub struct TaskRequest {
    requests: Requests,
}

impl TaskRequest {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn read(
        &mut self,
        callback: Callback<Result<Vec<Task>, Error>>,
    ) -> FetchTask {
        self.requests.get::<Vec<Task>>(
            "/console/task/read".to_string(),
            callback,
        )
    }

    pub fn read_by_id(
        &mut self,
        id: i32,
        callback: Callback<Result<Task, Error>>,
    ) -> FetchTask {
        self.requests.get::<Task>(
            format!("/console/task/read_by_id/{}", id),
            callback,
        )
    }

    pub fn delete(
        &mut self,
        id: i32,
        callback: Callback<Result<TaskStatus, Error>>,
    ) -> FetchTask {
        self.requests.delete::<TaskStatus>(
            format!("/console/task/delete/{}", id),
            callback,
        )
    }
}