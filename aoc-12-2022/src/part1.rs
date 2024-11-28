// Using Dijkstra's algorithm!
use std::collections::{HashMap, HashSet};

fn _pretty_print_distance(distance_from_start: &HashMap<(isize, isize), usize>) {
    let mut output = String::new();
    let mut x = 0;
    let mut y = 0;
    while let Some(_) = distance_from_start.get(&(x, y)) {
        while let Some(val) = distance_from_start.get(&(x, y)) {
            let mut val = val.to_string();
            if val.len() > 3 {
                val = String::from("MAX");
            }
            output.push_str(&val);
            for _ in 0..4 - val.len() {
                output.push(' ');
            }

            y += 1
        }
        output.push('\n');
        y = 0;
        x += 1
    }
    println!("{}", output);
}

fn can_step_to(from: char, to: char) -> bool {
    if from == 'S' {
        return true;
    };
    if from == 'E' || to == 'S' {
        return false;
    };
    if to == 'E' {
        from == 'y' || from == 'z'
    } else if from.is_lowercase() && to.is_lowercase() {
        (from as u32) >= (to as u32 - 1)
    } else {
        panic!("Not a valid character: from: {}, to: {}", from, to)
    }
}

fn update_distance(
    current_distance: usize,
    target_index: (isize, isize),
    distance_from_start: &mut HashMap<(isize, isize), usize>,
) {
    distance_from_start
        .entry(target_index)
        .or_insert(current_distance);
}

fn around_index(
    index: (isize, isize),
    squares: &HashMap<(isize, isize), char>,
    visited_indices: &HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    let mut indices_around = HashSet::new();
    let current_height = squares.get(&index).expect("Should be a valid index");

    let (x, y) = index;
    for (i, j) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
        if let Some(to_height) = squares.get(&(x + i, y + j)) {
            if !visited_indices.contains(&(x + i, y + j))
                && can_step_to(*current_height, *to_height)
            {
                indices_around.insert((x + i, y + j));
            }
        }
    }
    indices_around
}

fn around_indices(
    indices: HashSet<(isize, isize)>,
    squares: &HashMap<(isize, isize), char>,
    visited_indices: &HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    indices.into_iter().fold(HashSet::new(), |acc, index| {
        &acc | &around_index(index, squares, visited_indices)
    })
}

fn one_step(
    mut current_distance: usize,
    mut current_indices: HashSet<(isize, isize)>,
    mut visited_indices: HashSet<(isize, isize)>,
    distance_from_start: &mut HashMap<(isize, isize), usize>,
    squares: &HashMap<(isize, isize), char>,
) -> (usize, HashSet<(isize, isize)>, HashSet<(isize, isize)>) {
    // PSEUDOPLAN:
    // 1. +1 to current distance
    // 2. add current_indices to visited_indeces
    // 3. replace current_indices by visitable indices
    // 4. update distance of each current_indice
    // 5. return (current_distance, current_indices, visited_indeces)

    current_distance += 1;
    visited_indices.extend(&current_indices);
    current_indices = around_indices(current_indices, squares, &visited_indices);
    for index in current_indices.iter() {
        update_distance(current_distance, *index, distance_from_start);
    }
    // println!(
    //     "After doing step {}, the distances look as follows:\n",
    //     current_distance
    // );
    // _pretty_print_distance(&distance_from_start);
    // println!("The indices are: {:?}", &current_indices);
    (current_distance, current_indices, visited_indices)
}

pub fn do_part1(input: &str) -> usize {
    let mut squares = block_parser(input);

    // let mut distance_from_start: HashMap<(isize, isize), usize> = squares
    //     .iter()
    //     .map(|(key, _val)| (*key, 1_000_000))
    //     .collect();

    let start = *squares.iter().find(|(_key, &val)| val == 'S').unwrap().0;
    let end = *squares.iter().find(|(_key, &val)| val == 'E').unwrap().0;
    let mut distance_from_start = HashMap::from([(start, 0)]);

    // *distance_from_start.get_mut(&start).unwrap() = 0;
    // let end = squares.iter().find(|(_key, &val)| val == 'E').unwrap().0;
    // start
    let mut current_distance = 0;
    let mut current_indices = HashSet::from([start]);
    let mut visited_indices = HashSet::new();
    // for _ in 0..40 {
    while !current_indices.is_empty() {
        (current_distance, current_indices, visited_indices) = one_step(
            current_distance,
            current_indices,
            visited_indices,
            &mut distance_from_start,
            &mut squares,
        );
    }
    distance_from_start[&end]
}

fn block_parser(input: &str) -> HashMap<(isize, isize), char> {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(row, line)| line_parser(row, line))
        .collect()
}

fn line_parser(row: usize, input: &str) -> impl Iterator<Item = ((isize, isize), char)> + '_ {
    input
        .chars()
        .enumerate()
        .map(move |(col, c)| ((row as isize, col as isize), c))
}

#[cfg(test)]
mod tests {
    use super::*;
    // use anyhow::Result;

    const _INPUT_SMALL: &str = "Sab
abc
acE";

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn debug_parser() {
        dbg!(do_part1(INPUT));
        panic!();
    }

    #[test]
    fn around_indices_works() {}

    #[test]
    fn step_to_works() {
        assert_eq!(can_step_to('a', 'b'), true);
        assert_eq!(can_step_to('e', 'p'), false);
        assert_eq!(can_step_to('k', 'k'), true);
        assert_eq!(can_step_to('S', 'b'), true);
        assert_eq!(can_step_to('S', 'E'), true);
        assert_eq!(can_step_to('f', 'E'), true);
    }
    #[test]
    #[should_panic]
    fn step_to_panics() {
        can_step_to('a', 'K');
    }

    #[test]
    fn accessing_hashmap_works() {
        let mut test = HashMap::from([(1, 69), (2, 420)]);
        test.entry(2).or_insert(333);
        test.entry(69).or_insert(333);
        // test[&2] = 42069;
        dbg!(test);
        panic!();
    }
}
