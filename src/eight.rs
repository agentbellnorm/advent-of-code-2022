use crate::util::{ints, ints_c, read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a("src/input/eight_short.txt"), 21);
    assert_eq!(get_a("src/input/eight.txt"), 0)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/eight_short.txt"), 0);
    assert_eq!(get_b("src/input/eight.txt"), 0)
}

fn get_coords(index: i32, width: i32) -> (i32, i32) {
    let x = index % width;
    (x, (index - x) / width)
}

fn is_on_edge(size: i32, (x, y): (i32, i32)) -> bool {
    x == 0 || y == 0 || x == size - 1 || y == size - 1
}

fn get_sectors_to_check(trees: &Vec<Vec<i32>>, (x, y): (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    let size = trees.len() as i32;
    if is_on_edge(size, (x, y)) {
        return vec![];
    }

    let mut coords = Vec::new();

    coords.push(
        (0..y)
            .into_iter()
            .map(|y_coord| (x, y_coord))
            .collect::<Vec<(i32, i32)>>(),
    );

    coords.push(
        ((y + 1)..size)
            .into_iter()
            .map(|y_coord| (x, y_coord))
            .collect::<Vec<(i32, i32)>>(),
    );

    coords.push(
        (0..x)
            .into_iter()
            .map(|x_coord| (x_coord, y))
            .collect::<Vec<(i32, i32)>>(),
    );

    coords.push(
        ((x + 1)..size)
            .into_iter()
            .map(|x_coord| (x_coord, y))
            .collect::<Vec<(i32, i32)>>(),
    );

    coords
}

fn get_tree_height(trees: &Vec<Vec<i32>>, (x, y): (i32, i32)) -> i32 {
    trees
        .get(y as usize)
        .unwrap()
        .get(x as usize)
        .unwrap()
        .clone()
}

fn is_visible(trees: &Vec<Vec<i32>>, (x, y): (i32, i32)) -> bool {
    let this_tree_height = get_tree_height(trees, (x, y));
    let len = trees.len() as i32;

    if is_on_edge(len, (x, y)) {
        return true;
    }

    get_sectors_to_check(trees, (x, y)).iter().any(|sector| {
        sector.iter().all(|(x_coord, y_coord)| {
            let other_tree_height = get_tree_height(trees, (*x_coord, *y_coord));
            this_tree_height > other_tree_height
        })
    })
}

pub fn get_b(file: &str) -> i32 {
    split_lines(read_file(file).as_str()).into_iter().len() as i32
}

pub fn get_a(file: &str) -> i32 {
    let trees = split_lines(read_file(file).as_str())
        .into_iter()
        .map(|line| ints_c(line.chars()))
        .collect::<Vec<Vec<i32>>>();

    // assume square
    let size = trees.len();

    let mut n_visible = 0;

    for x in 0..size {
        for y in 0..size {
            if is_visible(&trees, (x as i32, y as i32)) {
                n_visible += 1
            }
        }
    }

    n_visible
}
