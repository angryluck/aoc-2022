use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::u64;
use nom::character::complete::{alphanumeric1, multispace0};
use nom::combinator::value;
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;

use crate::monkey::Amount;
use crate::monkey::Monkey;
use crate::monkey::Operation;
use crate::monkey::Symbol;
use crate::monkey::Test;

pub fn parse_monkeys(input: &str) -> IResult<&str, BTreeMap<u64, Monkey>> {
    fold_many1(
        parse_monkey,
        BTreeMap::new,
        move |mut acc: BTreeMap<_, _>, (id, monkey)| {
            acc.insert(id, monkey);
            acc
        },
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, (u64, Monkey)> {
    let (input, monkey_id) = parse_monkey_id(input)?;
    let (input, items) = parse_items(input)?;
    let items = RefCell::new(items);
    let (input, operation) = parse_operation(input)?;
    let (input, test) = parse_test(input)?;

    Ok((
        input,
        (
            monkey_id,
            Monkey {
                items,
                operation,
                test,
                inspections: Cell::from(0),
            },
        ),
    ))
}

// Inspired by: https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#whitespacehttps://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#whitespace
fn skip<'a>(text: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(multispace0, tag(text), multispace0)
}

fn parse_monkey_id(input: &str) -> IResult<&str, u64> {
    let (input, _) = skip("Monkey")(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = skip(":")(input)?;

    Ok((input, id))
    // // let (input, _) = tag("Monkey ")(input)?;
    // delimited(tag("Monkey "), u64, tag(":"))(input)
}

fn parse_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, _) = skip("Starting items:")(input)?;
    let (input, items) = separated_list1(tag(", "), u64)(input)?;
    let items = VecDeque::from(items);
    let (input, _) = multispace0(input)?;

    Ok((input, items))
}

fn parse_symbol(input: &str) -> IResult<&str, Symbol> {
    alt((
        value(Symbol::Multiply, tag("*")),
        value(Symbol::Add, tag("+")),
    ))(input)
}

fn parse_amount(input: &str) -> IResult<&str, Amount> {
    alt((
        value(Amount::Old, tag("old")),
        u64.map(|d| Amount::Constant(d)),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = skip("Operation: new = old")(input)?;
    let (input, symbol) = take(1usize)(input)?;
    let (_, symbol) = parse_symbol(symbol)?;
    let (input, _) = tag(" ")(input)?;
    let (input, amount) = alphanumeric1(input)?;
    let (_, amount) = parse_amount(amount)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Operation { symbol, amount }))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = multispace0(input)?;
    let (input, _) = skip("Test: divisible by")(input)?;
    let (input, divisible_by) = u64(input)?;
    let (input, _) = skip("If true: throw to monkey")(input)?;
    let (input, true_target) = u64(input)?;
    let (input, _) = skip("If false: throw to monkey")(input)?;
    let (input, false_target) = u64(input)?;
    let (input, _) = multispace0(input)?;

    // let (input, divisible_by) = delimited(tag("Test: divisible by "), u64, multispace0)(input)?;
    // let (input, true_target) =
    //     delimited(tag("If true: throw to monkey "), u64, multispace0)(input)?;
    // let (input, false_target) =
    //     delimited(tag("If false: throw to monkey "), u64, multispace0)(input)?;

    Ok((
        input,
        Test {
            divisible_by,
            true_target,
            false_target,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const ITEM_TEST: &str = "  Starting items: 79, 98";
    const OPERATION_TEST: &str = "  Operation: new = old * 19";
    // const TEST_TEST: &str = "  Test: divisible by 23";

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
    fn nom_item_works() -> Result<()> {
        let (_, items) = parse_items(ITEM_TEST)?;
        assert_eq!(items, vec![79, 98]);
        Ok(())
    }

    #[test]
    fn nom_operation_works() -> Result<()> {
        let (_, operation) = parse_operation(OPERATION_TEST)?;
        assert_eq!(
            operation,
            Operation {
                symbol: Symbol::Multiply,
                amount: Amount::Constant(19)
            }
        );
        Ok(())
    }

    #[test]
    fn nom_monkey_works() -> Result<()> {
        let (_, (id, monkey)) = parse_monkey(INPUT)?;
        dbg!(id, monkey);
        panic!()
    }

    #[test]
    fn nom_monkeys_works() -> Result<()> {
        dbg!(parse_monkeys(INPUT)?);
        panic!()
    }
}
