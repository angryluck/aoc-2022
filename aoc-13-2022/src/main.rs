// use anyhow::Result;
use std::fs;

use aoc_13_2022::do_part1;
use aoc_13_2022::do_part2;

fn main() {
    let input = fs::read_to_string("input.txt").expect("No such file");
    println!("Running part 1...");
    let result1 = do_part1(&input);
    println!("Part 1: Result is: {}", result1);
    println!("\nRunning part 2...");
    let result2 = do_part2(&input);
    println!("Part 2: Result is {}", result2);
}
