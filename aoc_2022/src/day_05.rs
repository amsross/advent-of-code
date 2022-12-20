use crate::helpers::split_every;
use regex::Regex;

#[allow(dead_code)]
fn parse_stacks(lines: Vec<&str>) -> Vec<Vec<char>> {
    let stack_count = lines.get(0).map(|line| (line.len() + 1) / 4).unwrap_or(0);
    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(lines.len()); stack_count];

    let lines = lines.into_iter();

    for line in lines.rev() {
        let chars = (line.to_owned() + " ").chars().collect::<Vec<char>>();
        let groups_of_4_chars = split_every(chars, 4);
        let letters = groups_of_4_chars
            .iter()
            .map(|v| v.get(1).unwrap_or(&' '))
            .collect::<Vec<&char>>();

        for (index, item) in letters.into_iter().enumerate() {
            if item != &' ' {
                stacks.get_mut(index).map(|stack| stack.push(*item));
            }
        }
    }

    stacks
}

#[allow(dead_code)]
fn parse_steps(lines: Vec<&str>) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"move (\d{1,3}) from (\d{1,3}) to (\d{1,3})").unwrap();

    let mut results: Vec<(usize, usize, usize)> = Vec::with_capacity(lines.len());

    for line in lines {
        for cap in re.captures_iter(line) {
            let count = cap[1].parse::<usize>().unwrap();
            let source = cap[2].parse::<usize>().unwrap();
            let destination = cap[3].parse::<usize>().unwrap();

            results.push((count, source, destination));
        }
    }

    results
}

// https://adventofcode.com/2022/day/5
#[allow(dead_code)]
pub fn part_01(contents: String) -> String {
    let mut sections = contents.split_terminator("\n\n");

    let mut stacks = sections
        .next()
        .unwrap()
        .split_terminator("\n")
        .collect::<Vec<&str>>();

    stacks.truncate(stacks.len() - 1);

    let mut stacks = parse_stacks(stacks);

    let steps = sections
        .next()
        .unwrap()
        .split_terminator("\n")
        .collect::<Vec<&str>>();

    let steps = parse_steps(steps);

    for step in steps {
        println!("move {} from {} to {}", step.0, step.1, step.2);

        for n in 1..=step.0 {
            println!("  moving {} from {} to {}", n, step.1, step.2);

            let source = stacks.get_mut(step.1 - 1).unwrap();
            let item = source.pop().unwrap();

            let destination = stacks.get_mut(step.2 - 1).unwrap();
            destination.push(item);
        }
    }

    stacks.iter().fold("".to_string(), |s, stack| {
        let top = stack.get(stack.len() - 1).unwrap_or(&' ').to_string();

        s.to_owned() + &top
    })
}

// https://adventofcode.com/2022/day/5#part2
#[allow(dead_code)]
pub fn part_02(contents: String) -> String {
    let mut sections = contents.split_terminator("\n\n");

    let mut stacks = sections
        .next()
        .unwrap()
        .split_terminator("\n")
        .collect::<Vec<&str>>();

    stacks.truncate(stacks.len() - 1);

    let mut stacks = parse_stacks(stacks);

    let steps = sections
        .next()
        .unwrap()
        .split_terminator("\n")
        .collect::<Vec<&str>>();

    let steps = parse_steps(steps);

    for step in steps {
        println!("move {} from {} to {}", step.0, step.1, step.2);

        println!("  moving {} from {} to {}", step.0, step.1, step.2);

        let source = stacks.get_mut(step.1 - 1).unwrap();
        let mut items = source.split_off(source.len() - step.0);

        let destination = stacks.get_mut(step.2 - 1).unwrap();
        destination.append(&mut items);
    }

    stacks.iter().fold("".to_string(), |s, stack| {
        let top = stack.iter().last().unwrap_or(&' ').to_string();
        let res = s.to_owned() + &top;

        res
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_02() {
        // https://adventofcode.com/2022/day/5/input
        let contents = fs::read_to_string("./input/day_05.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, "TPFFBDRJD");
    }

    #[test]
    fn test_part_01() {
        // https://adventofcode.com/2022/day/5/input
        let contents = fs::read_to_string("./input/day_05.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, "LBLVVTVLP");
    }

    #[test]
    fn test_parse_steps() {
        let result = parse_steps(vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]);

        assert_eq!(result, vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2),]);
    }

    #[test]
    fn test_parse_stacks() {
        let result = parse_stacks(vec!["    [D]    ", "[N] [C]    ", "[Z] [M] [P]"]);

        assert_eq!(
            result,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
        );
    }
}
