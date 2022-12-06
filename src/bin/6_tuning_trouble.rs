use aoc2022::read_input;
use std::collections::HashSet;

fn main() {
    let input = read_input("6_tuning_trouble");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    for (idx, chunk) in input.as_bytes().windows(4).into_iter().enumerate() {
        let s = HashSet::<_>::from_iter(chunk);
        if s.len() == 4 {
            return idx + 4;
        }
    }
    unreachable!()
}

fn part_2(input: &str) -> usize {
    for (idx, chunk) in input.as_bytes().windows(14).into_iter().enumerate() {
        let s = HashSet::<_>::from_iter(chunk);
        if s.len() == 14 {
            return idx + 14;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
    mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(7, part_1(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(19, part_2(TEST_INPUT));
    }
}
