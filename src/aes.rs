use crate::demo::Demo;
use crate::console::io::IOHelper;

pub struct Aes {}

impl Aes {
    fn acquire_key() -> String {
        let key = IOHelper::get_string_loop(
            16,
            "Please input key (16 characters long): ".to_string(),
        );
        println!("Your key is {key}.");
        key
    }
}

impl Demo for Aes {
    fn get_name() -> String {
        "Advanced Encryption Standard".to_string()
    }

    fn start_demo() {
        let key = Aes::acquire_key();
    }
}