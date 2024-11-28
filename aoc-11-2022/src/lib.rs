/// Learning to use nom
///
use anyhow::{anyhow, Result};

mod monkey;
use monkey::common_divisor;
use monkey::do_round;
use monkey::get_monkey_business;

mod parser;
use parser::parse_monkeys;

pub fn do_part1(input: &str) -> Result<u64> {
    let monkeys = match parser::parse_monkeys(input) {
        Ok((_, monkeys)) => monkeys,
        _ => return Err(anyhow!("Parse Error")),
    };
    for _ in 0..20 {
        do_round(&monkeys, 20);
    }
    Ok(get_monkey_business(monkeys))
}

pub fn do_part2(input: &str) -> Result<u64> {
    let monkeys = match parse_monkeys(input) {
        Ok((_, monkeys)) => monkeys,
        _ => return Err(anyhow!("Parse Error")),
    };
    let common_divisor = common_divisor(&monkeys);
    for _ in 0..10_000 {
        do_round(&monkeys, common_divisor);
    }
    Ok(get_monkey_business(monkeys))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_it_works() -> Result<()> {
        let result = do_part1(INPUT)?;
        assert_eq!(result, 10605);
        Ok(())
    }

    #[test]
    fn part2_it_works() -> Result<()> {
        panic!();
    }

    #[test]
    fn wholla() {
        dbg!(15 % 2, 27 % 5, 5 % 5);
        panic!()
    }
}
