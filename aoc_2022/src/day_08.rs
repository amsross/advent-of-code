fn parse_line(line: &str) -> Vec<isize> {
    let chars = line.chars();
    let mut numbers = Vec::with_capacity(line.len());

    for c in chars {
        match c.to_string().parse::<isize>() {
            Ok(n) => numbers.push(n),
            _ => (),
        }
    }

    numbers
}

fn parse_block(block: &str) -> Vec<Vec<isize>> {
    let lines = block.split_terminator('\n');

    lines.map(|line| parse_line(line)).collect()
}

// https://adventofcode.com/2022/day/8
#[allow(dead_code)]
fn part_01(content: &str) -> usize {
    let lines = parse_block(content);
    let mut map: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];

    let mut from_above: Vec<isize> = vec![-1; lines.len()];
    let mut from_left: Vec<isize> = vec![-1; lines[0].len()];

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            map[y][x] = lines[y][x] > from_above[x] || lines[y][x] > from_left[y];

            from_above[x] = lines[y][x].max(from_above[x]);
            from_left[y] = lines[y][x].max(from_left[y]);
        }
    }

    let mut from_below: Vec<isize> = vec![-1; lines.len()];
    let mut from_right: Vec<isize> = vec![-1; lines[0].len()];

    for y in (0..lines.len()).rev() {
        for x in (0..lines[y].len()).rev() {
            map[y][x] = map[y][x] || lines[y][x] > from_below[x] || lines[y][x] > from_right[y];

            from_below[x] = lines[y][x].max(from_below[x]);
            from_right[y] = lines[y][x].max(from_right[y]);
        }
    }

    println!("{:#?}", map);

    map.iter()
        .map(|line| line.iter().filter(|b| **b).count())
        .sum()
}

// https://adventofcode.com/2022/day/8#part2
#[allow(dead_code)]
fn part_02(_console_lines: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_block() {
        [("12\n34\n56", vec![vec![1, 2], vec![3, 4], vec![5, 6]])].map(|(block, expected)| {
            let actual = parse_block(block);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_parse_line() {
        [("12345", vec![1, 2, 3, 4, 5])].map(|(signal, expected)| {
            let actual = parse_line(signal);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_part_01() {
        // https://adventofcode.com/2022/day/8/input
        let contents = fs::read_to_string("./input/day_08.txt").expect("unable to read file");

        [
            (
                concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390"),
                21,
            ),
            (&contents, 1789),
        ]
        .map(|(signal, expected)| {
            let actual = part_01(signal);

            assert_eq!(actual, expected);
        });
    }
}
