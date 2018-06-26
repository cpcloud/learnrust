#![feature(alloc_system, lang_items)]

extern crate alloc_system;
extern crate clap;
extern crate learnrust;
extern crate regex;

use clap::Arg;
use learnrust::common;
use regex::Regex;
use std::io::BufRead;
use std::ops::Index;
use std::str::FromStr;

const FILTER_RE_STRING: &str = "(?P<index>\\d+)\\s*=\\s*(?P<regex>.+)";

fn matches_filter_pattern(string: String) -> Result<(), String> {
    let filter_re = Regex::new(FILTER_RE_STRING).unwrap();
    if !filter_re.is_match(&string) {
        Err(format!(
            "String {:?} does not match regex {:?}",
            string,
            filter_re.as_str()
        ))
    } else {
        Ok(())
    }
}

fn separator_is_length_1(string: String) -> Result<(), String> {
    let len = string.len();
    if len != 1 {
        Err(format!(
            "String {:?} must be length 1, but is length {}",
            string, len
        ))
    } else {
        Ok(())
    }
}

fn parse_filter_expr(expr: &str) -> (usize, Regex) {
    let filter_re = Regex::new(FILTER_RE_STRING).unwrap();
    let matched = filter_re.captures(expr).expect("Should not be reachable");
    let index = matched.index("index");
    let regex = matched.index("regex");
    (
        usize::from_str(index).expect("Should not be reachable"),
        Regex::new(regex).unwrap(),
    )
}

fn filter<R: BufRead>(reader: R, filter_exprs: &Vec<(usize, Regex)>, separator: &str) -> () {
    for line in reader.lines().map(Result::unwrap) {
        let pieces: Vec<_> = line.split(separator).collect();
        if filter_exprs
            .iter()
            .all(|(index, expr)| expr.is_match(pieces[*index - 1usize]))
        {
            println!("{}", line);
        }
    }
}

fn main() {
    let app = common::make_app("filter")
        .arg(
            Arg::with_name("filter")
                .short("f")
                .long("filter")
                .value_name("FILTER")
                .help("Regex based filter(s)")
                .required(true)
                .multiple(true)
                .takes_value(true)
                .validator(matches_filter_pattern),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .value_name("SEP")
                .help("Optional separator")
                .validator(separator_is_length_1)
                .required(false)
                .takes_value(true),
        );
    let matches = app.get_matches();
    let separator = matches.value_of("separator").unwrap_or(",");
    let reader = common::get_lines_reader(&matches);
    let filter_exprs: Vec<_> = matches
        .values_of("filter")
        .expect("Unable to extract filters")
        .map(parse_filter_expr)
        .collect();
    filter(reader, &filter_exprs, separator)
}
