#[derive(PartialEq, Clone, Debug)]
enum LevelDirection {
    INCREASING,
    DECREASING,
}

// Any two adjacent levels that differ by at least one and at most three are safe.
fn level_is_safe(direction: LevelDirection, previous_level: i32, current_level: i32) -> bool {
    if current_level == previous_level {
        return false;
    }

    match direction {
        LevelDirection::INCREASING => {
            return current_level > previous_level && current_level <= previous_level + 3
        }
        LevelDirection::DECREASING => {
            return current_level < previous_level && current_level >= previous_level - 3
        }
    }
}

fn unsafe_level_count(levels: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previous_level = levels[0];
    let mut direction: Option<LevelDirection> = None;

    for i in 1..levels.len() {
        let current_level = levels[i];

        if direction == None {
            if current_level > previous_level {
                direction = Some(LevelDirection::INCREASING);
            } else {
                direction = Some(LevelDirection::DECREASING);
            }
        }

        let is_safe = level_is_safe(direction.clone().unwrap(), previous_level, current_level);

        if !is_safe {
            count += 1;
        } else {
            previous_level = current_level;
        }
    }

    count
}

fn is_report_safe(threshold: i32, report: &str) -> bool {
    let levels = report
        .split(" ")
        .map_while(|level| level.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    if levels.len() < 1 {
        return false;
    }

    let unsafe_levels = unsafe_level_count(levels.clone());
    if unsafe_levels < threshold {
        return true;
    }

    for i in 0..levels.len() {
        let (left, right) = levels.split_at(i);
        let mut left = left.to_vec();
        let mut right = right
            .split_first()
            .map(|(_, rest)| rest.to_vec())
            .unwrap_or(Vec::new());

        left.append(&mut right);

        let unsafe_levels = unsafe_level_count(left.clone()) + 1;
        if unsafe_levels < threshold {
            return true;
        }
    }

    false
}

// https://adventofcode.com/2024/day/1
#[allow(dead_code)]
pub fn part_01(reports: String) -> i32 {
    let mut safe_reports = 0;

    for report in reports.lines() {
        if is_report_safe(1, report) {
            safe_reports += 1;
        }
    }

    safe_reports
}

// https://adventofcode.com/2024/day/1#part2
#[allow(dead_code)]
pub fn part_02(reports: String) -> i32 {
    let mut safe_reports = 0;

    for report in reports.lines() {
        if is_report_safe(2, report) {
            safe_reports += 1;
        }
    }

    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::day_02::LevelDirection::{DECREASING, INCREASING};
    use std::fs;

    #[test]
    fn test_level_is_safe() {
        assert_eq!(level_is_safe(DECREASING, 5, 0), false, "5, 0");
        assert_eq!(level_is_safe(DECREASING, 5, 1), false, "5, 1");
        assert_eq!(level_is_safe(DECREASING, 5, 2), true, "5, 2");
        assert_eq!(level_is_safe(DECREASING, 5, 3), true, "5, 3");
        assert_eq!(level_is_safe(DECREASING, 5, 4), true, "5, 4");
        assert_eq!(level_is_safe(DECREASING, 5, 5), false, "5, 5");

        assert_eq!(level_is_safe(INCREASING, 5, 5), false, "5, 5");
        assert_eq!(level_is_safe(INCREASING, 5, 6), true, "5, 6");
        assert_eq!(level_is_safe(INCREASING, 5, 7), true, "5, 7");
        assert_eq!(level_is_safe(INCREASING, 5, 8), true, "5, 8");
        assert_eq!(level_is_safe(INCREASING, 5, 9), false, "5, 9");
        assert_eq!(level_is_safe(INCREASING, 5, 10), false, "5, 10");
    }

    #[test]
    fn test_part_01_01() {
        // https://adventofcode.com/2024/day/2
        let contents: String = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_owned();

        let result = part_01(contents);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_01_02() {
        // https://adventofcode.com/2024/day/2/input
        let contents = fs::read_to_string("./input/day_02_1.txt").expect("unable to read file");

        let result = part_01(contents);

        assert_eq!(result, 230);
    }

    #[test]
    fn test_part_02_01() {
        // https://adventofcode.com/2024/day/2
        assert_eq!(part_02("7 6 4 2 1".to_owned()), 1);
        assert_eq!(part_02("1 2 7 8 9".to_owned()), 0);
        assert_eq!(part_02("9 7 6 2 1".to_owned()), 0);
        assert_eq!(part_02("1 3 2 4 5".to_owned()), 1);
        assert_eq!(part_02("8 6 4 4 1".to_owned()), 1);
        assert_eq!(part_02("1 3 6 7 9".to_owned()), 1);

        assert_eq!(
            part_02(
                "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n".to_owned()
            ),
            4
        );

        assert_eq!(part_02("0 4 6 7 9".to_owned()), 1);
        assert_eq!(part_02("9 3 6 7 9".to_owned()), 1);
    }

    #[test]
    fn test_part_02_02() {
        // https://adventofcode.com/2024/day/2/input
        let contents = fs::read_to_string("./input/day_02_2.txt").expect("unable to read file");

        let result = part_02(contents);

        assert_eq!(result, 301);
    }
}
