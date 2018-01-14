use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

extern crate tle;

#[test]
fn norad100() {
    let f = File::open("tests/visual-100-brightest-2018-01-13.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let lines :Vec<&str> = contents.split("\r\n").collect();
    let num_tles = lines.len() / 3;
    let mut tle;
    for i in 0..num_tles {
        tle = tle::parse_tle(&format!("{}\n{}\n{}", lines[3*i], lines[3*i + 1], lines[3*i + 2]));
    }

    // just making sure it doesn't panic.
    assert_eq!(2, 2);
}
