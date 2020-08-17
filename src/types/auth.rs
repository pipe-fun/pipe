use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoginInfo {
    pub user_name: String,
    pub user_password: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_name: String,
}