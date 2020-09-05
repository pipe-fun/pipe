use crate::types::device::{
    Device,
    NewDevice,
};

use yew::Callback;
use yew::services::fetch::FetchTask;
use status_protoc::status::console::device::DeviceStatus;
use crate::services::requests::Requests;
use crate::error::Error;

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
            format!("/console/device/read"),
            callback,
        )
    }

    pub fn delete(
        &mut self,
        token: &str,
        callback: Callback<Result<DeviceStatus, Error>>,
    ) -> FetchTask {
        self.requests.delete::<DeviceStatus>(
            format!("/console/device/delete/{}", token),
            callback,
        )
    }

    pub fn update(
        &mut self,
        info: Device,
        callback: Callback<Result<DeviceStatus, Error>>,
    ) -> FetchTask {
        self.requests.put::<Device, DeviceStatus>(
            format!("/console/device/update"),
            info,
            callback,
        )
    }

    pub fn create(
        &mut self,
        info: NewDevice,
        callback: Callback<Result<DeviceStatus, Error>>,
    ) -> FetchTask {
        self.requests.post::<NewDevice, DeviceStatus>(
            format!("/console/device/create"),
            info,
            callback,
        )
    }
}