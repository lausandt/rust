use std::fs::{File,read_to_string};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
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

    // same code  without the matches
    let _greeting_file2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //let rhino_file = File::open("rhino.txt").unwrap();

    //let croc_file = File::open("croc.txt").expect("croc.txt should be included in this project");

    //propagating 2: the error could be handled here
    let username = read_username_from_file();

    match username {
        Ok(user) => println!("{user}"),
        Err(e) => println!("{:?}", e),

    }

    let george = read_username_from_file4();

    match george {
        Ok(user) => println!("{user}"),
        Err(e) => println!("{:?}", e),

    }

    let empty = last_char_of_first_line("");

    match empty {
        Some(value) => Some(value),
        None => None,
    }; 


}


//  propagating 1 return the error to the calling code so that it can decide what to do
fn read_username_from_file() -> Result<String, io::Error> { //io::Error is the type of error we can expect
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

// this code is the same as above  it uses the ? operator to propogate the potential error to the calling function.
// The ? placed after a Result value is defined to work in almost the same way as the match expression
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

//short version for going the naming of username_file while propagating both potential errors with two ? 
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// shortest version, the read_to_string function returns a Result
fn read_username_from_file4() -> Result<String, io::Error> {
    read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
