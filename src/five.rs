
use crate::util::{read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a("src/input/five_short.txt"), 0);
    assert_eq!(get_a("src/input/five.txt"), 0)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/five_short.txt"), 0);
    assert_eq!(get_b("src/input/five.txt"), 0)
}

pub fn get_b(file: &str) -> i32 {
    split_lines(read_file(file).as_str()).into_iter().len() as i32
}

pub fn get_a(file: &str) -> i32 {
    split_lines(read_file(file).as_str()).into_iter().len() as i32
}

