use crate::util::{ints, read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a(), 71502)
}

#[test]
fn b() {
    assert_eq!(get_b(), 208191)
}

pub fn get_b() -> i32 {
    let mut numbers = read_file("src/input/one.txt")
        .split("\n\n")
        .into_iter()
        .map(|elf_group| ints(split_lines(elf_group)).iter().sum())
        .collect::<Vec<i32>>();

    numbers.sort();
    numbers.reverse();

    numbers[0..3].into_iter().sum()
}

pub fn get_a() -> i32 {
    read_file("src/input/one.txt")
        .split("\n\n")
        .into_iter()
        .map(|elf_group| ints(split_lines(elf_group)).iter().sum())
        .max()
        .unwrap()
}
