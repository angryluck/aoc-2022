use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

fn main() -> Result<()> {
    let result = run("input.txt")?;
    println!("{}", result);
    Ok(())
}

fn run(input: &str) -> Result<usize> {
    let input = read_input(input)?;
    let input = parse_input(input);
    Ok(visible_trees(input))
}

fn read_input(input: &str) -> Result<Vec<String>> {
    let file = File::open(input)?;
    let buf = BufReader::new(file);
    buf.lines().map(|x| Ok(x?)).collect()
}

fn parse_input(input: Vec<String>) -> HashMap<(usize, usize), u32> {
    input
        .into_iter()
        .enumerate()
        .map(|(row, line)| (parse_tree_line(row, line).into_iter()))
        .flatten()
        .collect()
}

fn visible_trees(trees: HashMap<(usize, usize), u32>) -> usize {
    trees.keys().filter(|i| is_visible(i, &trees)).count()
}

fn is_visible(index: &(usize, usize), trees: &HashMap<(usize, usize), u32>) -> bool {
    let tree_height = trees[index];
    let right = trees.iter()
        .filter(|((x, y), _)| *x > index.0 && *y == index.1)
        .all(|((_, _), val)| *val < tree_height);
    let left = trees.iter()
        .filter(|((x, y), _)| *x < index.0 && *y == index.1)
        .all(|((_, _), val)| *val < tree_height);
    let over = trees.iter()
        .filter(|((x, y), _)| *x == index.0 && *y > index.1)
        .all(|((_, _), val)| *val < tree_height);
    let under = trees.iter()
        .filter(|((x, y), _)| *x == index.0 && *y < index.1)
        .all(|((_, _), val)| *val < tree_height);
    right || left || over || under 
}

fn parse_tree_line(row: usize, line: String) -> Vec<((usize, usize), u32)> {
    line.chars()
        .enumerate()
        .map(move |(i, c)| ((row, i), c.to_digit(10).unwrap()))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() -> Result<()> {
        let result = run("test.txt")?;
        assert_eq!(21, result);
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
