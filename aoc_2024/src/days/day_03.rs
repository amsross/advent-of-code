use regex::Regex;

// https://adventofcode.com/2024/day/3
#[allow(dead_code)]
pub fn part_01(contents: String) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results = vec![];
    for (_, [left, right]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push((left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()));
    }

    results
        .into_iter()
        .fold(0, |acc, (left, right)| acc + left * right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01_01() {
        // https://adventofcode.com/2024/day/3
        let contents: String =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned();

        let result = part_01(contents);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_01_02() {
        // https://adventofcode.com/2024/day/3/input
        let contents = fs::read_to_string("./input/day_03_1.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 170778545);
    }
}
