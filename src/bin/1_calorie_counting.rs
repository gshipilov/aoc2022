use aoc2022::read_input_lines;

fn main() {
    let input = read_input_lines("1_calorie_counting");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> u32 {
    let mut max_elf = 0;
    let mut current_elf = 0;

    for line in input {
        if line.is_empty() {
            max_elf = max_elf.max(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }

    max_elf
}

fn part_2(input: &[String]) -> u32 {
    let mut elves = Vec::new();
    let mut current_elf = 0;

    for line in input {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    elves.sort_unstable();

    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;

    #[test]
    fn test_part_1() {
        let lines: Vec<_> = TEST_INPUT.lines().map(String::from).collect();
        assert_eq!(part_1(&lines), 24000);
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<_> = TEST_INPUT.lines().map(String::from).collect();
        assert_eq!(part_2(&lines), 45000);
    }
}
