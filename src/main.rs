use std::io;
use std::fs::File;
use std::io::ErrorKind;

const FILE_NAME: &str = "hello.txt";

fn main() {
    let f = open_or_create_file();
}

fn open_or_create_file() -> Result<File, io::Error> {
    let f = File::open(FILE_NAME); 
    match f {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_NAME) {
                Ok(fc) => Ok(fc),
                Err(e) => Err(e)
            },
            _ => Err(error),
        },
    }
}
