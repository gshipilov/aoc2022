use aoc2022::read_input;

fn main() {
    let input = read_input("10_cathode_ray_tube");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));
}

fn part_1(input: &str) -> isize {
    let mut clock = 0;
    let mut x: isize = 1;
    let mut total = 0;

    for line in input.lines() {
        let tokens: Vec<_> = line.split(' ').collect();

        match tokens[0] {
            "noop" => {
                clock += 1;
                total += x * check_clock(clock);
            }
            "addx" => {
                clock += 1;
                total += x * check_clock(clock);
                clock += 1;
                total += x * check_clock(clock);
                x += tokens[1].parse::<isize>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    total
}

fn check_clock(clock: isize) -> isize {
    match clock {
        20 | 60 | 100 | 140 | 180 | 220 => clock,
        _ => 0,
    }
}

fn part_2(input: &str) -> String {
    let mut x: isize = 1;

    let mut screen = Screen {
        buf: String::new(),
        pos: 0,
    };

    for line in input.lines() {
        let tokens: Vec<_> = line.split(' ').collect();

        match tokens[0] {
            "noop" => {
                screen.draw(x);
            }
            "addx" => {
                screen.draw(x);
                screen.draw(x);
                x += tokens[1].parse::<isize>().unwrap();
            }
            _ => unreachable!(),
        }
    }
    screen.buf
}

struct Screen {
    buf: String,
    pos: isize,
}

impl Screen {
    fn draw(&mut self, x: isize) {
        if (self.pos - 1..=self.pos + 1).contains(&x) {
            self.buf.push('#');
        } else {
            self.buf.push(' ');
        }
        self.pos += 1;

        if self.pos == 40 {
            self.buf.push('\n');
            self.pos = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
    addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(13140, part_1(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        println!("{}", part_2(TEST_INPUT));
    }
}
