extern crate clap;
extern crate learnrust;

use clap::Arg;
use learnrust::common;
use std::collections::VecDeque;
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
        5usize
    };

    let mut lines: VecDeque<String> = VecDeque::with_capacity(nlines);

    for line in reader.lines().map(Result::unwrap) {
        let current_number_of_lines = lines.len();

        // if adding a line would overflow then pop the front
        if current_number_of_lines + 1 > nlines {
            lines.pop_front();
        }
        lines.push_back(line);
    }

    for line in &lines {
        println!("{}", line);
    }
}
