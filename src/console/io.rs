use super::err::IOError;

use std::io::{self, Write};

pub struct IOHelper {}

impl IOHelper {
    fn get_line() -> Result<String, IOError> {
        let mut buf = String::new();
        io::stdout().flush()?;
        io::stdin().read_line(&mut buf)?;
        Ok(buf)
    }

    pub fn get_string(len: usize) -> Result<String, IOError> {
        // only alphabet or numeric is kept
        let line: String = IOHelper::get_line()?
            .chars().filter(|c| c.is_ascii_alphanumeric()).collect();
        if len != line.len() {
            Err(IOError::MismatchedLength {
                required: (len), acquired: (line.len())
            })
        } else {
            Ok(line)
        }
    }

    pub fn get_string_loop(len: usize, prompt: String) -> String {
        loop{
            print!("{}", prompt);
            match IOHelper::get_string(len) {
                Ok(s) => return s,
                Err(e) => println!("{}", e.to_string()),
            }
        }
    }
}