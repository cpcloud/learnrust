extern crate learnrust;

use learnrust::common;
use std::io::BufRead;

fn main() {
    let app = common::make_app("total");
    let reader = common::get_lines_reader(&app.get_matches());
    println!("{}", reader.lines().count());
}
