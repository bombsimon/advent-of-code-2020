use crate::input;

#[derive(Clone, PartialEq)]
enum MapItem {
    Square,
    Tree,
}

pub fn solve() {
    let x = input::file_for_day(3);
    let map = parse_file(x);

    println!("Solution part 1: {}", part_one(map.clone()));
    println!("Solution part 2: {}", part_two(map.clone()));
}

fn parse_file(vec: Vec<String>) -> Vec<Vec<MapItem>> {
    let mut map: Vec<Vec<MapItem>> = Vec::new();

    for x in vec {
        let mut row: Vec<MapItem> = Vec::new();

        for y in x.chars() {
            match y {
                '.' => row.push(MapItem::Square),
                '#' => row.push(MapItem::Tree),
                p => panic!(format!("Unexpected character: '{}'", p)),
            }
        }

        map.push(row.clone());
    }

    map
}

fn trees_seen_for_step(vec: &[Vec<MapItem>], rstep: usize, dstep: usize) -> i64 {
    let mut row = 0;
    let mut col = 0;
    let mut trees_seen = 0i64;

    while row < vec.len() {
        if vec[row][col] == MapItem::Tree {
            trees_seen += 1;
        }

        col += rstep;
        if col > vec[row].len() - 1 {
            col -= vec[row].len();
        }

        row += dstep;
    }

    trees_seen
}

fn part_one(vec: Vec<Vec<MapItem>>) -> i64 {
    trees_seen_for_step(&vec, 3, 1)
}

fn part_two(vec: Vec<Vec<MapItem>>) -> i64 {
    let step_pairs = vec![vec![3, 1], vec![5, 1], vec![7, 1], vec![1, 2]];
    let mut sum = trees_seen_for_step(&vec, 1, 1);

    for pair in step_pairs {
        sum *= trees_seen_for_step(&vec, pair[0], pair[1]);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;
    static SOLUTION_ONE: i64 = 7;
    static SOLUTION_TWO: i64 = 336;

    #[test]
    fn part_one() {
        let x = input::string_to_vec(TEST_INPUT);
        let map = parse_file(x);

        assert_eq!(super::part_one(map), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::string_to_vec(TEST_INPUT);
        let map = parse_file(x);

        assert_eq!(super::part_two(map), SOLUTION_TWO);
    }
}
