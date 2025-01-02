use crate::helpers::string_to_matrix;
use std::collections::HashSet;

#[allow(dead_code)]
fn turn(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Unable to turn: Invalid direction"),
    }
}

#[allow(dead_code)]
fn coords_are_within_bounds(matrix: &Vec<Vec<char>>, (x, y): (i32, i32)) -> bool {
    x >= 0 && y >= 0 && x < matrix[0].len() as i32 && y < matrix.len() as i32
}

#[allow(dead_code)]
fn move_forward((x, y): (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '^' => (x, y - 1),
        '>' => (x + 1, y),
        'v' => (x, y + 1),
        '<' => (x - 1, y),
        _ => panic!("Unable to move: Invalid direction"),
    }
}

// https://adventofcode.com/2024/day/6
#[allow(dead_code)]
pub fn part_01(contents: String) -> i32 {
    let matrix = string_to_matrix(contents);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    // step through each coord in the matrix
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let mut x = col as i32;
            let mut y = row as i32;
            let mut direction = matrix[row][col];

            // if we find a starting point
            if direction == '^' {
                loop {
                    visited.insert((x, y));
                    let (new_x, new_y) = move_forward((x, y), direction);

                    if coords_are_within_bounds(&matrix, (new_x, new_y)) {
                        if matrix[new_y as usize][new_x as usize] == '#' {
                            direction = turn(direction);
                        } else {
                            x = new_x;
                            y = new_y;
                        }
                    } else {
                        break;
                    }
                }

                return visited.len() as i32;
            }
        }
    }

    visited.len() as i32
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
    fn test_part_01_02() {
        // https://adventofcode.com/2024/day/6/input
        let contents = fs::read_to_string("./input/day_06_1.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 4663);
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
