use std::collections::HashMap;

// https://adventofcode.com/2024/day/1
#[allow(dead_code)]
pub fn part_01(contents: String) -> i32 {
    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();

        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        lefts.push(left.parse::<i32>().unwrap());
        rights.push(right.parse::<i32>().unwrap());
    }

    lefts.sort();
    rights.sort();

    let mut distance: i32 = 0;

    for i in 0..lefts.len() {
        let left = lefts.get(i).unwrap();
        let right = rights.get(i).unwrap();

        distance += (left - right).abs();
    }

    println!("{}", distance);

    distance.try_into().unwrap()
}

// https://adventofcode.com/2024/day/1#part2
#[allow(dead_code)]
pub fn part_02(contents: String) -> i32 {
    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: HashMap<i32, i32> = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();

        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        println!("{} - {}", left, right);

        lefts.push(left.parse::<i32>().unwrap());

        let right = right.parse::<i32>().unwrap();
        let mut rights_count = rights.get(&right).unwrap_or(&0).to_owned();
        rights_count += 1;
        rights.insert(right, rights_count);
    }

    let mut result = 0;

    for left in lefts.iter() {
        let right_count = rights.get(&left).unwrap_or(&0).to_owned();

        result += left * right_count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01_01() {
        // https://adventofcode.com/2024/day/1
        let contents: String = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_owned();

        let result = part_01(contents);

        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_01_02() {
        // https://adventofcode.com/2024/day/1/input
        let contents = fs::read_to_string("./input/day_01_1.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 1722302);
    }

    #[test]
    fn test_part_02_01() {
        // https://adventofcode.com/2024/day/1
        let contents: String = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_owned();

        let result = part_02(contents);

        assert_eq!(result, 31);
    }

    #[test]
    fn test_part_02_02() {
        // https://adventofcode.com/2024/day/1/input
        let contents = fs::read_to_string("./input/day_01_2.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, 20373490);
    }
}
