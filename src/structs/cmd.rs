pub struct Cmd {
    name: String,
    key: String,
    description: String,
}

impl Cmd {
    pub fn new(name: String, key: String) -> Cmd {
        Cmd {
            name,
            key,
            description: "".to_string(),
        }
    }

    pub fn describe(&mut self, value: &String) {
        self.description = value.clone();
    }
}
