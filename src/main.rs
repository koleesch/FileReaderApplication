use core::str;
use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("My file name is: {}", args[1git]);

    match File::open(args[1].clone()) {
        Ok(file) => {
            let mut reader =  BufReader::new(file);
            let mut buffer = Vec::new();
            match reader.read_to_end(&mut buffer) {
                Ok(_) => {
                    match str::from_utf8(&buffer) {
                        Ok(valid_text) => {
                            println!("File content: {}", valid_text);
                        }
                        Err(_) => {
                            println!("Invalid UTF-8 sequence");
                        }
                    }
                },
                Err (error) => {
                    println!("Error reading file: {:?}", error);
                }

            }
        }
        Err(error) => {
            println!("Error opening file: {:?}", error);
        }    
}
}