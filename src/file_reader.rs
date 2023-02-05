use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

pub fn read_file(filename: String) -> Result<String, Error> {
    match File::open(filename.clone()) {
        Ok(mut file) => get_file_contents(filename, &mut file),
        Err(error) => Err(error),
    }
}

fn get_file_contents(filename: String, file: &mut File) -> Result<String, Error> {
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => {
            match fs::read_dir(filename) {
                Ok(dir) => {
                    let mut full_string = String::new();
                    for subpath in dir {
                        full_string.push_str(&read_file(subpath?.path().display().to_string())?);
                    }
                    Ok(full_string)
                },
                Err(error) => Err(error),
            }
        },
    }
}