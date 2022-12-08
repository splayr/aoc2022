const INPUT: &str = include_str!("./input.txt");

pub fn dec8_part_one() {
    let trees = parse_tree_grid_from_input(INPUT);
    let visible_trees = check_number_of_visible_trees(trees.as_slice());
    println!(
        "number of visible trees : {}",
        visible_trees.0 + visible_trees.1
    )
}

pub fn dec8_part_two() {
    let trees = parse_tree_grid_from_input(INPUT);
    let max_scenic_score = get_maximum_scenic_score(trees.as_slice());
    println!("maximum scenic score : {max_scenic_score}")
}

fn parse_tree_grid_from_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn check_number_of_visible_trees(trees: &[Vec<u8>]) -> (u16, u16) {
    let mut edge_count = 0;
    let mut count = 0;
    for y in 0..trees.len() {
        for x in 0..trees[0].len() {
            let current_tree = trees[y][x];

            if x == 0 || x == trees.len() - 1 || y == 0 || y == trees.len() - 1 {
                dbg!("tree at edge");
                edge_count += 1;
                continue;
            }

            let is_visible_from_left = trees
                .get(y)
                .unwrap()
                .iter()
                .enumerate()
                .filter(|tree| tree.0 < x)
                .all(|tree| tree.1 < &current_tree);

            if is_visible_from_left {
                dbg!("visible from left");
                count += 1;
                continue;
            }

            let is_visible_from_right = trees
                .get(y)
                .unwrap()
                .iter()
                .enumerate()
                .filter(|tree| tree.0 > x)
                .all(|tree| tree.1 < &current_tree);

            if is_visible_from_right {
                dbg!("visible from right");
                count += 1;
                continue;
            }

            let is_visible_from_up = trees
                .iter()
                .map(|y_trees| y_trees[x])
                .enumerate()
                .filter(|tree| tree.0 < y)
                .all(|tree| tree.1 < current_tree);

            if is_visible_from_up {
                dbg!("visible from up");
                count += 1;
                continue;
            }

            let is_visible_from_down = trees
                .iter()
                .map(|y_trees| y_trees[x])
                .enumerate()
                .filter(|tree| tree.0 > y)
                .all(|tree| tree.1 < current_tree);

            if is_visible_from_down {
                dbg!("visible from down");
                count += 1;
                continue;
            }

            dbg!("not visible");
        }
    }

    (edge_count, count)
}

fn get_scenic_score(trees: &[Vec<u8>], x: usize, y: usize) -> u64 {
    let current_tree = trees[y][x];
    let max_x = trees.len() - 1;
    let max_y = trees[0].len() - 1;
    //dbg!(current_tree, max_x, max_y);

    let left_distance = 1 + trees[y]
        .iter()
        .enumerate()
        .filter(|tree| tree.0 < x)
        .rev()
        .position(|tree| &current_tree <= tree.1)
        .unwrap_or(x - 1);

    //dbg!(left_distance);

    let right_distance = 1 + trees[y]
        .iter()
        .enumerate()
        .filter(|tree| tree.0 > x)
        .position(|tree| &current_tree <= tree.1)
        .unwrap_or(max_x - x - 1);

    //dbg!(right_distance);

    let up_distance = 1 + trees
        .iter()
        .map(|y_trees| y_trees[x])
        .enumerate()
        .filter(|tree| tree.0 < y)
        .rev()
        .position(|tree| current_tree <= tree.1)
        .unwrap_or(y - 1);

    //dbg!(up_distance);

    let down_distance = 1 + trees
        .iter()
        .map(|y_trees| y_trees[x])
        .enumerate()
        .filter(|tree| tree.0 > y)
        .position(|tree| current_tree <= tree.1)
        .unwrap_or(max_y - y - 1);

    //dbg!(down_distance);

    left_distance as u64 * right_distance as u64 * up_distance as u64 * down_distance as u64
}

fn get_maximum_scenic_score(trees: &[Vec<u8>]) -> u64 {
    let max_x = trees[0].len() - 1;
    let max_y = trees.len() - 1;
    let mut max = 0;

    'y: for y in 0..trees.len() {
        if y == 0 || y == max_y { continue 'y }
        'x: for x in 0..trees[0].len() {
            if x == 0 || x == max_x { continue 'x }

            let score = get_scenic_score(trees, x, y);
            if score > max {
                max = score
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::{check_number_of_visible_trees, get_scenic_score, parse_tree_grid_from_input};

    const TEST_INPUT: &str = include_str!("./test_input.txt");

    #[test]
    fn correct_amount_of_visible_trees() {
        let (expected_at_edges, expected_others) = (16, 5);

        let trees = parse_tree_grid_from_input(TEST_INPUT);

        dbg!(trees.get(0));

        let (at_edges, others) = check_number_of_visible_trees(trees.as_slice());

        assert_eq!(expected_at_edges, at_edges);
        assert_eq!(expected_others, others);
    }

    #[test]
    fn scenic_score_is_correct() {
        let trees = parse_tree_grid_from_input(TEST_INPUT);

        get_scenic_score(trees.as_slice(), 2, 2);
        //dbg!(get_scenic_score(trees.as_slice(), 2, 3));
    }
}
