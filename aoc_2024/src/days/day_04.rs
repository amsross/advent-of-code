#[allow(dead_code)]
fn next_char_in_string(string: &String, substring: &String) -> Option<char> {
    if string.len() == substring.len() {
        return None;
    }

    let mut string_chars = string.chars();
    let mut substring_chars = substring.chars();

    while let Some(string_char) = string_chars.next() {
        match substring_chars.next() {
            // when we've reached the end of the substring, we return the next character in the string
            None => return Some(string_char),
            // if there are still substring_chars, then iterate again
            Some(substring_char) => {
                // if the characters don't match, then these strings might not be valid for
                // comparison
                if string_char != substring_char {
                    return None;
                }
            }
        }
    }

    None
}

#[allow(dead_code)]
fn string_to_matrix(string: String) -> Vec<Vec<char>> {
    string.lines().map(|line| line.chars().collect()).collect()
}

#[allow(dead_code)]
fn get_matrix_value(matrix: &Vec<Vec<char>>, row: usize, column: usize) -> Option<char> {
    let row = matrix.get(row)?;
    let value = row.get(column)?;

    Some(*value)
}

#[allow(dead_code)]
fn check(
    matrix: &Vec<Vec<char>>,
    string_to_match: &String,
    mut substring: String,
    row: usize,
    column: usize,
    coord_stepper: fn(usize, usize) -> (usize, usize),
) -> Option<bool> {
    if *string_to_match == substring {
        return Some(true);
    }

    if row >= matrix.len() || column >= matrix.get(row)?.len() {
        return None;
    }

    let expected = next_char_in_string(&string_to_match, &substring)?;
    let actual = get_matrix_value(matrix, row, column)?;

    if expected == actual {
        substring.push(actual);
        let (row, column) = coord_stepper(row, column);
        return check(matrix, string_to_match, substring, row, column, coord_stepper);
    }

    None
}

// https://adventofcode.com/2024/day/4
#[allow(dead_code)]
pub fn part_01(contents: String) -> i32 {
    let string_to_match = String::from("XMAS");
    let matrix = string_to_matrix(contents);
    let mut count = 0;

    let coord_steppers: Vec<Box<fn(usize, usize) -> (usize, usize)>> = vec![
        Box::new(|row, column| (row.wrapping_sub(1), column)),
        Box::new(|row, column| (row.wrapping_sub(1), column + 1)),
        Box::new(|row, column| (row, column + 1)),
        Box::new(|row, column| (row + 1, column + 1)),
        Box::new(|row, column| (row + 1, column)),
        Box::new(|row, column| (row + 1, column.wrapping_sub(1))),
        Box::new(|row, column| (row, column.wrapping_sub(1))),
        Box::new(|row, column| (row.wrapping_sub(1), column.wrapping_sub(1))),
    ];

    for row_index in 0..matrix.len() {
        let row = matrix.get(row_index).unwrap();

        for character_index in 0..row.len() {
            for coord_stepper in &coord_steppers {
                if let Some(_) = check(
                    &matrix,
                    &string_to_match,
                    String::new(),
                    row_index,
                    character_index,
                    **coord_stepper,
                ) {
                    count += 1;
                }
            }
        }
    }

    count
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
    fn test_next_char_in_string_01() {
        let string = "MMMSXXMASM".to_owned();
        let subjects = vec![
            ("", Some('M')),
            ("M", Some('M')),
            ("MM", Some('M')),
            ("MMM", Some('S')),
            ("MMMS", Some('X')),
            ("MMMSX", Some('X')),
            ("MMMSXX", Some('M')),
            ("MMMSXXM", Some('A')),
            ("MMMSXXMA", Some('S')),
            ("MMMSXXMAS", Some('M')),
            ("MMMSXXMASM", None),
            ("MS", None),
        ];

        for (substring, expected) in subjects {
            let result = next_char_in_string(&string, &String::from(substring));

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_string_to_matrix_01() {
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

        let result = string_to_matrix(contents);
        let expected = vec![
            "MMMSXXMASM".chars().collect::<Vec<char>>(),
            "MSAMXMSMSA".chars().collect::<Vec<char>>(),
            "AMXSXMAAMM".chars().collect::<Vec<char>>(),
            "MSAMASMSMX".chars().collect::<Vec<char>>(),
            "XMASAMXAMM".chars().collect::<Vec<char>>(),
            "XXAMMXXAMA".chars().collect::<Vec<char>>(),
            "SMSMSASXSS".chars().collect::<Vec<char>>(),
            "SAXAMASAAA".chars().collect::<Vec<char>>(),
            "MAMMMXMMMM".chars().collect::<Vec<char>>(),
            "MXMXAXMASX".chars().collect::<Vec<char>>(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_01_01() {
        // https://adventofcode.com/2024/day/4
        let contents: String = "
MMMSXXMASM
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

    #[test]
    fn test_part_01_02() {
        // https://adventofcode.com/2024/day/4/input
        let contents = fs::read_to_string("./input/day_04_1.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 2358);
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
