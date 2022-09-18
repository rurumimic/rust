use std::fs::File;
use std::io::ErrorKind;

#[derive(Debug)]
struct Aa {
    name: i32,
}

fn main() {
    let aa = Aa { name: 32 };

    println!("{:?}", aa);

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

fn _crash() {
    panic!("crash and burn");
    // thread 'main' panicked at 'crash and burn'
}

fn _out_of_range() {
    let v = vec![1, 2, 3];
    v[99];
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
}

fn file_not_found() {
    let greeting_file_result = File::open("hello.txt");
    // Result<File, Error> path
    // Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
