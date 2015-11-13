#[macro_use]
extern crate clap;

use clap::{App};

fn main() {
    let matches = App::new("clap-test")
        .arg_from_usage("--print=<something>... 'Print stuff'")
        .get_matches();

    for thing in matches.values_of("something").unwrap() {
        println!("stuff: {}", thing);
    }
}
