extern crate clap;
extern crate learnrust;

use clap::Arg;
use learnrust::common;
use std::io::BufRead;
use std::str::FromStr;

fn pick<R: BufRead>(reader: R, separator: &str, column_indices: &Vec<usize>) -> () {
    reader
        .lines()
        .map(|wrapped_line| {
            let line = wrapped_line.unwrap();
            let pieces: Vec<_> = line.split(separator).collect();
            let indexed_pieces: Vec<_> = column_indices
                .iter()
                .map(|index| pieces[*index - 1usize])
                .collect();
            println!("{}", indexed_pieces.join(separator))
        })
        .collect()
}

fn main() {
    let app = common::make_app("pick")
        .arg(
            Arg::with_name("columns")
                .short("c")
                .long("columns")
                .value_name("COLUMNS")
                .multiple(true)
                .use_delimiter(true)
                .help("Comma separated 1-based column indices to output")
                .required(true)
                .index(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .value_name("SEP")
                .help("Optional separator")
                .required(false)
                .takes_value(true),
        );
    let matches = app.get_matches();
    let reader = common::get_lines_reader(&matches);
    let separator = matches.value_of("separator").unwrap_or(",");
    let column_indices: Vec<_> = matches
        .values_of("columns")
        .unwrap()
        .map(|colstring| {
            usize::from_str(colstring).expect(&format!("Invalid column string {:?}", colstring))
        })
        .collect();

    pick(reader, separator, &column_indices)
}
