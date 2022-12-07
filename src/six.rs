use crate::util::{read_file, split_lines};
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, VecDeque};

#[test]
fn a() {
    assert_eq!(get_a("src/input/six_short.txt"), 7);
    assert_eq!(get_a("src/input/six.txt"), 1723)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/six_short.txt"), 19);
    assert_eq!(get_b("src/input/six.txt"), 3708)
}

fn get_packet_pos(message: &str, marker_size: usize) -> i32 {
    let mut buf = VecDeque::with_capacity(marker_size);

    for (i, c) in message.char_indices() {
        if buf.len() == marker_size {
            buf.pop_front();
        }
        buf.push_back(c);

        let set: HashSet<&char, RandomState> = HashSet::from_iter(buf.iter());
        if set.iter().count() == marker_size {
            return (i + 1) as i32;
        }
    }
    panic!("did not find it!")
}

pub fn get_b(file: &str) -> i32 {
    get_packet_pos(read_file(file).as_str(), 14)
}

pub fn get_a(file: &str) -> i32 {
    get_packet_pos(read_file(file).as_str(), 4)
}
