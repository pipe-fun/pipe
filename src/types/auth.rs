#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoginInfo {
    pub user_name: String,
    pub user_password: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewPassword {
    pub code: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub authorized: bool,
    pub user_name: String,
}

impl UserInfo {
    pub fn new(name: &str) -> Self {
        Self {
            authorized: true,
            user_name: name.into(),
        }
    }
}