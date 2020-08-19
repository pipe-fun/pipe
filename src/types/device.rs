#[derive(Serialize, Deserialize)]
pub struct PreDevice {
    name: String,
    owner: String
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    token: String,
    name: String,
    owner: String
}