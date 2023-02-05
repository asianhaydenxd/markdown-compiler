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
