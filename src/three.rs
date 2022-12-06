use crate::util::{read_file, split_lines};
use std::collections::HashSet;

#[test]
fn a() {
    assert_eq!(get_a(), 8109)
}

#[test]
fn b() {
    assert_eq!(get_b(), 2738)
}

pub fn get_badge(backpacks: Vec<&str>) -> char {
    let one = backpacks.get(0).unwrap();
    let two = backpacks.get(1).unwrap();
    let three = backpacks.get(2).unwrap();

    let mut a: HashSet<String> = HashSet::new();
    let mut b: HashSet<String> = HashSet::new();
    let mut c: HashSet<String> = HashSet::new();

    one.chars().for_each(|ch| {
        a.insert(ch.to_string());
    });
    two.chars().for_each(|ch| {
        b.insert(ch.to_string());
    });
    three.chars().for_each(|ch| {
        c.insert(ch.to_string());
    });

    (&(&a & &b) & &c)
        .into_iter()
        .next()
        .unwrap()
        .clone()
        .chars()
        .next()
        .unwrap()
}

fn get_priority(n: char) -> i32 {
    if n.is_uppercase() {
        return 27 + (n as i32) - ('A' as i32);
    }

    1 + (n as i32) - ('a' as i32)
}

pub fn get_b() -> i32 {
    split_lines(read_file("src/input/three.txt").as_str())
        .chunks(3)
        .map(|v| get_badge(v.iter().map(|bp| *bp).collect::<Vec<&str>>()))
        .map(get_priority)
        .sum()
}

pub fn get_compartment_intersection(compartment: &str) -> char {
    let (first, second) = compartment.split_at(compartment.len() / 2);

    let mut a: HashSet<String> = HashSet::new();
    let mut b: HashSet<String> = HashSet::new();

    first.chars().for_each(|c| {
        a.insert(c.to_string());
    });
    second.chars().for_each(|c| {
        b.insert(c.to_string());
    });

    a.clone()
        .intersection(&b.clone())
        .next()
        .unwrap()
        .clone()
        .chars()
        .next()
        .unwrap()
}

pub fn get_a() -> i32 {
    split_lines(read_file("src/input/three.txt").as_str())
        .into_iter()
        .map(get_compartment_intersection)
        .map(get_priority)
        .sum()
}
