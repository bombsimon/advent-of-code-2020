use std::collections::HashMap;

use crate::{grid, input};

pub fn solve() {
    let x = input::file_for_day(17).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    let mut cube_map: HashMap<grid::Cube, char> = HashMap::new();
    for (x, line) in input.lines().filter(|&c| c != "").enumerate() {
        for (y, value) in line.chars().enumerate() {
            cube_map.insert(
                grid::Cube {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                },
                value,
            );
        }
    }

    for _ in 1..=6 {
        let mut prefilled = cube_map.clone();
        for (cube, v) in &cube_map {
            prefilled.insert(cube.clone(), *v);

            let neighbors = cube.neighbors();

            for n in &neighbors {
                if !cube_map.contains_key(&n) {
                    prefilled.insert(n.clone(), '.');
                }
            }
        }

        let mut cube_map_copy = prefilled.clone();
        for (cube, state) in &prefilled {
            let neighbors = cube.neighbors();

            let active = neighbors.iter().fold(0, |acc, n| match cube_map.get(&n) {
                Some(&v) => acc + if v == '#' { 1 } else { 0 },
                None => acc,
            });

            match state {
                '#' => {
                    if active != 2 && active != 3 {
                        cube_map_copy.insert(cube.clone(), '.');
                    }
                }
                '.' => {
                    if active == 3 {
                        cube_map_copy.insert(cube.clone(), '#');
                    }
                }
                _ => unreachable!(),
            };
        }

        cube_map = cube_map_copy;
    }

    cube_map
        .iter()
        .fold(0, |acc, (_, &v)| acc + if v == '#' { 1 } else { 0 })
}

fn part_two(_input: String) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r#"
.#.
..#
###
"#;

    static SOLUTION_ONE: i64 = 112;
    static SOLUTION_TWO: i64 = 848;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
