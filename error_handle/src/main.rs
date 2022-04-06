use std::env;
use std::process;

use error_handle::Search;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let search = Search::new(&args).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(0);
    });

    if let Err(err) = error_handle::run(search) {
        eprintln!("error: {}", err);
        process::exit(0);
    }
}