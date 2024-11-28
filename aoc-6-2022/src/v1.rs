use anyhow::Result;
// use itertools::Itertools;
// use std::collections::HashMap;

fn main() -> Result<()> {
    run("input.txt").expect("Everything should work");
    Ok(())
}

fn run(input_file: &str) -> Result<()> {
    let input = read_input(input_file)?;
    let index = find_marker(input);
    println!("{}", index);
    Ok(())
}

fn read_input(input_file: &str) -> Result<String> {
    Ok(std::fs::read_to_string(input_file)?)
}

fn find_marker(signal: String) -> usize {
    let mut marker_index = 4;
    let mut signal = signal.chars().into_iter();
    let mut current_chars: Vec<char> = signal.by_ref().take(4).collect();

    for c in signal {
        if no_duplicates(&current_chars) {
            break
        }
        marker_index += 1;
        current_chars.remove(0);
        current_chars.push(c);
    }
    marker_index
}

fn no_duplicates(characters: &Vec<char>) -> bool {
    let charactes_amount = characters.len();
    for i in 0..charactes_amount {
        if characters[i+1..].contains(&characters[i]) {
            return false
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_check() {
        let input1: Vec<char> = "abca".chars().collect();
        let input2: Vec<char>= "abcd".chars().collect();

        assert_eq!(no_duplicates(&input1), false);
        assert_eq!(no_duplicates(&input2), true);
    }

    #[test]
    fn test1() {
        let input = read_input("test1.txt").expect("didn't work");
        let index = find_marker(input);
        assert_eq!(index, 7);
    }

    #[test]
    fn test2() {
        let input = read_input("test2.txt").expect("didn't work");
        let index = find_marker(input);
        assert_eq!(index, 5);
    }

    #[test]
    fn test3() {
        let input = read_input("test3.txt").expect("didn't work");
        let index = find_marker(input);
        assert_eq!(index, 6);
    }

    #[test]
    fn test4() {
        let input = read_input("test4.txt").expect("didn't work");
        let index = find_marker(input);
        assert_eq!(index, 10);
    }

    #[test]
    fn test5() {
        let input = read_input("test5.txt").expect("didn't work");
        let index = find_marker(input);
        assert_eq!(index,11);
    }
}
