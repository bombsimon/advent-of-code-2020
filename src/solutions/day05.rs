use crate::input;

pub fn solve() {
    let x = input::file_for_day(5);
    let mut l = parse_file(x);

    println!("Solution part 1: {}", part_one(&l));
    println!("Solution part 2: {}", part_two(&mut l));
}

fn parse_file(vec: Vec<String>) -> Vec<i64> {
    vec.iter()
        .map(|s| -> i64 {
            let n: String = s
                .chars()
                .map(|c| -> char {
                    if c == 'B' || c == 'R' {
                        return '1';
                    }
                    '0'
                })
                .collect();
            i64::from_str_radix(&n, 2).unwrap()
        })
        .collect()
}

fn part_one(vec: &Vec<i64>) -> i64 {
    *vec.iter().max().unwrap()
}

fn part_two(vec: &mut Vec<i64>) -> i64 {
    vec.sort();

    let first = *vec.first().unwrap();
    let last = *vec.last().unwrap();

    for x in first..last {
        if !vec.contains(&x) {
            return x;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_ONE: &str = r#"
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
"#;

    static SOLUTION_ONE: i64 = 820;

    #[test]
    fn part_one() {
        let x = input::string_to_vec(TEST_INPUT_ONE, false);
        let l = parse_file(x);

        assert_eq!(super::part_one(&l), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(1, 1);
    }
}
