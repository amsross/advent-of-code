// https://adventofcode.com/2024/day/4
#[allow(dead_code)]
pub fn part_01(_contents: String) -> i32 {
    0
}

// https://adventofcode.com/2024/day/4#part2
#[allow(dead_code)]
pub fn part_02(_contents: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01_01() {
        // https://adventofcode.com/2024/day/4
        let contents: String = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_owned();

        let result = part_01(contents);

        assert_eq!(result, 18);
    }

    #[ignore]
    #[test]
    fn test_part_01_02() {
        // https://adventofcode.com/2024/day/4/input
        let contents = fs::read_to_string("./input/day_04_1.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 0);
    }

    #[ignore]
    #[test]
    fn test_part_02_01() {
        // https://adventofcode.com/2024/day/4#part2
        let contents: String = "".to_owned();

        let result = part_02(contents);

        assert_eq!(result, 0);
    }

    #[ignore]
    #[test]
    fn test_part_02_02() {
        // https://adventofcode.com/2024/day/4/input
        let contents = fs::read_to_string("./input/day_04_2.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, 0);
    }
}
