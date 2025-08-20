use std::env;
use std::process;

use scop::{Config, run};

fn main() {
    let conf = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments {err}");
        process::exit(1);
    });

    if let Err(e) = run(conf) {
        eprintln!("Application error : {e}");
        process::exit(1);
    }
}
