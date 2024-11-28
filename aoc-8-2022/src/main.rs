use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn main() -> Result<()> {
    let result = run("input.txt")?;
    println!("{}", result);
    Ok(())
}

fn run(input: &str) -> Result<usize> {
    let input = read_input(input)?;
    let input = parse_input(input);
    Ok(max_scenic_score(input))
}

fn read_input(input: &str) -> Result<Vec<String>> {
    let file = File::open(input)?;
    let buf = BufReader::new(file);
    buf.lines().map(|x| Ok(x?)).collect()
}

fn parse_input(input: Vec<String>) -> HashMap<(isize, isize), u32> {
    (0..).zip(input
        .into_iter())
        .map(|(row, line)| (parse_tree_line(row, line).into_iter()))
        .flatten()
        .collect()
}

fn parse_tree_line(row: isize, line: String) -> Vec<((isize, isize), u32)> {
    (0..).zip(line.chars())
        .map(move |(i, c)| ((row, i), c.to_digit(10).unwrap()))
        .collect::<Vec<_>>()
}

fn visible_trees(
    direction: Direction,
    mut index: (isize, isize),
    trees: &HashMap<(isize, isize), u32>,
) -> usize {
    let mut count = 0;
    let tree_height = trees[&index];
    loop {
        match direction {
            Direction::Right => index.0 += 1,
            Direction::Up => index.1 += 1,
            // Way to do this, if using (usize, usize) as index, rather than (isize, isize).
            Direction::Left => match index.0.checked_sub(1) {
                Some(x) => index.0 -= x,
                None => break,
            }
            // Way that only works when using (isize, isize)
            Direction::Down => index.1 -= 1,
        }
        match trees.get(&index) {
            Some(x) if *x < tree_height => {
                count += 1;
                continue;
            }
            Some(_) => {
                count += 1;
                break;
            }
            None => break,
        }
    }
    count
}

fn max_scenic_score(trees: HashMap<(isize, isize), u32>) -> usize {
    trees
        .keys()
        .map(|&i| scenic_score(i, &trees))
        .max()
        .expect("Should not be empty")
}

fn scenic_score(index: (isize, isize), trees: &HashMap<(isize, isize), u32>) -> usize {
    let right = visible_trees(Direction::Right, index, trees);
    let left = visible_trees(Direction::Left, index, trees);
    let over = visible_trees(Direction::Up, index, trees);
    let under = visible_trees(Direction::Down, index, trees);

    right * left * over * under
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() -> Result<()> {
        let result = run("test.txt")?;
        assert_eq!(8, result);
        Ok(())
    }

    #[test]
    fn can_read_input() -> Result<()> {
        let input = read_input("input.txt")?;
        dbg!(input);
        panic!()
    }

    #[test]
    fn can_parse_input() -> Result<()> {
        let input = read_input("test.txt")?;
        let input = parse_input(input);
        dbg!(input);
        panic!()
    }
}
