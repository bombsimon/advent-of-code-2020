use crate::input;

pub fn solve() {
    let x = input::file_for_day(11).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

fn get_adjecent(x: usize, y: usize, input: &[Vec<char>]) -> Vec<(Direction, char, (usize, usize))> {
    let mut adj: Vec<(Direction, char, (usize, usize))> = Vec::new();

    if x > 0 {
        adj.push((Direction::Up, input[x - 1][y], (x - 1usize, y)));

        if y > 0 {
            adj.push((
                Direction::UpLeft,
                input[x - 1][y - 1],
                (x - 1usize, y - 1usize),
            ));
        }

        if y < input[x].len() - 1 {
            adj.push((
                Direction::UpRight,
                input[x - 1][y + 1],
                (x - 1usize, y + 1usize),
            ));
        }
    }

    if y < input[x].len() - 1 {
        adj.push((Direction::Right, input[x][y + 1], (x, y + 1usize)));
    }

    if y > 0 {
        adj.push((Direction::Left, input[x][y - 1], (x, y - 1usize)));
    }

    if x < input.len() - 1 {
        adj.push((Direction::Down, input[x + 1][y], (x + 1usize, y)));

        if y > 0 {
            adj.push((
                Direction::DownLeft,
                input[x + 1][y - 1],
                (x + 1usize, y - 1usize),
            ));
        }

        if y < input[x].len() - 1 {
            adj.push((
                Direction::DownRight,
                input[x + 1][y + 1],
                (x + 1usize, y + 1usize),
            ));
        }
    }

    adj
}

fn replace_map(input: &mut [Vec<char>]) -> bool {
    let original = input.to_owned().clone();
    let mut did_change = false;

    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == '.' {
                continue;
            }

            let adj = get_adjecent(x, y, &original);
            let mut seen_occupied = 0;
            for (_, t, _) in adj {
                if t == '#' {
                    seen_occupied += 1;
                }
            }

            // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat
            // becomes occupied.
            if input[x][y] == 'L' && seen_occupied == 0 {
                input[x][y] = '#';

                did_change = true;
                continue;
            }

            // If a seat is occupied (#) and four or more seats adjacent to it are also occupied,
            // the seat becomes empty.
            if input[x][y] == '#' && seen_occupied >= 4 {
                input[x][y] = 'L';

                did_change = true;
            }
        }
    }

    did_change
}

fn part_one(input: String) -> i64 {
    let mut original: Vec<Vec<char>> = input
        .lines()
        .filter(|&l| l != "")
        .map(|l| l.chars().collect())
        .collect();

    loop {
        if !replace_map(&mut original) {
            break;
        }
    }

    original.iter().fold(0, |acc, vec| {
        acc + vec.iter().filter(|&&c| c == '#').count() as i64
    })
}

fn traverse_direction(a: (Direction, char, (usize, usize)), input: &[Vec<char>]) -> usize {
    let mut seen = 0;
    let is_in_bounds = |(dir, _, (x, y))| -> Option<(Direction, char, (usize, usize))> {
        match dir {
            Direction::UpLeft => {
                if x == 0 || y == 0 {
                    return None;
                }

                let new_x: usize = x - 1;
                let new_y: usize = y - 1;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
            Direction::Up => {
                if x == 0 {
                    return None;
                }

                let new_x: usize = x - 1;
                let new_y: usize = y;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
            Direction::UpRight => {
                if x == 0 || y == input[x as usize].len() - 1 {
                    return None;
                }

                let new_x: usize = x - 1;
                let new_y: usize = y + 1;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
            Direction::Right => {
                if y == input[x as usize].len() - 1 {
                    return None;
                }

                let new_x: usize = x;
                let new_y: usize = y + 1;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
            Direction::DownRight => {
                if x == input.len() - 1 || y == input[x as usize].len() - 1 {
                    return None;
                }

                let new_x: usize = x + 1;
                let new_y: usize = y + 1;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
            Direction::Down => {
                if x == input.len() - 1 {
                    return None;
                }

                let new_x: usize = x + 1;
                let new_y: usize = y;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
            Direction::DownLeft => {
                if y == 0 || x == input.len() - 1 {
                    return None;
                }

                let new_x: usize = x + 1;
                let new_y: usize = y - 1;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
            Direction::Left => {
                if y == 0 {
                    return None;
                }

                let new_x: usize = x;
                let new_y: usize = y - 1;
                let new_char = input[new_x][new_y];

                return Some((dir, new_char, (new_x, new_y)));
            }
        }
    };

    match a.1 {
        '#' => seen += 1,
        '.' => match is_in_bounds(a) {
            Some(x) => seen += traverse_direction(x, input),
            None => (),
        },
        _ => (),
    };

    seen
}

fn replace_map_two(input: &mut [Vec<char>]) -> bool {
    let original = input.to_owned().clone();
    let mut did_change = false;

    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == '.' {
                continue;
            }

            let adj = get_adjecent(x, y, &original);
            let mut seen_occupied = 0;
            for a in adj {
                seen_occupied += traverse_direction(a, &original);
            }

            // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat
            // becomes occupied.
            if input[x][y] == 'L' && seen_occupied == 0 {
                input[x][y] = '#';

                did_change = true;
                continue;
            }

            // If a seat is occupied (#) and four or more seats adjacent to it are also occupied,
            // the seat becomes empty.
            if input[x][y] == '#' && seen_occupied >= 5 {
                input[x][y] = 'L';

                did_change = true;
            }
        }
    }

    did_change
}

fn part_two(input: String) -> i64 {
    let mut original: Vec<Vec<char>> = input
        .lines()
        .filter(|&l| l != "")
        .map(|l| l.chars().collect())
        .collect();

    loop {
        if !replace_map_two(&mut original) {
            break;
        }
    }

    original.iter().fold(0, |acc, vec| {
        acc + vec.iter().filter(|&&c| c == '#').count() as i64
    })
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r#"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

    static SOLUTION_ONE: i64 = 37;
    static SOLUTION_TWO: i64 = 26;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
