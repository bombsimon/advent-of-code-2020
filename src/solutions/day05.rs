use crate::input;

#[derive(Debug)]
enum BinarySide {
    Upper,
    Lower,
}

pub fn solve() {
    let x = input::file_for_day(5);
    let l = parse_file(x);

    println!("Solution part 1: {}", part_one(l.clone()));
    println!("Solution part 2: {}", part_two(l.clone()));
}

fn binary_find(vec: &Vec<BinarySide>, size: i64) -> i64 {
    let mut lower = 0f64;
    let mut upper = size as f64;

    for side in vec {
        match side {
            BinarySide::Upper => lower = ((lower + upper) / 2.).ceil(),
            BinarySide::Lower => upper = (lower + upper) / 2.,
        }
    }

    lower as i64
}

fn parse_file(vec: Vec<String>) -> Vec<i64> {
    let mut seats: Vec<i64> = Vec::new();

    for l in vec {
        let mut row = Vec::new();
        let mut seat = Vec::new();
        let (r, s) = l.split_at(7);

        for c in r.chars() {
            match c {
                'F' => row.push(BinarySide::Lower),
                'B' => row.push(BinarySide::Upper),
                x => panic!(format!("Unexpected input: {}", x)),
            }
        }

        for c in s.chars() {
            match c {
                'L' => seat.push(BinarySide::Lower),
                'R' => seat.push(BinarySide::Upper),
                x => panic!(format!("Unexpected input: {}", x)),
            }
        }

        let row_id = binary_find(&row, 127);
        let seat_id = binary_find(&seat, 7);
        let result = row_id * 8 + seat_id;

        seats.push(result);
    }

    seats
}

fn part_one(vec: Vec<i64>) -> i64 {
    *vec.iter().max().unwrap()
}

fn part_two(mut vec: Vec<i64>) -> i64 {
    if vec.len() == 0 {
        return -1;
    }

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
    static TEST_INPUT_TWO: &str = r#""#;

    static SOLUTION_ONE: i64 = 820;
    static SOLUTION_TWO: i64 = -1;

    #[test]
    fn part_one() {
        let x = input::string_to_vec(TEST_INPUT_ONE, false);
        let l = parse_file(x);

        assert_eq!(super::part_one(l), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::string_to_vec(TEST_INPUT_TWO, false);
        let l = parse_file(x);

        assert_eq!(super::part_two(l), SOLUTION_TWO);
    }
}
