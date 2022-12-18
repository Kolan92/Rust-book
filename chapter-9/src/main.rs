use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    match my_function_with_result(11) {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    }
    let greeting_file_result = File::open("hello.txt");
    match greeting_file_result {
        Ok(file) => {}
        Err(error) => {}
    }
    let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    panic!("crash and burn");

    let v = vec![1, 2, 3];

    v[99];
}

fn my_function_with_result(input: i32) -> Result<i32, &'static str> {
    if input < 10 {
        return Ok(input);
    }
    return Err("number is to big");
}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    //    fs::read_to_string("hello.txt")
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
