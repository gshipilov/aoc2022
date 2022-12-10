use aoc2022::read_input;
use std::collections::HashSet;

fn main() {
    let input = read_input("9_rope_bridge");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let directions = parse_directions(input);

    let mut visited = HashSet::new();

    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);

    visited.insert(tail);

    for direction in directions {
        match direction {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
        };

        tail.move_towards(&head);
        visited.insert(tail);
    }

    visited.len()
}

fn part_2(input: &str) -> usize {
    let directions = parse_directions(input);

    let mut visited = HashSet::new();
    let mut head = Point::new(0, 0);

    let mut knots = vec![Point { x: 0, y: 0 }; 9];
    visited.insert(head);

    for direction in directions {
        match direction {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
        };

        let mut prev = head;
        for knot in knots.iter_mut() {
            knot.move_towards(&prev);
            prev = *knot;
        }
        visited.insert(prev);
    }

    visited.len()
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .flat_map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            let num = tokens[1].parse().unwrap();
            let direction = match tokens[0] {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => unreachable!(),
            };

            vec![direction; num]
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn move_towards(&mut self, other: &Point) {
        let xdiff = (self.x - other.x).abs();
        let ydiff = (self.y - other.y).abs();

        if xdiff <= 1 && ydiff <= 1 {
            return;
        }

        if ydiff >= 1 {
            if (self.y - other.y) < 0 {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }

        if xdiff >= 1 {
            if (self.x - other.x) < 0 {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(13, part_1(TEST_INPUT));
    }

    const TEST_INPUT_2: &str = indoc! {r#"
    R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20
    "#};

    #[test]
    fn test_part_2() {
        assert_eq!(36, part_2(TEST_INPUT_2));
    }
}
