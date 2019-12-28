use std::fs::File;
use std::io::{self, BufRead, BufReader};

extern crate clap;
use self::clap::{App, Arg, ArgMatches};

pub fn make_app(name: &str) -> App {
    App::new(name)
        .version("1.0")
        .author("Phillip Cloud <cpcloud@gmail.com>")
        .about("Output the first n lines of file or stdin")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Input file")
                .required(false)
                .index(1),
        )
}

pub fn get_lines_reader(matches: &ArgMatches) -> Box<BufRead> {
    match matches.value_of("file") {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(
            File::open(filename).expect(&format!("Unable to open file {}", filename)),
        )),
    }
}
