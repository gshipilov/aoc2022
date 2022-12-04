use aoc2022::read_input_lines;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;
use std::ops::RangeInclusive;

fn main() {
    let input = read_input_lines("4_camp_cleanup");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (_, ranges) = parse_line(line).unwrap();
            ranges
        })
        .filter(|(l, r)| {
            (l.contains(r.start()) && l.contains(r.end()))
                || (r.contains(l.start()) && r.contains(l.end()))
        })
        .count() as u32
}

fn part_2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (_, ranges) = parse_line(line).unwrap();
            ranges
        })
        .filter(|(l, r)| {
            (l.contains(r.start()) || l.contains(r.end()))
                || (r.contains(l.start()) || r.contains(l.end()))
        })
        .count() as u32
}

fn parse_line(s: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    separated_pair(parse_range, tag(","), parse_range)(s)
}

fn parse_range(s: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(separated_pair(digit1, tag("-"), digit1), |(l, r)| {
        (str::parse::<u32>(l).unwrap())..=(str::parse::<u32>(r).unwrap())
    })(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::str_to_lines;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(&str_to_lines(TEST_INPUT)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4, part_2(&str_to_lines(TEST_INPUT)));
    }
}
