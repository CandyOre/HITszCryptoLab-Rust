use super::err::IOError;

use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};

use std::path::Path;

pub struct IOHelper {}

// get functions
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
            .chars()
            .filter(|c|
                c.is_ascii_alphanumeric() ||
                c.is_ascii_punctuation()
            ).collect();
        if len != 0 && len != line.len() {
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

// print functions
impl IOHelper {
    pub fn make_char_hex<T, E>(content: T, len: usize)
        -> Vec<String>
    where
        T: IntoIterator<Item = E>,
        E: std::fmt::LowerHex,
    {
        content.into_iter()
            .map(|x| format!("{:#01$x}", x, len + 2))
            .collect()
    }

    pub fn print_with_newline<T, E>(content: T, per_line: usize)
    where
        T: IntoIterator<Item = E>,
        E: std::fmt::Display,
    {
        let mut cnt = 0 as usize;
        for e in content {
            print!("{e}");
            cnt += 1;
            if cnt >= per_line {
                cnt = 0;
                print!("\n");
            } else {
                print!(" ");
            }
        }
    }
}

// file functions   
impl IOHelper {
    pub fn write_file(path: &String, content: &[u8]) -> Result<(), IOError> {
        let path = Path::new(&path);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;
        file.write_all(content)?;
        Ok(())
    }

    pub fn read_file(path: &String) -> Result<Vec<u8>, IOError> {
        let path = Path::new(&path);
        let mut file = OpenOptions::new()
            .read(true)
            .open(&path)?;
        let mut content = [0u8; 1024];
        match file.read(&mut content) {
            Ok(n) => Ok(content[0..n].to_vec()),
            Err(e) => Err(IOError::from(e)),
        }
    }
}