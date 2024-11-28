use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    run();
}

#[derive(Debug, Clone)]
enum Hand {
    Rock,     // Rock
    Paper,    // Paper
    Scissors, // Scissors
}

#[derive(Debug)]
enum Strategy {
    Lose, // nvm "lose"
    Draw, // nvm "draw"
    Win,  // nvm "win"
}

impl From<&str> for Strategy {
    fn from(value: &str) -> Self {
        match value {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!(),
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!(),
        }
    }
}

impl Strategy {
    fn calculate_hand(self, opponent: &Hand) -> Hand {
        match self {
            Strategy::Lose => match opponent {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            },

            Strategy::Draw => opponent.clone(),

            Strategy::Win => match opponent {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
        }
    }
}

impl Hand {
    fn hand_points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

fn battle_points(opponent: &Hand, me: &Hand) -> u32 {
    match (opponent, me) {
        (Hand::Rock, Hand::Paper)
        | (Hand::Paper, Hand::Scissors)
        | (Hand::Scissors, Hand::Rock) => 6,
        (Hand::Rock, Hand::Rock)
        | (Hand::Paper, Hand::Paper)
        | (Hand::Scissors, Hand::Scissors) => 3,
        (Hand::Rock, Hand::Scissors)
        | (Hand::Paper, Hand::Rock)
        | (Hand::Scissors, Hand::Paper) => 0,
    }
}

fn read_input() -> Vec<String> {
    // Should work for any AOC project
    let file = File::open("input.txt").expect("File does not exist");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_line(line: &String) -> (Hand, Hand) {
    let mut output = line.split(' ');
    let opponent = Hand::from(output.next().expect("Should never fail"));
    let me = Strategy::from(output.next().expect("Should never fail"));
    let me = me.calculate_hand(&opponent);

    (opponent, me)
}

fn parse_input(input: Vec<String>) -> Vec<(Hand, Hand)> {
    input.iter().map(|l| parse_line(l)).collect()
}

fn total_points(parsed_input: Vec<(Hand, Hand)>) -> u32 {
    parsed_input
        .iter()
        .map(|(opp, me)| me.hand_points() + battle_points(opp, me))
        .sum()
}

fn run() {
    let battles = read_input();
    let battles = parse_input(battles);
    println!("{}", total_points(battles))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() {
        let input_string: Vec<String> = vec!["A Y", "B X", "C Z"]
            .into_iter()
            .map(|x| x.to_owned())
            .collect();
        let battles = parse_input(input_string);
        let battles2 = battles.clone();
        for (opp, me) in battles {
            println!(
                "Shape points: {}, battle points: {}",
                me.hand_points(),
                battle_points(&opp, &me)
            );
        }
        let points = total_points(battles2);
        assert_eq!(points, 12)
    }
}
