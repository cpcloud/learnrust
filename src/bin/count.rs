extern crate learnrust;

use learnrust::common;
use std::io::BufRead;

fn main() {
    let app = common::make_app("total");
    let reader = common::get_lines_reader(&app.get_matches());
//    let mut line = String::new(); // may also use with_capacity if you can guess
//    let mut count = 0_usize;
//    while reader.read_line(&mut line).unwrap() > 0 {
//        // do something with line
//        count += 1_usize;
//        line.clear(); // clear to reuse the buffer
//    }
    println!("{}", reader.lines().count());
}
