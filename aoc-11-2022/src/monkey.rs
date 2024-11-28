use std::cell::Cell;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: RefCell<VecDeque<u64>>,
    pub operation: Operation,
    pub test: Test, // For future proofing, can add "divisible" as part of data
    pub inspections: Cell<u64>,
}

#[derive(Debug, Clone)]
pub struct Test {
    pub divisible_by: u64,
    pub true_target: u64,
    pub false_target: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    pub symbol: Symbol,
    pub amount: Amount,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Multiply,
    Add,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Amount {
    Old,
    Constant(u64),
}

impl Monkey {
    fn throw_item(&self, common_divisor: u64) -> Option<(u64, u64)> {
        let mut items = self.items.borrow_mut();
        match items.pop_front() {
            Some(x) => {
                // println!("    Item val before: {x}");
                let item = self.do_operation(x);
                // println!("    Item val after operation: {item}");
                let target = self.test_item(item);
                // println!("    Throw target: {target}\n");
                let item = item % common_divisor;

                let inspections = self.inspections.get();
                self.inspections.set(inspections + 1);

                Some((item, target))
            }
            None => None,
        }
    }

    fn throw_all_items(&self, monkeys: &BTreeMap<u64, Monkey>, common_divisor: u64) {
        while let Some((item, target)) = self.throw_item(common_divisor) {
            let target_monkey = monkeys.get(&target).expect("Index should be legit");
            let mut target_items = target_monkey.items.borrow_mut();
            (*target_items).push_back(item);
        }
    }

    fn test_item(&self, item: u64) -> u64 {
        match item % self.test.divisible_by == 0 {
            true => self.test.true_target,
            false => self.test.false_target,
        }
    }

    fn do_operation(&self, item: u64) -> u64 {
        match self.operation.symbol {
            Symbol::Multiply => item * self.operation.amount(item),
            Symbol::Add => item + self.operation.amount(item),
        }
    }
}

impl Operation {
    fn amount(&self, item: u64) -> u64 {
        match self.amount {
            Amount::Old => item,
            Amount::Constant(x) => x,
        }
    }
}

pub fn get_monkey_business(monkeys: BTreeMap<u64, Monkey>) -> u64 {
    let mut inspection_numbers: Vec<u64> = monkeys
        .into_iter()
        .map(|(_id, monkey)| monkey.inspections.get())
        .collect();
    inspection_numbers.sort();
    let val1 = inspection_numbers.pop().unwrap();
    let val2 = inspection_numbers.pop().unwrap();
    val1 * val2
}

pub fn common_divisor(monkeys: &BTreeMap<u64, Monkey>) -> u64 {
    monkeys
        .iter()
        .map(|(_, monkey)| monkey.test.divisible_by)
        .product()
}

pub fn do_round(monkeys: &BTreeMap<u64, Monkey>, common_divisor: u64) {
    for (_index, monkey) in monkeys {
        // println!("\nMonkey {_index}:");
        monkey.throw_all_items(monkeys, common_divisor);

        #[cfg(test)]
        println!(
            "    Monkey {_index} inspected items {} times.",
            monkey.inspections.get()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_monkeys;
    use anyhow::Result;

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
    fn common_divisor_works() -> Result<()> {
        let (_input, monkeys) = parse_monkeys(INPUT)?;
        assert_eq!(common_divisor(&monkeys), 96577); // 23 * 19 * 13 *17
        Ok(())
    }

    #[test]
    fn round_works() -> Result<()> {
        let (_input, monkeys) = parse_monkeys(INPUT)?;
        let common_divisor = monkeys
            .iter()
            .map(|(_, monkey)| monkey.test.divisible_by)
            .product();
        for i in 1..=10_000 {
            println!("\n== After round {i} ==");
            do_round(&monkeys, common_divisor);
        }
        // dbg!(&monkeys);
        dbg!(get_monkey_business(monkeys));
        panic!()
    }
}
