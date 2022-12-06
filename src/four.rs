use crate::util::{int, read_file, split_lines};
use regex::Regex;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

#[test]
fn a() {
    assert_eq!(get_a("src/input/four_short.txt"), 2);
    assert_eq!(get_a("src/input/four.txt"), 556)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/four_short.txt"), 4);
    assert_eq!(get_b("src/input/four.txt"), 876)
}

fn in_interval(min: &i32, x: &i32, max: &i32) -> bool {
    min <= x && x <= max
}

fn is_any_pair_in_other(t: &(i32, i32, i32, i32)) -> bool {
    let (a, b, x, y) = t;
    (in_interval(a, x, b) && in_interval(a, y, b)) || (in_interval(x, a, y) && in_interval(x, b, y))
}

fn is_any_overlap((a, b, x, y): &(i32, i32, i32, i32)) -> bool {
    let ab: HashSet<i32, RandomState> =
        HashSet::from_iter((a.clone()..(b.clone() + 1)).into_iter());
    let xy: HashSet<i32, RandomState> =
        HashSet::from_iter((x.clone()..(y.clone() + 1)).into_iter());

    !(&ab & &xy).is_empty()
}

pub fn get_b(file: &str) -> i32 {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    split_lines(read_file(file).as_str())
        .into_iter()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (int(&cap[1]), int(&cap[2]), int(&cap[3]), int(&cap[4]))
        })
        .filter(is_any_overlap)
        .count() as i32
}

pub fn get_a(file: &str) -> i32 {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    split_lines(read_file(file).as_str())
        .into_iter()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (int(&cap[1]), int(&cap[2]), int(&cap[3]), int(&cap[4]))
        })
        .filter(is_any_pair_in_other)
        .count() as i32
}
