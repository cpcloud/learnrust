extern crate learnrust;

use learnrust::common;
use std::io::BufRead;
use std::result::Result;
use std::str::FromStr;

fn main() {
    let app = common::make_app("total");
    let reader = common::get_lines_reader(&app.get_matches());
    let values = reader.lines().map(Result::unwrap).map(|string| {
        f64::from_str(&string).expect(&format!("Unable to parse {} into float64", &string))
    });
    let total: f64 = values.sum();
    println!("{}", total);
}
