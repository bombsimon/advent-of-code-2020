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
