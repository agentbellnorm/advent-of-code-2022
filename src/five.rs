use crate::util::{char_at, char_at_opt, int, read_file, split_lines};
use regex::Regex;

#[test]
fn a() {
    assert_eq!(get_a("src/input/five_short.txt"), "CMZ");
    assert_eq!(get_a("src/input/five.txt"), "VCTFTJQCG")
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/five_short.txt"), "MCD");
    assert_eq!(get_b("src/input/five.txt"), "GCFGLDNJZ")
}

fn get_initial_stacks(crates: &str, num_stacks: i32) -> Vec<Vec<char>> {
    let mut stacks = vec![vec![]; num_stacks as usize];
    let mut lines = split_lines(crates);
    lines.reverse();

    let mut lin = lines.iter();

    let first = lin.next().unwrap();

    for line in lin {
        for i in 1..first.len() {
            let letter = char_at_opt(line, i as i32);
            if letter.is_none() {
                continue;
            }
            let confirmed_letter = letter.unwrap();
            if confirmed_letter.is_alphabetic() {
                let crate_nr = int(&char_at(first, i as i32).to_string());
                stacks
                    .get_mut((crate_nr - 1) as usize)
                    .unwrap()
                    .push(confirmed_letter);
            }
        }
    }

    stacks
}

fn get_instructions(raw: &str) -> Vec<(i32, i32, i32)> {
    let instr_pat = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    split_lines(raw)
        .into_iter()
        .map(|line| {
            let captures = instr_pat.captures(line).unwrap();
            (
                int(&captures[1]),
                int(&captures[2]) - 1,
                int(&captures[3]) - 1,
            )
        })
        .collect::<Vec<(i32, i32, i32)>>()
}

pub fn get_b(file: &str) -> String {
    let num_crates_pat = Regex::new(r"\s{3}(\d+)\n\n").unwrap();
    let file_content = read_file(file);
    let num_crates = int(&num_crates_pat.captures(file_content.as_str()).unwrap()[1]);
    let halves = file_content.split("\n\n").collect::<Vec<&str>>();
    let (crates, instructions) = (&halves[0], &halves[1]);
    let mut stacks = get_initial_stacks(crates, num_crates);

    let parsed_instructions = get_instructions(instructions);

    for (n, from, to) in parsed_instructions {
        let mut move_stack: Vec<char> = Vec::new();

        for _ in 0..n {
            let to_move = stacks.get_mut(from as usize).unwrap().pop().unwrap();
            move_stack.push(to_move)
        }

        for _ in 0..n {
            stacks
                .get_mut(to as usize)
                .unwrap()
                .push(move_stack.pop().unwrap());
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect::<String>()
}

pub fn get_a(file: &str) -> String {
    let num_crates_pat = Regex::new(r"\s{3}(\d+)\n\n").unwrap();
    let file_content = read_file(file);
    let num_crates = int(&num_crates_pat.captures(file_content.as_str()).unwrap()[1]);
    let halves = file_content.split("\n\n").collect::<Vec<&str>>();
    let (crates, instructions) = (&halves[0], &halves[1]);
    let mut stacks = get_initial_stacks(crates, num_crates);

    let parsed_instructions = get_instructions(instructions);

    for (n, from, to) in parsed_instructions {
        for _ in 0..n {
            let to_move = stacks.get_mut(from as usize).unwrap().pop().unwrap();
            stacks.get_mut(to as usize).unwrap().push(to_move);
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect::<String>()
}
