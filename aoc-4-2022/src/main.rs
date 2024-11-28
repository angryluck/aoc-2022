use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug)]
struct Elf(u32, u32);

impl<S: Into<String>> From<S> for Elf {
    // Only accepts input of the form "x-y"
    fn from(value: S) -> Self {
        let (start, end) = value
            .into()
            .split('-')
            .map(|x| x.parse::<u32>().expect("Should not fail"))
            .tuples()
            .next()
            .expect("Should not fail");

        Elf(start, end)
    }
}

fn main() {
    let input = read_input().expect("Should be able to read file");
    let input = parse_input(input);
    println!("{}", pairs_overlap(input));
}

fn read_input() -> Result<Vec<String>, std::io::Error> {
    // Should work for any AOC project
    let file = std::fs::File::open("input.txt")?;
    let buf = std::io::BufReader::new(file);
    buf.lines().collect()
}

fn parse_line(line: String) -> (Elf, Elf) {
    let mut line = line.split(',');
    (
        Elf::from(line.next().expect("No first elf")),
        Elf::from(line.next().expect("No second elf")),
    )
}

fn parse_input(input: Vec<String>) -> Vec<(Elf, Elf)> {
    input.into_iter().map(|l| parse_line(l)).collect()
}

fn _contained(elf1: &Elf, elf2: &Elf) -> bool {
    elf1.0 <= elf2.0 && elf1.1 >= elf2.1 || elf1.0 >= elf2.0 && elf1.1 <= elf2.1
}

fn overlaps(elf1: &Elf, elf2: &Elf) -> bool {
    elf2.0 <= elf1.0 && elf1.0 <= elf2.1
        || elf2.0 <= elf1.1 && elf1.0 <= elf2.1
        || elf1.0 <= elf2.0 && elf2.0 <= elf1.1
        || elf1.0 <= elf2.1 && elf2.0 <= elf1.1
}

fn _pairs_contained(pairs: Vec<(Elf, Elf)>) -> usize {
    pairs
        .iter()
        .filter(|(elf1, elf2)| _contained(elf1, elf2))
        .count()
}

fn pairs_overlap(pairs: Vec<(Elf, Elf)>) -> usize {
    pairs
        .iter()
        .filter(|(elf1, elf2)| overlaps(elf1, elf2))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample_contains() {
        let input = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
        let input: Vec<String> = input.split('\n').map(|l| l.to_owned()).collect();
        dbg!(&input);
        let input = parse_input(input);
        dbg!(&input);

        assert_eq!(_pairs_contained(input), 2)
    }

    #[test]
    fn small_sample_overlaps() {
        let input = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
        let input: Vec<String> = input.split('\n').map(|l| l.to_owned()).collect();
        dbg!(&input);
        let input = parse_input(input);
        dbg!(&input);

        assert_eq!(pairs_overlap(input), 4)
    }
}
