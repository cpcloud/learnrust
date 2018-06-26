extern crate clap;
extern crate learnrust;

use clap::Arg;
use learnrust::common;
use std::io::BufRead;
use std::result::Result;

fn main() {
    let app = common::make_app("first").arg(
        Arg::with_name("lines")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .help("Number of lines to output")
            .takes_value(true),
    );
    let matches = app.get_matches();
    let reader = common::get_lines_reader(&matches);

    let nlines = if let Some(value) = matches.value_of("lines") {
        value.parse().expect(&format!("Unable to parse {}", value))
    } else {
        10usize
    };

    reader
        .lines()
        .take(nlines)
        .map(Result::unwrap)
        .map(|line| println!("{}", line))
        .collect()
}
