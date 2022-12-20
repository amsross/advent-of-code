fn range_to_bounds(range: &str) -> (u8, u8) {
    let mut bounds = range.split_terminator('-');

    let left = bounds.next().unwrap().parse::<u8>().unwrap();
    let right = bounds.next().unwrap().parse::<u8>().unwrap();

    (left, right)
}

fn line_to_bounds(line: &&str) -> ((u8, u8), (u8, u8)) {
    let mut lines = line.split_terminator(',');

    let left = lines.next().unwrap();
    let right = lines.next().unwrap();

    let left = range_to_bounds(left);
    let right = range_to_bounds(right);

    (left, right)
}

fn is_contained(n: u8, (start, end): (u8, u8)) -> bool {
    std::ops::Range {
        start,
        end: end + 1,
    }
    .contains(&n)
}

fn keep_subsets(left: (u8, u8), right: (u8, u8)) -> bool {
    (is_contained(left.0, right) && is_contained(left.1, right))
        || (is_contained(right.0, left) && is_contained(right.1, left))
}

fn keep_overlaps(left: (u8, u8), right: (u8, u8)) -> bool {
    is_contained(left.0, right)
        || is_contained(left.1, right)
        || is_contained(right.0, left)
        || is_contained(right.1, left)
}

// https://adventofcode.com/2022/day/4
#[allow(dead_code)]
pub fn part_01(contents: String) -> usize {
    contents
        .split_whitespace()
        .into_iter()
        .filter(|line| {
            let (left, right) = line_to_bounds(line);
            let keep = keep_subsets(left, right);

            println!(
                "{}-{},{}-{} - {:#?}",
                left.0, left.1, right.0, right.1, keep
            );

            keep
        })
        .count()
}

// https://adventofcode.com/2022/day/4#part2
#[allow(dead_code)]
pub fn part_02(contents: String) -> usize {
    contents
        .split_whitespace()
        .into_iter()
        .filter(|line| {
            let (left, right) = line_to_bounds(line);
            let keep = keep_overlaps(left, right);

            println!(
                "{}-{},{}-{} - {:#?}",
                left.0, left.1, right.0, right.1, keep
            );

            keep
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01() {
        // https://adventofcode.com/2022/day/4/input
        let contents = fs::read_to_string("./input/day_04.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 503);
    }

    #[test]
    fn test_part_02() {
        // https://adventofcode.com/2022/day/4/input
        let contents = fs::read_to_string("./input/day_04.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, 827);
    }
}
