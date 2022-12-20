use std::collections::HashSet;

// https://adventofcode.com/2022/day/3
// https://adventofcode.com/2022/day/3/#part2
#[allow(dead_code)]
pub fn part_02(contents: String) -> u16 {
    fn prioritize(c: char) -> u16 {
        (c as u16) - if c.is_lowercase() { 64 + 32 } else { 65 - 27 }
    }

    fn handle_group((a, b, c): &(HashSet<char>, HashSet<char>, HashSet<char>)) -> u16 {
        let left = a.intersection(b).collect::<HashSet<_>>();
        let right = b.intersection(c).collect::<HashSet<_>>();

        prioritize(**left.intersection(&right).nth(0).unwrap())
    }

    let mut lines = contents.split_whitespace().into_iter();
    let mut groups = Vec::new();

    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        groups.push((
            a.chars().collect::<HashSet<_>>(),
            b.chars().collect::<HashSet<_>>(),
            c.chars().collect::<HashSet<_>>(),
        ));
    }

    groups.iter().map(handle_group).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_02() {
        // https://adventofcode.com/2022/day/3/input
        let contents = fs::read_to_string("./input/day_03.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, 2817);
    }
}
