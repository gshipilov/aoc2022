use aoc2022::read_input_lines;

fn main() {
    let lines = read_input_lines("3_rucksack_reorganization");
    println!("Part 1: {}", part_1(lines.clone()));
    println!("Part 2: {}", part_2(lines));
}

fn part_1(input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|line| {
            let (left, right) = (&line[0..(line.len() / 2)], &line[line.len() / 2..]);
            for c in left.chars() {
                if right.contains(c) {
                    return priority(c);
                }
            }
            unreachable!()
        })
        .sum()
}

fn priority(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        u32::from(c) - 96
    } else {
        u32::from(c) - 38
    }
}

fn part_2(input: Vec<String>) -> u32 {
    input
        .chunks(3)
        .map(|chunk| {
            for c in chunk[0].chars() {
                if chunk[1].contains(c) && chunk[2].contains(c) {
                    return priority(c);
                }
            }
            unreachable!()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &str = indoc! {r#"
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    "#};

    #[test]
    fn test_part_1() {
        let lines = TEST_DATA.lines().map(String::from).collect();
        assert_eq!(157, part_1(lines));
    }

    #[test]
    fn test_part_2() {
        let lines = TEST_DATA.lines().map(String::from).collect();
        assert_eq!(70, part_2(lines));
    }
}
