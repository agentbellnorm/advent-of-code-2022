use crate::util::{char_at, int, read_file, split_lines};
use regex::Regex;
use std::collections::HashMap;

#[test]
fn a() {
    assert_eq!(get_a("src/input/seven_short.txt"), 95437);
    assert_eq!(get_a("src/input/seven.txt"), 1427048);
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/seven_short.txt"), 24933642);
    assert_eq!(get_b("src/input/seven.txt"), 2940614);
}

#[test]
fn pwd_test() {
    assert_eq!(current_dir("/hej/grej/tjej"), "tjej");
    assert_eq!(current_dir("/hej/grej/tjej"), "tjej");
    assert_eq!(push("/hej/svej", "grej"), "/hej/svej/grej");
    assert_eq!(pop("/hej/svej/grej"), "/hej/svej");
    assert_eq!(pop("/hej"), "/");

    assert_eq!(all_dirs("/hej"), vec!["/hej", "/"]);
    assert_eq!(
        all_dirs("/hej/svej/grej"),
        vec!["/hej/svej/grej", "/hej/svej", "/hej", "/"]
    );
}

fn current_dir(pwd: &str) -> &str {
    pwd.split("/")
        .last()
        .expect(format!("could not current_dir on {}", pwd).as_str())
}

fn push(pwd: &str, dir: &str) -> String {
    return if pwd == "/" {
        format!("/{}", dir)
    } else {
        format!("{}/{}", pwd, dir)
    };
}

fn pop(pwd: &str) -> String {
    match pwd.matches("/").count() {
        1 => "/".to_string(),
        0 => panic!("wtf"),
        _ => {
            let mut v = pwd.split("/").collect::<Vec<&str>>();
            v.pop();
            v.join("/")
        }
    }
}

fn all_dirs(pwd: &str) -> Vec<String> {
    let mut dirs = Vec::new();
    let mut remaining_dirs = pwd.to_string();
    while remaining_dirs.len() > 1 {
        dirs.push(remaining_dirs.to_string());
        remaining_dirs = pop(remaining_dirs.as_str());
    }
    dirs.push(remaining_dirs);
    dirs
}

fn df<'a>(lines: &'a Vec<&'a str>) -> HashMap<String, i32> {
    let size_pat = Regex::new(r"(\d+)").unwrap();
    let cd_pat = Regex::new(r"cd (.+)").unwrap();

    let mut pwd: String = "".to_string();

    let mut file_tree: HashMap<String, i32> = HashMap::new();

    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        let size_match = size_pat.captures(line);
        let cd_match = cd_pat.captures(line);

        if size_match.is_some() {
            let size = int(&size_match.unwrap()[1]);
            for d in all_dirs(pwd.as_str()) {
                let curr_size = file_tree.entry(d).or_insert(0);
                *curr_size += size;
            }
        } else if cd_match.is_some() {
            let target = &cd_match.unwrap()[1];
            if target == ".." {
                if pwd.len() > 1 && !current_dir(pwd.as_str()).eq("/") {
                    pwd = pop(pwd.as_str());
                }
            } else if target == "/" {
                pwd = "/".to_string();
            } else {
                pwd = push(pwd.as_str(), target);
            }
        }
    }

    file_tree
}

pub fn get_a(file: &str) -> i32 {
    let parsed = read_file(file);
    let lines = split_lines(parsed.as_str());
    let fs = df(&lines);

    fs.into_iter()
        .filter(|(_, size)| size <= &100000)
        .map(|(_, size)| size)
        .sum()
}

pub fn get_b(file: &str) -> i32 {
    let parsed = read_file(file);
    let lines = split_lines(parsed.as_str());

    let dirs = df(&lines);
    let disk_size = 70000000;
    let needed_space = 30000000;
    let used_space = dirs.get("/").unwrap();
    let free_space = disk_size - used_space;

    dirs.clone()
        .into_iter()
        .filter(|(_, size)| free_space + size > needed_space)
        .map(|(_, size)| size)
        .min()
        .unwrap()
}
