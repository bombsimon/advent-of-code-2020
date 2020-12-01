use crate::input;

pub fn solve() {
    let x = input::file_for_day_as_int(2);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(_vec: Vec<i32>) -> i32 {
    -1
}

fn part_two(_vec: Vec<i32>) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r#""#;
    static SOLUTION_ONE: i32 = -1;
    static SOLUTION_TWO: i32 = -1;

    #[test]
    fn part_one() {
        let x = input::string_vec_to_int_vec(TEST_INPUT.split("\n").collect::<Vec<_>>().as_ref());
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::string_vec_to_int_vec(TEST_INPUT.split("\n").collect::<Vec<_>>().as_ref());
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
