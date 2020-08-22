use crate::services::requests::Requests;
use crate::error::Error;
use yew::Callback;
use crate::types::task::Task;
use yew::services::fetch::FetchTask;
use crate::types::device::Device;

#[derive(Default, Debug)]
pub struct DeviceRequest {
    requests: Requests,
}

impl DeviceRequest {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn read(
        &mut self,
        callback: Callback<Result<Vec<Device>, Error>>,
    ) -> FetchTask {
        self.requests.get::<Vec<Device>>(
            "/console/device/read".to_string(),
            callback,
        )
    }
}