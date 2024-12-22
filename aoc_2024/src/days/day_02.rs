#[derive(PartialEq, Clone)]
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

fn report_is_safe(report: &str) -> bool {
    let mut safe = true;
    let mut levels = report.split(" ");
    let mut previous_level = levels.next().unwrap().parse::<i32>().unwrap();
    let mut direction: Option<LevelDirection> = None;

    while let Some(level) = levels.next() {
        let current_level = level.parse::<i32>().unwrap();

        if direction == None {
            if current_level > previous_level {
                direction = Some(LevelDirection::INCREASING);
            } else {
                direction = Some(LevelDirection::DECREASING);
            }
        }

        safe = safe && level_is_safe(direction.clone().unwrap(), previous_level, current_level);
        previous_level = current_level;
    }

    safe
}

// https://adventofcode.com/2024/day/1
#[allow(dead_code)]
pub fn part_01(reports: String) -> i32 {
    let mut safe_reports = 0;

    for report in reports.lines() {
        if report_is_safe(report) {
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
}
