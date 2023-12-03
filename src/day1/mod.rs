use regex::Regex;

pub fn solve() {
    let input = include_str!("./input.txt");
    let res = decrypt_lines(input);
    dbg!(res);

    let res = decrypt_lines2(input);
    dbg!(res);
}

fn decrypt_lines(input: &str) -> u32 {
    let lines = input.split("\n");
    lines.map(decrypt_line).sum()
}

fn decrypt_lines2(input: &str) -> u32 {
    let lines = input.split("\n");

    lines.map(decrypt_line2).sum()
}

fn decrypt_line(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    first * 10 + last
}

fn decrypt_line2(line: &str) -> u32 {
    dbg!(line);
    let digits = get_digits(line);
    dbg!(&digits);
    let first = digits.0;
    let last = digits.1;

    first * 10 + last
}

fn rev(line: &str) -> String {
    line.chars().rev().collect()
}

fn get_digits(line: &str) -> (u32, u32) {
    let regex = Regex::new(r"(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reverse_regex =
        Regex::new(&r"(\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|orez)").unwrap();

    let rev_line: String = rev(line);

    let first = word_to_digit(regex.find(line).unwrap().as_str());
    let last = word_to_digit(&rev(&reverse_regex.find(&rev_line).unwrap().as_str()));

    (first, last)
}

fn word_to_digit(word: &str) -> u32 {
    if word.len() == 1 {
        if let Some(value) = word.chars().nth(0).and_then(|c| c.to_digit(10)) {
            return value;
        }
    }

    match word.to_lowercase().as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("non valid digit"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_line_returns_number() {
        assert_eq!(decrypt_line("1ab3c2"), 12);
    }

    #[test]
    fn decrypt_lines_returns_number() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(decrypt_lines(input), 142);
    }

    #[test]
    fn get_digits_reutrns_word_digits() {
        let input = "two1blanine5";
        let result = get_digits(input);
        assert_eq!(result, (2, 5));

        let input = "three98oneightzn";
        let result = get_digits(input);
        assert_eq!(result, (3, 8));
    }

    #[test]
    fn decrypt_lines_2_returns_number() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(decrypt_lines2(input), 281);
    }
}
