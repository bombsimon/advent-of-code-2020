use crate::input;

pub fn solve() {
    let x = input::file_for_day(11).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

#[derive(Clone, Copy)]
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

fn find_adjecent(
    x: usize,
    y: usize,
    directions: &[Direction],
    input: &[Vec<char>],
) -> Vec<(Direction, (usize, usize))> {
    let mut adjecent: Vec<(Direction, (usize, usize))> = Vec::new();

    for direction in directions {
        match direction {
            Direction::UpLeft => {
                if x != 0 && y != 0 {
                    adjecent.push((Direction::UpLeft, (x - 1, y - 1)));
                }
            }
            Direction::Up => {
                if x != 0 {
                    adjecent.push((Direction::Up, (x - 1, y)));
                }
            }
            Direction::UpRight => {
                if x != 0 && y != input[x as usize].len() - 1 {
                    adjecent.push((Direction::UpRight, (x - 1, y + 1)));
                }
            }
            Direction::Right => {
                if y != input[x as usize].len() - 1 {
                    adjecent.push((Direction::Right, (x, y + 1)));
                }
            }
            Direction::DownRight => {
                if x != input.len() - 1 && y != input[x as usize].len() - 1 {
                    adjecent.push((Direction::DownRight, (x + 1, y + 1)));
                }
            }
            Direction::Down => {
                if x != input.len() - 1 {
                    adjecent.push((Direction::Down, (x + 1, y)));
                }
            }
            Direction::DownLeft => {
                if y != 0 && x != input.len() - 1 {
                    adjecent.push((Direction::DownLeft, (x + 1, y - 1)));
                }
            }
            Direction::Left => {
                if y != 0 {
                    adjecent.push((Direction::Left, (x, y - 1)));
                }
            }
        }
    }

    adjecent
}

fn count_seen_occupied(
    adjecent: (Direction, (usize, usize)),
    recursive: bool,
    input: &[Vec<char>],
) -> usize {
    let direction = adjecent.0.clone();
    let (x, y) = adjecent.1;

    match input[x][y] {
        '#' => 1,
        '.' => {
            if recursive {
                let adjecent = find_adjecent(x, y, &[direction], input);

                if adjecent.len() == 1 {
                    return count_seen_occupied(adjecent[0], true, input);
                }
            }

            0
        }
        _ => 0,
    }
}

fn update_map(input: &mut [Vec<char>], recursive: bool, seen_limit: usize) -> bool {
    let original = input.to_owned().clone();
    let mut did_change = false;

    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == '.' {
                continue;
            }

            let all_directions = vec![
                Direction::UpLeft,
                Direction::Up,
                Direction::UpRight,
                Direction::Right,
                Direction::DownRight,
                Direction::Down,
                Direction::DownLeft,
                Direction::Left,
            ];
            let adj = find_adjecent(x, y, &all_directions, input);

            let mut seen_occupied = 0;
            for a in adj {
                seen_occupied += count_seen_occupied(a, recursive, &original);
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
            if input[x][y] == '#' && seen_occupied >= seen_limit {
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
        if !update_map(&mut original, false, 4) {
            break;
        }
    }

    original.iter().fold(0, |acc, vec| {
        acc + vec.iter().filter(|&&c| c == '#').count() as i64
    })
}

fn part_two(input: String) -> i64 {
    let mut original: Vec<Vec<char>> = input
        .lines()
        .filter(|&l| l != "")
        .map(|l| l.chars().collect())
        .collect();

    loop {
        if !update_map(&mut original, true, 5) {
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
