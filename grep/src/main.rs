use std::env;

use grep::{Config, search} ;

use std::process;

fn main() {
    
    //env::args function returns an iterator!
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });



    // println!("we are looking for {} in {}", config.query, config.file_path);

    //grep::run(config);

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    search("oc", "Croc, George, Ente");
}
