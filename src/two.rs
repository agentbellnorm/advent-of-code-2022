use crate::util::{read_file, split_lines};
use std::collections::HashMap;

#[test]
fn a() {
    assert_eq!(get_a(), 13005)
}

#[test]
fn b() {
    assert_eq!(get_b(), 11373)
}

fn get_game_score(game: &str) -> i32 {
    let rock = 1;
    let paper = 2;
    let scissor = 3;
    let win = 6;
    let tie = 3;
    let loss = 0;

    let map = HashMap::from([
        ("A X", tie + rock),
        ("A Y", win + paper),
        ("A Z", loss + scissor),
        ("B X", loss + rock),
        ("B Y", tie + paper),
        ("B Z", win + scissor),
        ("C X", win + rock),
        ("C Y", loss + paper),
        ("C Z", tie + scissor),
    ]);

    map.get(game)
        .expect(format!("no result for game {}", game).as_str())
        .clone()
}

fn get_secret_strategy_score(game: &str) -> i32 {
    let rock = 1;
    let paper = 2;
    let scissor = 3;
    let win = 6;
    let tie = 3;
    let loss = 0;

    let map = HashMap::from([
        ("A X", loss + scissor),
        ("A Y", tie + rock),
        ("A Z", win + paper),
        ("B X", loss + rock),
        ("B Y", tie + paper),
        ("B Z", win + scissor),
        ("C X", loss + paper),
        ("C Y", tie + scissor),
        ("C Z", win + rock),
    ]);

    map.get(game)
        .expect(format!("no result for game {}", game).as_str())
        .clone()
}

pub fn get_b() -> i32 {
    split_lines(read_file("src/input/two.txt").as_str())
        .into_iter()
        .map(get_secret_strategy_score)
        .sum()
}

pub fn get_a() -> i32 {
    split_lines(read_file("src/input/two.txt").as_str())
        .into_iter()
        .map(get_game_score)
        .sum()
}
