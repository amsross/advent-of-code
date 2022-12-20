use std::collections::HashMap;

use regex::Regex;

#[derive(PartialEq, Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(PartialEq, Debug)]
enum ConsoleLine {
    Command(Command),
    Directory(String),
    File(String, usize),
}

#[allow(dead_code)]
fn parse_line(line: &str) -> Result<ConsoleLine, String> {
    if line.starts_with('$') {
        let command = line.replace("$ ", "");

        if command == "ls" {
            Ok(ConsoleLine::Command(Command::Ls))
        } else {
            let re = Regex::new(r"cd (.*)").map_err(|_err| "unable to compile regex".to_string());

            let captures = re.and_then(|re| match re.captures(line) {
                Some(captures) => Ok(captures),
                _ => Err("unable to capture on string".to_string()),
            });

            captures.and_then(|captures| {
                let dir = captures[1].parse::<String>();

                match dir {
                    Ok(dir) => Ok(ConsoleLine::Command(Command::Cd(dir))),
                    _ => Err("unable to parse directory".to_string()),
                }
            })
        }
    } else {
        if line.starts_with("dir") {
            let re = Regex::new(r"dir (.*)").map_err(|_err| "unable to compile regex".to_string());

            let captures = re.and_then(|re| match re.captures(line) {
                Some(captures) => Ok(captures),
                _ => Err("unable to capture on string".to_string()),
            });

            captures.and_then(|captures| {
                let dir = captures[1].parse::<String>();

                match dir {
                    Ok(dir) => Ok(ConsoleLine::Directory(dir)),
                    _ => Err("unable to parse directory".to_string()),
                }
            })
        } else {
            let re =
                Regex::new(r"(\d{1,}) (.*)").map_err(|_err| "unable to compile regex".to_string());

            let captures = re.and_then(|re| match re.captures(line) {
                Some(captures) => Ok(captures),
                _ => Err("unable to capture on string".to_string()),
            });

            captures.and_then(|captures| {
                let size = captures[1].parse::<usize>();
                let file = captures[2].parse::<String>();

                match (size, file) {
                    (Ok(size), Ok(file)) => Ok(ConsoleLine::File(file, size)),
                    _ => Err("unable to parse file or size".to_string()),
                }
            })
        }
    }
}

// https://adventofcode.com/2022/day/7
#[allow(dead_code)]
fn get_path_sizes(console_lines: &str) -> HashMap<String, usize> {
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut breadcrumbs = Vec::new();

    for line in console_lines.split_terminator('\n').into_iter() {
        let parsed_line = parse_line(line);

        match parsed_line {
            Ok(ConsoleLine::Command(Command::Cd(dir))) => {
                if dir == ".." {
                    breadcrumbs.pop();
                } else if dir == "/" {
                    breadcrumbs.clear();
                    breadcrumbs.push("".to_owned());
                } else {
                    breadcrumbs.push(dir);
                }

                ()
            }
            Ok(ConsoleLine::File(_name, file_size)) => {
                for i in 0..breadcrumbs.len() {
                    let path = (&breadcrumbs[..=i]).join("/");
                    let path = if path == "" { "/".to_string() } else { path };

                    let mut size: usize = 0;

                    if sizes.contains_key(&path) {
                        size = size + sizes.get(&path).unwrap();
                    }

                    sizes.insert(path, size + file_size);
                }

                ()
            }
            _ => (),
        }
    }

    sizes
}

// https://adventofcode.com/2022/day/7
#[allow(dead_code)]
fn part_01(console_lines: &str) -> usize {
    let sizes = get_path_sizes(console_lines);

    println!("{:#?}", sizes);

    sizes.values().filter(|size| size <= &&1000000).sum()
}

// https://adventofcode.com/2022/day/7#part2
#[allow(dead_code)]
fn part_02(console_lines: &str) -> usize {
    let sizes = get_path_sizes(console_lines);

    println!("{:#?}", sizes);

    let root = dbg!(sizes.get("/").unwrap_or(&0));
    let unused = dbg!(70000000 - root);
    let needed = dbg!(30000000 - unused);

    let min = sizes
        .values()
        .filter(|size| size >= &&needed)
        .min()
        .unwrap_or(&0);

    *min
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_line() {
        [
            (
                "$ cd /",
                Ok(ConsoleLine::Command(Command::Cd("/".to_string()))),
            ),
            ("$ ls", Ok(ConsoleLine::Command(Command::Ls))),
            ("dir a", Ok(ConsoleLine::Directory("a".to_string()))),
            (
                "12345 a.txt",
                Ok(ConsoleLine::File("a.txt".to_string(), 12345)),
            ),
        ]
        .map(|(line, expected)| {
            let actual = parse_line(line);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_part_02() {
        // https://adventofcode.com/2022/day/7/input
        let contents = fs::read_to_string("./input/day_07.txt").expect("unable to read file");

        [(&contents, 24933642)].map(|(signal, expected)| {
            let actual = part_02(signal);

            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_part_01() {
        // https://adventofcode.com/2022/day/7/input
        let contents = fs::read_to_string("./input/day_07.txt").expect("unable to read file");

        [(&contents, 95437)].map(|(signal, expected)| {
            let actual = part_01(signal);

            assert_eq!(actual, expected);
        });
    }
}
