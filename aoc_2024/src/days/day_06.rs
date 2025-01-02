use crate::helpers::string_to_matrix;

// https://adventofcode.com/2024/day/6
#[allow(dead_code)]
pub fn part_01(contents: String) -> i32 {
    let _matrix = string_to_matrix(contents);
    println!("{:?}", _matrix);

    0
}

// https://adventofcode.com/2024/day/6#part2
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
        // https://adventofcode.com/2024/day/4
        let contents: String = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .trim()
            .to_owned();

        let result = part_01(contents);

        assert_eq!(result, 41);
    }

    #[test]
    #[ignore]
    fn test_part_01_02() {
        // https://adventofcode.com/2024/day/6/input
        let contents = fs::read_to_string("./input/day_06_1.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 2358);
    }

    #[test]
    #[ignore]
    fn test_part_02_01() {
        // https://adventofcode.com/2024/day/4#part2
        let contents: String = "".to_owned();

        let result = part_02(contents);

        assert_eq!(result, 9);
    }

    #[test]
    #[ignore]
    fn test_part_02_02() {
        // https://adventofcode.com/2024/day/6/input
        let contents = fs::read_to_string("./input/day_06_2.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, 1737);
    }
}
