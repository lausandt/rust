
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let george = User { //George is immutable
        active: true,
        username: String::from("George the Forge"),
        email: String::from("george@woolyrhino.com"),
        sign_in_count: 1
    };
    
    println!("{}", george.email);

    let mut croc = User { //Croc is a mutable struct
        active: true,
        username: String::from("George the Forge"), 
        email: String::from("george@woolyrhino.com"),
        sign_in_count: 1
    };

    croc.username = String::from("Croc the land salty");
    croc.email = String::from("croc@peckish.com");

    println!("{}", croc.username);

    let ente = User {
        username: String::from("Ente"),
        email: String::from("ente@ibjc.org"),
        ..george // copies the values are not introduced from the george struct
    };

    println!("Ich bin {}!", ente.username);

    let black = Colour(0,0,0);
    let origin = Point(0,0,0);

    println!("colour = {}, point = {}", black.0, origin.1)


}

struct User {
    active: bool,
    username: String, //String type rather than the &str string slice type ensure that structs own their data
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true, // default value at creation
        username, //this can be coded as username:username
        email,    //ditto
        sign_in_count: 1, //starting value at creation
    }
}
