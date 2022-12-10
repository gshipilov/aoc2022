use aoc2022::read_input;

fn main() {
    let input = read_input("8_tree_house");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let forest = parse_forest(input);
    let visibility = visibility(forest);

    visibility
        .iter()
        .flatten()
        .filter(|&&v| v == Visibility::Visible)
        .count()
}

fn part_2(input: &str) -> usize {
    let forest = parse_forest(input);
    let scores = scores(forest);

    *scores.iter().flatten().max().unwrap()
}

fn parse_forest(input: &str) -> Forest {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn visibility(forest: Forest) -> Vec<Vec<Visibility>> {
    let mut visibility = Vec::new();

    for y in 0..forest.len() {
        let mut line = Vec::with_capacity(forest[y].len());

        for x in 0..forest[0].len() {
            // Edges
            if x == 0 || y == 0 || x == forest[0].len() - 1 || y == forest.len() - 1 {
                line.push(Visibility::Visible);
                continue;
            }

            let tree = forest[y][x];

            let mut ups = (0..y).map(|ny| forest[ny][x]);
            if ups.all(|h| h < tree) {
                line.push(Visibility::Visible);
                continue;
            }

            let mut rights = (x + 1..forest[y].len()).map(|nx| forest[y][nx]);
            if rights.all(|h| h < tree) {
                line.push(Visibility::Visible);
                continue;
            }

            let mut downs = (y + 1..forest.len()).map(|ny| forest[ny][x]);
            if downs.all(|h| h < tree) {
                line.push(Visibility::Visible);
                continue;
            }

            let mut lefts = (0..x).map(|nx| forest[y][nx]);
            if lefts.all(|h| h < tree) {
                line.push(Visibility::Visible);
                continue;
            }

            line.push(Visibility::NotVisible);
        }

        visibility.push(line);
    }

    visibility
}

fn scores(forest: Forest) -> Vec<Vec<usize>> {
    let mut scores = Vec::new();

    for y in 0..forest.len() {
        let mut line = Vec::with_capacity(forest[y].len());

        for x in 0..forest[0].len() {
            // Edges
            if x == 0 || y == 0 || x == forest[0].len() - 1 || y == forest.len() - 1 {
                line.push(0);
                continue;
            }

            let tree = forest[y][x];

            let ups = (0..y).rev().map(|ny| forest[ny][x]);

            let all_ups = ups.clone().count();
            let filtered_ups = ups.take_while(|&h| h < tree).count();

            let ups = if filtered_ups < all_ups {
                filtered_ups + 1
            } else {
                filtered_ups
            };

            let rights = (x + 1..forest[y].len()).map(|nx| forest[y][nx]);

            let all_rights = rights.clone().count();
            let filtered_rights = rights.take_while(|&h| h < tree).count();

            let rights = if filtered_rights < all_rights {
                filtered_rights + 1
            } else {
                filtered_rights
            };

            let downs = (y + 1..forest.len()).map(|ny| forest[ny][x]);
            let all_downs = downs.clone().count();
            let filtered_downs = downs.take_while(|&h| h < tree).count();

            let downs = if filtered_downs < all_downs {
                filtered_downs + 1
            } else {
                filtered_downs
            };

            let lefts = (0..x).rev().map(|nx| forest[y][nx]);

            let all_lefts = lefts.clone().count();
            let filtered_lefts = lefts.take_while(|&h| h < tree).count();

            let lefts = if filtered_lefts < all_lefts {
                filtered_lefts + 1
            } else {
                filtered_lefts
            };

            line.push(ups * rights * downs * lefts);
        }

        scores.push(line);
    }

    scores
}

type Forest = Vec<Vec<u8>>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Visibility {
    Visible,
    NotVisible,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &str = indoc! {r#"
    30373
    25512
    65332
    33549
    35390
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(21, part_1(TEST_DATA));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(8, part_2(TEST_DATA));
    }
}
