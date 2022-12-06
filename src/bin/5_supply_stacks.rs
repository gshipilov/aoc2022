use aoc2022::read_input;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, digit1, newline};
use nom::combinator::{map, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

fn main() {
    let input = read_input("5_supply_stacks");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> String {
    let (_, (mut stacks, instructions)) = parse(input).unwrap();

    for instruction in instructions {
        let from = stacks.get_mut(instruction.from).unwrap();
        let want_len = from.len() - instruction.amount;

        let tail = from.split_off(want_len);

        let to = stacks.get_mut(instruction.to).unwrap();
        to.extend(tail.iter().rev());
    }

    stacks.iter_mut().flat_map(|stack| stack.pop()).collect()
}

fn part_2(input: &str) -> String {
    let (_, (mut stacks, instructions)) = parse(input).unwrap();

    for instruction in instructions {
        let from = stacks.get_mut(instruction.from).unwrap();
        let want_len = from.len() - instruction.amount;

        let tail = from.split_off(want_len);

        let to = stacks.get_mut(instruction.to).unwrap();
        to.extend(tail);
    }

    stacks.iter_mut().flat_map(|stack| stack.pop()).collect()
}

type Stack = Vec<char>;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse(s: &str) -> IResult<&str, (Vec<Stack>, Vec<Instruction>)> {
    separated_pair(parse_stacks, newline, parse_instructions)(s)
}

fn parse_stacks(s: &str) -> IResult<&str, Vec<Stack>> {
    terminated(
        map(separated_list1(newline, parse_crates), |lists| {
            let mut results = Vec::new();

            let outer = lists.len();
            let inner = lists[0].len();
            for i in 0..inner {
                let mut stack = Vec::new();
                for j in (0..outer).rev() {
                    if let Some(c) = lists[j][i] {
                        stack.push(c);
                    }
                }
                results.push(stack);
            }

            results
        }),
        tuple((newline, parse_crate_footer, newline)),
    )(s)
}

fn parse_crates(s: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), parse_crate)(s)
}

fn parse_crate(s: &str) -> IResult<&str, Option<char>> {
    alt((
        value(None, tag("   ")),
        map(delimited(tag("["), alpha1, tag("]")), |s: &str| {
            s.chars().next()
        }),
    ))(s)
}

fn parse_crate_footer(s: &str) -> IResult<&str, ()> {
    value((), pair(tag(" 1"), is_not("\n")))(s)
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_instruction)(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), digit1),
            preceded(tag(" from "), digit1),
            preceded(tag(" to "), digit1),
        )),
        |(amount, from, to)| Instruction {
            amount: str::parse::<usize>(amount).unwrap(),
            from: str::parse::<usize>(from).unwrap() - 1,
            to: str::parse::<usize>(to).unwrap() - 1,
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "#;

    #[test]
    fn test_part_1() {
        assert_eq!("CMZ".to_string(), part_1(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("MCD", part_2(TEST_INPUT));
    }
}
