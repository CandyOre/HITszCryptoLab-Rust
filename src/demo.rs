use crate::console::io::IOHelper;

pub trait Demo {

    fn get_name(&self) -> String;
    fn start_demo(&mut self);

    fn acquire_key(&mut self);
    fn acquire_cypher(&mut self);
    fn decrypt_plain(&mut self);

    fn acquire_plain() -> String {
        IOHelper::get_string_loop(
            0,
            "\nPlease input plain text: ".to_string(),
        )
    }

    fn write_file(save_path: &mut String, cypher: &Vec<u8>) {
        *save_path = IOHelper::get_string_loop(
            0,
            "\nPlease input save path(default path: enc.hex): ".to_string(),
        );
        if save_path.is_empty() {
            *save_path = "enc.hex".to_string();
        }
        if let Err(e) = IOHelper::write_file(
            save_path,
            cypher,
        ) {
            println!("Error occur during writing file.");
            panic!("Demo - write_file: {}", e.to_string());
        }
    }

    fn read_file(save_path: &mut String, cypher: &mut Vec<u8>) -> bool {
        if "1" != IOHelper::get_string_loop(
            0, 
            "\nInput \"1\" to continue decypher demo: ".to_string()
        ) {
            false
        } else {
            *save_path = IOHelper::get_string_loop(
                0,
                "\nPlease input save path(default path: enc.hex): ".to_string(),
            );
            if save_path.is_empty() {
                *save_path = "enc.hex".to_string();
            }
            match IOHelper::read_file(save_path) {
                Ok(content) =>
                    *cypher = content,
                Err(e) => {
                    println!("Error occur during reading file.");
                    panic!("Demo - read_file: {}", e.to_string());
                }
            }
            true
        }
    }

    fn decrypt_write(save_path: &String, decoded: &String) {
        if let Err(e) = IOHelper::write_file(
            &(save_path.clone() + ".txt"),
            &decoded.as_bytes(),
        ) {
            println!("Error occur during writing file.");
            panic!("Demo - write_file: {}", e.to_string());
        }
        println!("\nThe plain text is available at \"{}.txt\".", &save_path);
    }
}