use crate::demo::Demo;
use crate::console::io::IOHelper;

pub struct Aes {
    io : IOHelper,
}

impl Aes {
    pub fn new() -> Self {
        Self {
            io : IOHelper::new(),
        }
    }
}

impl Demo for Aes {
    fn get_name(&self) -> String {
        "Advanced Encryption Standard".to_string()
    }

    fn start_demo(&self) {
        todo!()
    }
}