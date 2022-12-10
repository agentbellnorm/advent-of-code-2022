use crate::util::{ints_c, read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a("src/input/eight_short.txt"), 21);
    assert_eq!(get_a("src/input/eight.txt"), 1805)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/eight_short.txt"), 8);
    assert_eq!(get_b("src/input/eight.txt"), 444528)
}

fn is_on_edge(size: i32, (x, y): (i32, i32)) -> bool {
    x == 0 || y == 0 || x == size - 1 || y == size - 1
}

fn get_sectors_to_check(trees: &Vec<Vec<i32>>, (x, y): (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    let size = trees.len() as i32;
    let mut coords = Vec::new();

    coords.push(
        (0..y)
            .into_iter()
            .map(|y_coord| (x, y_coord))
            .rev()
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
            .rev()
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

fn get_tree_height(trees: &Vec<Vec<i32>>, (x, y): &(i32, i32)) -> i32 {
    trees
        .get(*y as usize)
        .unwrap()
        .get(*x as usize)
        .unwrap()
        .clone()
}

fn is_visible(trees: &Vec<Vec<i32>>, tree: (i32, i32)) -> bool {
    let this_tree_height = get_tree_height(trees, &tree);
    let len = trees.len() as i32;

    if is_on_edge(len, tree) {
        return true;
    }

    get_sectors_to_check(trees, tree).iter().any(|sector| {
        sector
            .iter()
            .all(|other_tree| this_tree_height > get_tree_height(trees, other_tree))
    })
}

fn get_score(trees: &Vec<Vec<i32>>, tree: (i32, i32)) -> i32 {
    let this_tree_height = get_tree_height(trees, &tree);

    get_sectors_to_check(trees, tree)
        .into_iter()
        .map(|sector| {
            let mut sector_score = 0;
            for other_tree in &sector {
                sector_score += 1;
                let other_height = get_tree_height(trees, &other_tree);
                if other_height >= this_tree_height {
                    break;
                }
            }
            sector_score
        })
        .fold(1, |acc, v| acc * v)
}

fn get_tree(lines: Vec<&str>) -> Vec<Vec<i32>> {
    lines
        .into_iter()
        .map(|line| ints_c(line.chars()))
        .collect::<Vec<Vec<i32>>>()
}

pub fn get_b(file: &str) -> i32 {
    let trees = get_tree(split_lines(read_file(file).as_str()));
    let size = trees.len();
    let mut max_score = 0;

    for x in 0..size {
        for y in 0..size {
            max_score = i32::max(max_score, get_score(&trees, (x as i32, y as i32)));
        }
    }

    max_score
}

pub fn get_a(file: &str) -> i32 {
    let trees = get_tree(split_lines(read_file(file).as_str()));
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
