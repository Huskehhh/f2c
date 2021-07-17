extern crate clap;

use std::fs;

use clap::{App, Arg};
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let matches = App::new("f2c")
        .version("1.0")
        .author("Jordyn N. <jordyn@husk.pro>")
        .about("Copy file to clipboard")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // This is required, it should never be None
    let input = matches.value_of("INPUT").unwrap();

    match fs::read_to_string(input) {
        Ok(data) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

            match ctx.set_contents(data) {
                Ok(_) => {
                    println!("Contents copied to clipboard.");
                },
                Err(why) => {
                    eprintln!("Error when copying contents to clipboard... {}", why);
                }
            }
        }
        Err(why) => {
            eprintln!("Error reading file to string... {}", why);
        }
    }
}
