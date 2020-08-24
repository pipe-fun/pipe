#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct NewDevice {
    pub name: String,
    owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Device {
    pub token: String,
    pub name: String,
    pub owner: String,
}

impl NewDevice {
    pub fn edit_name(&mut self, name: &str) {
        self.name = name.into();
    }
}

impl Device {
    pub fn edit_name(&mut self, name: &str) {
        self.name = name.into();
    }
}