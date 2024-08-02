use std::fs::File;
use std::io::prelude::*;

// TODO: handle non-ascii
// TODO: handle string literals across line breaks
// ...

/*
fn string_to_words(string: String) -> Vec<&str> {
    let mut result = Vec::new();
    let mut current_word = String::new();
    let mut inside_str = false;
    for (i, ch) in string.bytes().enumerate() {
        match (ch as char) {
            ' ' if inside_str => current_word.push(' '),
            ' ' | '\n'             => {
                if current_word.trim() != "" {
                    result.push(current_word);
                }
            }   
            "\"" => {
                inside_str = !inside_str;

            }
        }
    }

}*/

fn pad_to_square(string: String, width: usize) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    let mut available: usize = width;
    let mut result = String::new();
    for word in &words {
        assert!(word.len() < width as usize);
        if word.len() < available - 4 {
            result.push_str(word);
            available -= word.len();

            result.push(' ');
            available -= 1;
        }
        else {
            result.push_str("/*");
            available -= 2;

            while available > 6 {
                result.push_str("*/ /*");
                available -= 5;
            }

            while available > 2 {
                result.push('*');
                available -= 1;
            }
            result.push_str("*/");
            available -= 2;

            assert_eq!(available, 0);
            available = width;

            result.push('\n');
            result.push_str(word);
            available -= word.len();

            result.push(' ');
            available -= 1;
        }
    }
    result.push_str("/*");
    available -= 2;

    while available > 6 {
        result.push_str("*/ /*");
        available -= 5;
    }

    while available > 2 {
        result.push('*');
        available -= 1;
    }
    result.push_str("*/\n");
    available -= 2;
    assert_eq!(available, 0);
    return result;
}

fn pad_to_square_inner(string: String, width: usize) -> String {
    let string = string
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n");
    let words: Vec<&str> = string.split_whitespace().collect();
    let mut result = String::new();
    let start = "\"";
    result.push_str(start);
    let mut available: usize= width - start.len();
    for word in &words {
        assert!(word.len() < width as usize);
        if word.chars().count() < available - 6 {
            result.push_str(word);
            available -= word.chars().count();

            result.push(' ');
            available -= 1;
        }
        else {
            while available > 2 {
                result.push('@');
                available -= 1;
            }
            result.push_str("\",");
            available -= 2;

            assert_eq!(available, 0);
            available = width;
            result.push('\n');

            result.push('\"');
            available -= 1;

            result.push_str(word);
            available -= word.chars().count();

            result.push(' ');
            available -= 1;
        }
    }
    // pop space
    result.pop();
    while available > 0 {
        result.push('@');
        available -= 1;
    }
    result.push_str("\"");
    //available -= 2;
    assert_eq!(available, 0);
    result.push('\n');
    return result;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("stage1.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    const ASPECT_RATIO: f32 = 32.0 / 9.0;
    let width = (contents.len() as f32 * 2.0* ASPECT_RATIO).sqrt() as usize + 2 as usize;
    let _height = (contents.len() as f32 * 2.0 / ASPECT_RATIO).sqrt() as usize;
    let mut content_iter = contents.split("\"`\"");
    let beginning : String = content_iter.next().unwrap().to_string();
    let ending :String = content_iter.next().unwrap().to_string();
    let beginning = pad_to_square(beginning, width);
    let ending = pad_to_square(ending, width);
    let mut middle = String::new();
    middle.push_str(&beginning);
    middle.push_str("\"`\"\n");
    middle.push_str(&ending);
    let middle = pad_to_square_inner(middle, width);
    let mut file = File::create("stage2.rs")?;
    write!(file, "{}", beginning)?;
    write!(file, "{}", middle)?;
    write!(file, "{}", ending)?;
    Ok(())
}
