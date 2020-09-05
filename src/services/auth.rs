use crate::types::auth::{
    LoginInfo,
    UserInfo,
};

use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use status_protoc::status::user::login::LoginStatus;
use crate::error::Error;
use crate::services::requests::Requests;

/// Apis for authentication
#[derive(Default, Debug)]
pub struct Auth {
    requests: Requests,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Login a user
    pub fn login(
        &mut self,
        login_info: LoginInfo,
        callback: Callback<Result<LoginStatus, Error>>,
    ) -> FetchTask {
        self.requests.post::<LoginInfo, LoginStatus>(
            format!("/user/login"),
            login_info,
            callback,
        )
    }

    pub fn logout(
        &mut self,
        callback: Callback<Result<(), Error>>,
    ) -> FetchTask {
        self.requests.get::<()>(
            format!("/user/logout"),
            callback
        )
    }

    // /// Register a new user
    // pub fn register(
    //     &mut self,
    //     register_info: RegisterInfo,
    //     callback: Callback<Result<UserInfoWrapper, Error>>,
    // ) -> FetchTask {
    //     self.requests.post::<RegisterInfoWrapper, UserInfoWrapper>(
    //         "/user/register".to_string(),
    //         register_info,
    //         callback,
    //     )
    // }

    pub fn authorize(
        &mut self,
        callback: Callback<Result<UserInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<UserInfo>(
            format!("/user/auth"),
            callback,
        )
    }
}
