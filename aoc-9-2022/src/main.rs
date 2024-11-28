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
    // First is head, last is tail
    rope_location: [(isize, isize); 10],
    // tail_location: (isize, isize),
    // head_visited: HashSet<(isize, isize)>,
    tail_visited: HashSet<(isize, isize)>,
}

impl Rope {
    fn new() -> Self {
        Self {
            // Could make this more generalizable, but array length must be known at compile time,
            // and using a vector might produce some problems with multiple mut references at
            // the same time...
            rope_location: [(0, 0); 10],
            tail_visited: HashSet::from([(0, 0)]),
        }
    }

    fn step(&mut self, direction: &Direction) {
        let head = self.rope_location.first_mut().expect("Should not be empty");
        match direction {
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Left => head.0 -= 1,
        }
        for i in 1..10 {
            self.move_knot(i);
        }
        self.tail_visited.insert(
            self.rope_location
                .last()
                .expect("Should not be empty")
                .clone(),
        );
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

    fn move_knot(&mut self, index: usize) {
        let previous_knot = self.rope_location[index - 1];
        let knot = self
            .rope_location
            .get_mut(index)
            .expect("Should not be empty");
        let delta_x = previous_knot.0 - knot.0;
        let delta_y = previous_knot.1 - knot.1;
        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            knot.0 += delta_x.signum();
            knot.1 += delta_y.signum();
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
        let steps = value
            .next()
            .ok_or(anyhow!("Nothing after whitespace"))?
            .parse()?;
        Ok(Motion { direction, steps })
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
    println!("Result is: {}", result);
    Ok(())
}

fn run(input: &str) -> Result<usize> {
    let input = read_input(input)?;
    let movement_commands = parse_input(input)?;
    let mut rope = Rope::new();
    for command in movement_commands {
        rope.movement(&command);
        // dbg!(&rope);
    }
    // println!("Answer is: {}", rope.tail_visited_count());
    Ok(rope.tail_visited_count())
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
    fn small_sample() -> Result<()> {
        let result = run("test.txt")?;
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn small_sample_2() -> Result<()> {
        let result = run("test2.txt")?;
        assert_eq!(result, 36);
        Ok(())
    }

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
