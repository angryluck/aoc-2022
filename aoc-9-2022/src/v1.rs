use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

use anyhow::{anyhow, Result};

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Motion {
    direction: Direction,
    steps: u32,
}

#[derive(Debug)]
struct Rope {
    head_location: (isize, isize),
    tail_location: (isize, isize),
    head_visited: HashSet<(isize, isize)>,
    tail_visited: HashSet<(isize, isize)>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head_location: (0, 0),
            tail_location: (0, 0),
            head_visited: HashSet::from([(0, 0)]),
            tail_visited: HashSet::from([(0, 0)]),
        }
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.head_location.1 += 1,
            Direction::Down => self.head_location.1 -= 1,
            Direction::Right => self.head_location.0 += 1,
            Direction::Left => self.head_location.0 -= 1,
        }
        self.move_tail();
        self.head_visited.insert(self.head_location.clone());
        self.tail_visited.insert(self.tail_location.clone());
    }

    fn movement(&mut self, motion: &Motion) {
        let direction = &motion.direction;
        for _ in 1..=motion.steps {
            self.step(direction)

        }
    }

    fn tail_visited_count(&self) -> usize {
        self.tail_visited.len()
    }

    fn move_tail(&mut self) {
        // Moves tail one step
        // Head Over/Right => delta positive, Head Under, Left => delta negative
        let delta_x = self.head_location.0 - self.tail_location.0;
        let delta_y = self.head_location.1 - self.tail_location.1;
        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            self.tail_location.0 += delta_x.signum();
            self.tail_location.1 += delta_y.signum();
        }
    }
}

impl TryFrom<String> for Motion {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        // Needs to be of the form: "D 14", or likewise
        let mut value = value.split_whitespace();
        let direction = value.next().ok_or(anyhow!("Empty string"))?;
        let direction = Direction::try_from(direction)?;
        let steps = value.next().ok_or(anyhow!("Nothing after whitespace"))?.parse()?;
        Ok(Motion{direction, steps})
    }
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let direction = match value {
            x if x == "U" => Direction::Up,
            x if x == "D" => Direction::Down,
            x if x == "R" => Direction::Right,
            x if x == "L" => Direction::Left,
            x => return Err(anyhow!("Not a valid direction: {}", x)),
        };
        Ok(direction)
    }
}

fn main() -> Result<()> {
    let result = run("input.txt")?;
    println!("Result is: {:?}", result);
    Ok(())
}

fn run(input: &str) -> Result<()> {
    let input = read_input(input)?;
    let movement_commands = parse_input(input)?;
    let mut rope = Rope::new();
    for command in movement_commands {
        rope.movement(&command);
        // dbg!(&rope);
    }
    println!("Answer is: {}", rope.tail_visited_count());
    Ok(())
}

fn read_input(input: &str) -> Result<Vec<String>> {
    let file = File::open(input)?;
    let buf = BufReader::new(file);
    buf.lines().map(|x| Ok(x?)).collect()
}

fn parse_input(input: Vec<String>) -> Result<Vec<Motion>> {
    input
        .into_iter()
        .map(|x| Ok(Motion::try_from(x)?))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn few_steps() -> Result<()> {
        let mut rope = Rope::new();
        rope.step(&Direction::Up);
        dbg!(&rope);
        rope.step(&Direction::Up);
        dbg!(&rope);
        rope.step(&Direction::Right);
        dbg!(&rope);
        rope.step(&Direction::Right);
        dbg!(&rope);
        panic!()
    }
}
