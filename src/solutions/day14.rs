use crate::input;

use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    let x = input::file_for_day(14).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    let bit_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let x = input
        .lines()
        .filter(|&c| c != "")
        .filter_map(|l| {
            if l.starts_with("mask") {
                let mask = l.split(" = ").collect::<Vec<_>>()[1]
                    .chars()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, v)| match v {
                        '0' | '1' => Some((i, v as i64 - '0' as i64)),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                Some((0, mask))
            } else {
                let captures = bit_re.captures(l).unwrap();
                let address = &captures[1].parse::<i64>().unwrap();
                let value = &captures[2].parse::<i64>().unwrap();

                Some((*address, vec![(0, *value)]))
            }
        })
        .collect::<Vec<_>>();

    let mut mask = Vec::new();
    let mut data: HashMap<i64, i64> = HashMap::new();

    for (n, y) in x.iter() {
        if *n == 0 {
            mask = y.clone();
            continue;
        }

        let mut value = y[0].1;
        for (pos, val) in mask.iter() {
            match val {
                0 => value &= !(1 << pos),
                1 => value |= 1 << pos,
                _ => panic!("Unknown {}", val),
            }
        }

        data.insert(*n, value);
    }

    data.iter().fold(0, |acc, (_, v)| acc + v)
}

fn part_two(input: String) -> i64 {
    let bit_re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let x = input
        .lines()
        .filter(|&c| c != "")
        .map(|l| {
            if l.starts_with("mask") {
                (0, l.split(" = ").collect::<Vec<_>>()[1].to_string())
            } else {
                let captures = bit_re.captures(l).unwrap();
                let address = &captures[1].parse::<i64>().unwrap();
                let value = &captures[2];

                (*address, value.to_string())
            }
        })
        .collect::<Vec<_>>();

    let mut mask = Vec::new();
    let mut data: HashMap<i64, i64> = HashMap::new();

    for (mem, val) in x.iter() {
        if *mem == 0 {
            mask = val
                .chars()
                .rev()
                .enumerate()
                .map(|(i, v)| (i, v))
                .collect::<Vec<_>>();

            continue;
        }

        let mut value = *mem;
        let mut perm = Vec::new();

        for (pos, n) in mask.iter() {
            match n {
                '0' => (),
                '1' => value |= 1 << pos,
                _ => perm.push(pos),
            }
        }

        let addrs = perm.iter().fold(vec![value], |acc, &pos| {
            let mut inner = Vec::new();

            for addr in acc {
                inner.push(addr & !(1 << pos));
                inner.push(addr | 1 << pos);
            }

            inner
        });

        for a in addrs {
            data.insert(a, val.parse::<i64>().unwrap());
        }
    }

    data.iter().fold(0, |acc, (_, v)| acc + v)
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_ONE: &str = r#"
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"#;
    static TEST_INPUT_TWO: &str = r#"
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"#;

    static SOLUTION_ONE: i64 = 165;
    static SOLUTION_TWO: i64 = 208;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT_ONE.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT_TWO.to_string()), SOLUTION_TWO);
    }
}
