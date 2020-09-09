use crate::types::auth::{LoginInfo, UserInfo, RegisterInfo, NewPassword};

use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use status_protoc::status::user::login::LoginStatus;
use status_protoc::status::user::register::RegisterStatus;
use status_protoc::status::user::active::ActiveStatus;
use status_protoc::status::user::check::CheckStatus;
use status_protoc::status::user::change::ChangeStatus;
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
            callback,
        )
    }

    /// Register a new user
    pub fn register(
        &mut self,
        register_info: RegisterInfo,
        callback: Callback<Result<RegisterStatus, Error>>,
    ) -> FetchTask {
        self.requests.post::<RegisterInfo, RegisterStatus>(
            format!("/user/register"),
            register_info,
            callback,
        )
    }

    pub fn authorize(
        &mut self,
        callback: Callback<Result<UserInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<UserInfo>(
            format!("/user/auth"),
            callback,
        )
    }

    pub fn active(
        &mut self,
        code: String,
        callback: Callback<Result<ActiveStatus, Error>>,
    ) -> FetchTask {
        self.requests.get::<ActiveStatus>(
            format!("/user/active/{}", code),
            callback,
        )
    }

    pub fn send_check_code(
        &mut self,
        email: String,
        callback: Callback<Result<CheckStatus, Error>>
    ) -> FetchTask {
        self.requests.get::<CheckStatus>(
            format!("/user/send_code/{}", email),
            callback
        )
    }

    pub fn reset_password(
        &mut self,
        new: NewPassword,
        callback: Callback<Result<ChangeStatus, Error>>
    ) -> FetchTask {
        self.requests.post::<NewPassword, ChangeStatus>(
            format!("/user/update_password"),
            new,
            callback
        )
    }
}
