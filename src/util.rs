use std::fmt::Debug;

pub fn split_lines(s: &str) -> Vec<&str> {
    s.split("\n").collect()
}

pub fn char_at(str: &str, i: i32) -> char {
    str.chars().nth(i as usize).unwrap()
}

pub fn log_debug(debugable: &impl Debug) {
    println!(&format!("{:?}", debugable).into());
}

pub fn int(string: &str) -> i32 {
    match string.parse::<i32>() {
        Ok(number) => number,
        Err(_) => panic!("Could not parse {:?} to i32", string),
    }
}

pub fn int_big(string: &str) -> i64 {
    match string.parse::<i64>() {
        Ok(number) => number,
        Err(_) => panic!("Could not parse {:?} to i64", string),
    }
}

pub fn get_index((x, y): (i32, i32), n_cols: i32) -> i32 {
    (y * n_cols) + x
}

pub fn count_in_vec<T: Eq>(v: &Vec<T>, item: T) -> i32 {
    v.into_iter().filter(|i| item.eq(i)).count() as i32
}

pub fn print_ret<T: Debug>(v: T) -> T {
    log_debug(&v);
    v
}

pub fn log_all_items<T: Debug>(v: &Vec<T>) {
    for i in 0..v.len() {
        log_debug(&v.get(i).unwrap());
    }
}
