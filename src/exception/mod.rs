use std::fs::File;
use std::io::{*, self, ErrorKind, Read, Error};
use std::result;

pub fn test_exception(createfile: bool) {
    let file = String::from("hello.txt");
    if let Err(e) = File::open(&file) {
        println!("open file : {} failed:{}", &file, e);
        return;
    }
    println!("open file : {} success", &file);

    let file = "./src/exception/hello.txt";
    let f = File::open(file);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create(file) {
                    Ok(fc) => fc,
                    Err(e) => panic!("problem creating file:{}", file),
                },
                other => panic!("problem opening file {}, {:?}", file, other),
            }
        }
    };

    println!("open file:{:?} success", &f);

    println!("read file result:{:?}", match read_username_from_file(Some(f)) {
        Ok(name) => name,
        Err(err) => err.to_string(),
    })
}

pub fn test_unwarp_or_else() {
    // let f = File::open("hello.txt").unwrap();
    // left f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn read_username_from_file(opt: Option<File>) -> result::Result<String, io::Error> {
    let mut file: File = match opt {
        Some(f) => f,
        _ => File::open("./hello.txt")?,
    };
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}