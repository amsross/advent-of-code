use crate::helpers::string_to_matrix;

// https://adventofcode.com/2024/day/7
#[allow(dead_code)]
pub fn part_01(contents: String) -> i32 {
    let _matrix = string_to_matrix(contents);

    0
}

// https://adventofcode.com/2024/day/7#part2
#[allow(dead_code)]
pub fn part_02(contents: String) -> i32 {
    let _matrix = string_to_matrix(contents);

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01_01() {
        // https://adventofcode.com/2024/day/7
        let contents: String = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .trim()
            .to_owned();

        let result = part_01(contents);

        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn test_part_01_02() {
        let contents = fs::read_to_string("./input/day_07.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn test_part_02_01() {
        // https://adventofcode.com/2024/day/4#part2
        let contents: String = "".trim().to_owned();

        let result = part_02(contents);

        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn test_part_02_02() {
        let contents = fs::read_to_string("./input/day_07.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, 0);
    }
}
