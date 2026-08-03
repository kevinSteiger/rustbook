use std::fs::File;
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char>{
    text.lines().next()?.chars().last()
}

fn main() {
    let greeting_file_result = File::open("hello.txt").expect("hello.txt should be included in this project");

    let greeting_file = File::open("hello.txt")?;

}
