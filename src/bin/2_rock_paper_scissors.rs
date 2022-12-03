use aoc2022::read_input;
use nom::branch::alt;
use nom::character::complete::{char, newline, space1};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

fn main() {
    let input = read_input("2_rock_paper_scissors");
    let (_, rounds) = parse(&input).unwrap();

    println!("Part 1: {}", part_1(rounds));

    let (_, rounds) = parse_2(&input).unwrap();

    println!("Part 2: {}", part_2(rounds));
}

fn part_1(input: Vec<(Throw, Throw)>) -> u32 {
    input.into_iter().map(|(l, r)| play(l, r)).sum()
}

fn part_2(input: Vec<(Throw, Goal)>) -> u32 {
    input.into_iter().map(|(l, r)| play_2(l, r)).sum()
}

#[derive(Debug, Clone, Copy)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Goal {
    Lose,
    Tie,
    Win,
}

const LOSE: u32 = 0;
const TIE: u32 = 3;
const WIN: u32 = 6;

fn play(them: Throw, me: Throw) -> u32 {
    match me {
        Throw::Rock => match them {
            Throw::Rock => TIE + 1,
            Throw::Paper => LOSE + 1,
            Throw::Scissors => WIN + 1,
        },
        Throw::Paper => match them {
            Throw::Rock => WIN + 2,
            Throw::Paper => TIE + 2,
            Throw::Scissors => LOSE + 2,
        },
        Throw::Scissors => match them {
            Throw::Rock => LOSE + 3,
            Throw::Paper => WIN + 3,
            Throw::Scissors => TIE + 3,
        },
    }
}

fn play_2(them: Throw, goal: Goal) -> u32 {
    match them {
        Throw::Rock => match goal {
            Goal::Lose => play(them, Throw::Scissors),
            Goal::Tie => play(them, Throw::Rock),
            Goal::Win => play(them, Throw::Paper),
        },
        Throw::Paper => match goal {
            Goal::Lose => play(them, Throw::Rock),
            Goal::Tie => play(them, Throw::Paper),
            Goal::Win => play(them, Throw::Scissors),
        },
        Throw::Scissors => match goal {
            Goal::Lose => play(them, Throw::Paper),
            Goal::Tie => play(them, Throw::Scissors),
            Goal::Win => play(them, Throw::Rock),
        },
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Throw, Throw)>> {
    separated_list1(newline, separated_pair(parse_them, space1, parse_me))(input)
}

fn parse_2(input: &str) -> IResult<&str, Vec<(Throw, Goal)>> {
    separated_list1(newline, separated_pair(parse_them, space1, parse_goal))(input)
}

fn parse_them(input: &str) -> IResult<&str, Throw> {
    alt((
        value(Throw::Rock, char('A')),
        value(Throw::Paper, char('B')),
        value(Throw::Scissors, char('C')),
    ))(input)
}

fn parse_me(input: &str) -> IResult<&str, Throw> {
    alt((
        value(Throw::Rock, char('X')),
        value(Throw::Paper, char('Y')),
        value(Throw::Scissors, char('Z')),
    ))(input)
}

fn parse_goal(input: &str) -> IResult<&str, Goal> {
    alt((
        value(Goal::Lose, char('X')),
        value(Goal::Tie, char('Y')),
        value(Goal::Win, char('Z')),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
    A Y
    B X
    C Z
    "#};

    #[test]
    fn test_part_1() {
        let (_, rounds) = parse(TEST_INPUT).unwrap();
        assert_eq!(15, part_1(rounds));
    }

    #[test]
    fn test_part_2() {
        let (_, rounds) = parse_2(TEST_INPUT).unwrap();
        assert_eq!(12, part_2(rounds));
    }
}
