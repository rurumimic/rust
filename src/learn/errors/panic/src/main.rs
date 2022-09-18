use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // panic

    // _crash();
    // _out_of_range();

    // recover erros

    // _file_not_found();
    // _file_not_found_recover();
    // _file_not_found_recover_shorten();

    // ? operator
    // _read_username_from_file();
    // _read_username_from_file_shorten();
    // _read_username_from_file_chaining();
    // _read_username_from_file_lib();

    // ? operator Option
    // _last_char_of_first_line("hello");
    // _last_char_of_first_line("");
    // _last_char_of_first_line("\nhello");

    let greeting_file = File::open("hello.txt")?;
    Ok(())
    // Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
}

fn _crash() {
    panic!("crash and burn");
    // thread 'main' panicked at 'crash and burn'
}

fn _out_of_range() {
    let v = vec![1, 2, 3];
    v[99];
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
}

fn _file_not_found() {
    let greeting_file_result = File::open("hello.txt");
    // Result<File, Error> path

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
}

fn _file_not_found_recover() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn _file_not_found_recover_shorten() {
    let gretting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _file_not_found_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
}

fn _file_not_found_expect() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// ? operator
fn _read_username_from_file() -> Result<String, io::Error> {
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

    // Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
}

fn _read_username_from_file_shorten() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file_chaining() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file_lib() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
