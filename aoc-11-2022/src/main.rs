use anyhow::Result;
use aoc_11_2022::do_part2;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").expect("No such file");
    let result = do_part2(&input)?;
    println!("The level of monkey business is: {}", result);
    Ok(())
}
