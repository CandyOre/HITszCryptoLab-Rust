use crate::{
    console::io::IOHelper,
    cypher::{CypherCommon, Encryptor, Decryptor}
};

pub trait Demo {

    fn get_name(&self) -> String;

    fn acquire_key(&mut self);
    fn get_encryptor(&self) -> &dyn Encryptor;
    fn get_decryptor(&self) -> &dyn Decryptor;

    fn start_demo(&mut self) {
        let mut common: CypherCommon = Default::default();
        println!("\n======== Demo {} Start ========", self.get_name());
        self.acquire_key();
        acquire_plain(&mut common);
        self.acquire_cypher(&mut common);
        write_file(&mut common);
        if read_file(&mut common) {
            self.decrypt_plain(&mut common);
            decrypt_write(&mut common);
        }
        println!("\n========= Demo {} End =========", self.get_name());
    }

    fn acquire_cypher(&mut self, common: &mut CypherCommon) {
        println!("\nEncrypting...");
        common.cypher = self.get_encryptor().encrypt(&common.plain);
        println!("The cypher text is: ");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(common.cypher.clone(), 2),
            16
        );
    }

    fn decrypt_plain(&mut self, common: &mut CypherCommon) {
        println!("\nDecrypting...");
        common.plain = self.get_decryptor().decrypt(&common.cypher);
        println!("The plain text as ASCII is:");
        IOHelper::print_with_newline(
            IOHelper::make_char_hex(
                common.plain.as_bytes(),
                2),
            16
        );
        println!("The plain text is \"{}\".", common.plain);
    }
}

fn acquire_plain(common: &mut CypherCommon) {
    common.plain = IOHelper::get_string_loop(
        0,
        "\nPlease input plain text: ".to_string(),
    )
}

fn write_file(common: &mut CypherCommon) {
    common.save_path = IOHelper::get_string_loop(
        0,
        "\nPlease input save path(default path: enc.hex): ".to_string(),
    );
    if common.save_path.is_empty() {
        common.save_path = "enc.hex".to_string();
    }
    if let Err(e) = IOHelper::write_file(
        &common.save_path,
        &common.cypher,
    ) {
        println!("Error occur during writing file.");
        panic!("Demo - write_file: {}", e.to_string());
    }
}

fn read_file(common: &mut CypherCommon) -> bool {
    if "1" != IOHelper::get_string_loop(
        0, 
        "\nInput \"1\" to continue decypher demo: ".to_string()
    ) {
        false
    } else {
        common.save_path = IOHelper::get_string_loop(
            0,
            "\nPlease input save path(default path: enc.hex): ".to_string(),
        );
        if common.save_path.is_empty() {
            common.save_path = "enc.hex".to_string();
        }
        match IOHelper::read_file(&common.save_path) {
            Ok(content) =>
                common.cypher = content,
            Err(e) => {
                println!("Error occur during reading file.");
                panic!("Demo - read_file: {}", e.to_string());
            }
        }
        true
    }
}

fn decrypt_write(common: &mut CypherCommon) {
    if let Err(e) = IOHelper::write_file(
        &(common.save_path.clone() + ".txt"),
        &common.plain.as_bytes(),
    ) {
        println!("Error occur during writing file.");
        panic!("Demo - write_file: {}", e.to_string());
    }
    println!("\nThe plain text is available at \"{}.txt\".", &common.save_path);
}