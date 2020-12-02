use crate::input;

#[derive(Debug, Clone)]
struct Password {
    letter: String,
    min: i32,
    max: i32,
    password: String,
}

pub fn solve() {
    let x = parse_file(input::file_for_day(2));

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn parse_file(vec: Vec<String>) -> Vec<Password> {
    let mut rule: Vec<Password> = Vec::new();

    for line in vec {
        let split_colon: Vec<&str> = line.split(": ").collect();
        let range_and_letter = split_colon[0];
        let password = split_colon[1].to_string();

        let split_space: Vec<&str> = range_and_letter.split(" ").collect();
        let range = split_space[0];
        let letter = split_space[1].to_string();

        let min_max: Vec<&str> = range.split("-").collect();
        let min = min_max[0].parse::<i32>().unwrap();
        let max = min_max[1].parse::<i32>().unwrap();

        rule.push(Password {
            letter,
            min,
            max,
            password,
        });
    }

    rule
}

fn part_one(vec: Vec<Password>) -> i32 {
    let mut valid = 0i32;

    for p in vec {
        let mut count_letters = 0i32;

        for chr in p.password.chars() {
            if chr.to_string() == p.letter {
                count_letters += 1;
            }
        }

        if count_letters >= p.min && count_letters <= p.max {
            valid += 1;
        }
    }

    valid
}

fn part_two(vec: Vec<Password>) -> i32 {
    let mut valid = 0i32;

    for p in vec {
        let char_at_min = p
            .password
            .chars()
            .nth((p.min - 1) as usize)
            .unwrap_or('0')
            .to_string();

        let char_at_max = p
            .password
            .chars()
            .nth((p.max - 1) as usize)
            .unwrap_or('0')
            .to_string();

        let char_at_min_matches = char_at_min == p.letter && char_at_max != p.letter;
        let char_at_max_matches = char_at_min != p.letter && char_at_max == p.letter;

        if char_at_min_matches || char_at_max_matches {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;
    static SOLUTION_ONE: i32 = 2;
    static SOLUTION_TWO: i32 = 1;

    #[test]
    fn part_one() {
        let x = parse_file(input::string_to_vec(TEST_INPUT));
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = parse_file(input::string_to_vec(TEST_INPUT));
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
