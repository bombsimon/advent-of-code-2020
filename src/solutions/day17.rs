use std::collections::HashMap;

use crate::{grid, input};

// TODO: We're counting neighbors way to often, that should be fixed and dramatically reduce time.
// We should also be able to not have to pre-populate the outer layer before each cycle, that's
// just a dirty workaround. If we can skip this that would also reduce a lot of time.
pub fn solve() {
    let x = input::file_for_day(17).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    let mut cube_map: HashMap<grid::CubeN, char> = HashMap::new();
    for (x, line) in input.lines().filter(|&c| c != "").enumerate() {
        for (y, value) in line.chars().enumerate() {
            cube_map.insert(
                grid::CubeN {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: None,
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

fn part_two(input: String) -> i64 {
    // TODO: Would probably make sense to use a different data structure than this custom cube with
    // not that great multi dimension support.
    let mut cube_map: HashMap<grid::CubeN, char> = HashMap::new();
    for (x, line) in input.lines().filter(|&c| c != "").enumerate() {
        for (y, value) in line.chars().enumerate() {
            cube_map.insert(
                grid::CubeN {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: Some(0),
                },
                value,
            );
        }
    }

    for _ in 1..=6 {
        // TODO: This whole paragraph should be skipped. We shouldn't have to get ALL neighbors
        // before we even iterate to check for visible neighbors. And even if we did, we don't have
        // to get neighbors for anything but the outer most layer.
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
