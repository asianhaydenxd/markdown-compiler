
```rs src/main.rs
use clap::Parser;

mod file_reader;
mod compiler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Names of target files
    filenames: Vec<String>,
}


fn main() {
    let args = Args::parse();
    
    for filename in args.filenames {
        match file_reader::read_file(filename) {
            Ok(filetext) => compiler::compile(filetext),
            Err(error) => panic!("{}", error),
        }
    }
}
```

```rs src/file_reader.rs
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
```

```rs src/compiler.rs
use std::{fs::File, io::Write};

pub fn compile(text: String) {
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index..=index+2] == "```".chars().collect::<Vec<char>>() {
            render_file(chars.clone(), &mut index);
        }
        index += 1;
    }
}

fn render_file(chars: Vec<char>, index: &mut usize) {
    let mut delimiter_len = 0;
    while matches!(chars.get(*index), Some('`')) {
        *index += 1;
        delimiter_len += 1;
    }

    while !matches!(chars.get(*index), Some( ' ' | '\n' | '\t' )) {
        *index += 1;
    }

    while matches!(chars.get(*index), Some( ' ' | '\n' | '\t' )) {
        *index += 1;
    }

    let file_name = get_word(chars.clone(), index);

    let mut code = String::new();

    loop {
        if chars[*index] == '\n' {
            if chars[*index+1..*index+delimiter_len+1].iter().all(|x| *x == '`') {
                *index += delimiter_len + 1;
                break;
            }
        }
        code.push(chars[*index]);
        *index += 1;
    }

    let mut file = File::create(file_name).unwrap();

    file.write_all(code.as_bytes()).unwrap();
}

fn get_word(chars: Vec<char>, index: &mut usize) -> String {
    let mut word: String = String::new();
    
    while !matches!(chars.get(*index), Some( ' ' | '\n' | '\t' )) {
        word.push(chars[*index]);
        *index += 1;
    }

    while chars[*index] != '\n' {
        *index += 1;
    }
    *index += 1;

    word
}

```