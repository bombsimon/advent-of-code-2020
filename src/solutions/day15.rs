use crate::input;

pub fn solve() {
    let x = input::file_for_day(15).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i32 {
    let x = input
        .lines()
        .filter(|&c| c != "")
        .next()
        .unwrap()
        .split(",")
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();

    run(&x, 2020)
}

fn part_two(input: String) -> i32 {
    let x = input
        .lines()
        .filter(|&c| c != "")
        .next()
        .unwrap()
        .split(",")
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();

    run(&x, 30_000_000)
}

fn run(input: &[usize], count: usize) -> i32 {
    let mut map = vec![0; count];
    let mut last_spoken = 0;

    for (turn, &spoken) in input.iter().enumerate() {
        map[spoken] = turn + 1;
        last_spoken = turn + 1;
    }

    for turn in input.len()..count {
        let to_speak = match map[last_spoken] {
            0 => 0,
            last_spoken_at => turn - last_spoken_at,
        };

        map[last_spoken] = turn;
        last_spoken = to_speak;
    }

    last_spoken as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let cases = vec![
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];

        for tc in cases {
            assert_eq!(super::part_one(tc.0.to_string()), tc.1);
        }
    }

    #[test]
    fn part_two() {
        let cases = vec![
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];

        for tc in cases {
            assert_eq!(super::part_two(tc.0.to_string()), tc.1);
        }
    }
}
