#[allow(dead_code)]
pub fn string_to_matrix(string: String) -> Vec<Vec<char>> {
    string.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
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
}
