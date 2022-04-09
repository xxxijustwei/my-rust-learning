use std::env;
use std::process;

use error_handle::*;


fn main() {
    let profile = Profile::new(env::args()).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(0);
    });

    if let Err(err) = error_handle::run(profile) {
        eprintln!("error: {}", err);
        process::exit(0);
    }
}