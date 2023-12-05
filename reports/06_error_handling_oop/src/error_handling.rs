use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

// Функция, которая возвращает ошибки в вызывающий код, используя оператор match
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


// Функция, возвращающая ошибки в вызывающий код с помощью оператора ?
fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Цепочка вызовов методов после оператора ?
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    // 1 способ обработки ошибок
    let greeting_file = File::open("hello.txt").unwrap();
    // 2 способ обработки ошибок
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}