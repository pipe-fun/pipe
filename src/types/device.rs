#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NewDevice {
    name: String,
    owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Device {
    pub token: String,
    pub name: String,
    pub owner: String,
}

impl Device {
    pub fn edit_name(&mut self, name: &str) {
        self.name = name.into();
    }
}