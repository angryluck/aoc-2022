use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};
use itertools::Itertools;

#[derive(Debug)]
struct RucksackOld(HashSet<char>, HashSet<char>);

struct Rucksack(HashSet<char>);

struct ElfGroup(Rucksack, Rucksack, Rucksack);

impl<S: Into<String>> From<S> for Rucksack {
    fn from(value: S) -> Self {
        let items = value.into().chars().collect();
        Rucksack(items)
    }
}

impl ElfGroup {
    fn badge(mut self) -> char {
        self.0.0.retain(|item| {
            self.1.0.contains(item) & self.2.0.contains(item)
        });
        let mut badge_iter = self.0.0.into_iter();
        match badge_iter.next() {
            Some(x) => match badge_iter.next() {
                Some(_) => x,
                None => panic!("There was more than one intersection of badges"),
            }
            None => panic!("There was no intersection of badges")
        }
    }
}

impl<S: Into<String>> From<S> for RucksackOld {
    fn from(value: S) -> Self {
        let value = value.into();
        let length = value.len() / 2;
        let (compartment1, compartment2) = value.split_at(length);
        RucksackOld(string_to_items(compartment1), string_to_items(compartment2))
    }
}

fn string_to_items(items: &str) -> HashSet<char> {
    items.chars().collect()
}

// fn priority(item: &char, priority_list: &HashMap<char, u32>) -> u32 {
//     priority_list[item]
// }

impl RucksackOld {
    fn in_both_compartments(&self) -> &char {
        let mut in_both = (self.0).intersection(&self.1);
        match in_both.next() {
            Some(x) => match in_both.next() {
                None => x,
                Some(_) => panic!(),
            },
            None => {
                dbg!(&self);
                panic!()
            }
        }
    }

    fn in_both_priority(&self, priority_list: &HashMap<char, u32>) -> u32 {
        priority_list[self.in_both_compartments()]
    }
}

fn main() {
    run();
}

fn read_input() -> Vec<String> {
    // Should work for any AOC project
    let file = std::fs::File::open("input.txt").expect("File does not exist");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_input(input: Vec<String>) -> Vec<RucksackOld> {
    input.iter().map(|line| RucksackOld::from(line)).collect()
}

fn priority_sum(rucksacks: Vec<RucksackOld>, priority_list: &HashMap<char, u32>) -> u32 {
    rucksacks
        .iter()
        .map(|r| r.in_both_priority(priority_list))
        .sum()
}

fn run() {
    let priority_list: HashMap<char, u32> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
    let rucksacks = read_input();
    let rucksacks = parse_input(rucksacks);
    let total = priority_sum(rucksacks, &priority_list);
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() {
        let priority_list: HashMap<char, u32> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
        let input_string: Vec<&str> = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .split('\n')
            .collect();
        let input_string = input_string
            .into_iter()
            .map(|line| line.to_owned())
            .collect();
        let rucksacks = parse_input(input_string);
        let total: u32 = rucksacks
            .iter()
            .map(|r| r.in_both_priority(&priority_list))
            .sum();
        assert_eq!(157, total);
    }
}
