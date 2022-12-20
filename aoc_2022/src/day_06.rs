// https://adventofcode.com/2022/day/6
#[allow(dead_code)]
pub fn part_01(signal: &str) -> usize {
    let letters = signal.chars();
    println!("{:#?}", signal);

    let mut marker: Vec<char> = Vec::with_capacity(4);
    let mut start = 0;

    for (index, letter) in letters.enumerate() {
        if marker.len().eq(&4) {
            start = index;
            break;
        }

        if marker.contains(&letter) {
            println!("contains {:#?}", letter);

            let mut match_found = false;
            marker.retain(|&x| {
                let letter_matches = !match_found && x.eq(&letter);
                match_found = letter_matches || match_found;

                match_found && !letter_matches
            });

            println!("remaining: {:#?}", marker);
        } else {
            println!("does not contain {:#?}", letter);
        }

        marker.push(letter);
    }

    println!("{:#?}", start);
    println!("{:#?}", marker);

    start
}

// https://adventofcode.com/2022/day/6#part2
#[allow(dead_code)]
pub fn part_02(signal: &str) -> usize {
    let letters = signal.chars();
    println!("{:#?}", signal);

    let mut marker: Vec<char> = Vec::with_capacity(14);
    let mut start = 0;

    for (index, letter) in letters.enumerate() {
        if marker.len().eq(&14) {
            start = index;
            break;
        }

        if marker.contains(&letter) {
            println!("contains {:#?}", letter);

            let mut match_found = false;
            marker.retain(|&x| {
                let letter_matches = !match_found && x.eq(&letter);
                match_found = letter_matches || match_found;

                match_found && !letter_matches
            });

            println!("remaining: {:#?}", marker);
        } else {
            println!("does not contain {:#?}", letter);
        }

        marker.push(letter);
    }

    println!("{:#?}", start);
    println!("{:#?}", marker);

    start
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_02() {
        // https://adventofcode.com/2022/day/6/input
        let contents = fs::read_to_string("./input/day_06.txt").expect("unable to read file");

        [
            // ("mjqjp qmgbljsphdztnv jfqwrcgsmlb", 19),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
            (&contents, 2635),
        ]
        .map(|(signal, expected)| {
            let actual = part_02(signal);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_part_01() {
        // https://adventofcode.com/2022/day/6/input
        let contents = fs::read_to_string("./input/day_06.txt").expect("unable to read file");

        [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
            (&contents, 1779),
        ]
        .map(|(signal, expected)| {
            let actual = part_01(signal);

            assert_eq!(actual, expected);
        });
    }
}
