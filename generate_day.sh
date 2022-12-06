DAY=$1

echo "generating day $DAY skeleton"

echo "
use crate::util::{ints, log_debug, read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a(), 0)
}

#[test]
fn b() {
    assert_eq!(get_b(), 0)
}

pub fn get_b() -> i32 {
    let mut input = read_file(\"src/input/$DAY\_short.txt\");

    1
}

pub fn get_a() -> i32 {
    let mut input = read_file(\"src/input/$DAY\_short.txt\");

    1
}
" > src/$DAY.rs

echo "mod $DAY;\n$(cat src/main.rs)" > src/main.rs
touch src/input/$DAY.txt
touch src/input/$DAY_short.txt

cargo test