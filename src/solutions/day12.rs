use crate::{grid::Direction, input};

#[derive(Clone, Debug)]
struct Walker {
    instructions: Vec<(Direction, isize)>,
    instructions_ptr: usize,
    facing: Direction,
    waypoint: (isize, isize),
    coordinates: (isize, isize),
}

impl Walker {
    fn new(instructions: Vec<(Direction, isize)>) -> Self {
        Walker {
            instructions,
            instructions_ptr: 0usize,
            facing: Direction::East,
            waypoint: (-1, 10),
            coordinates: (0, 0),
        }
    }

    fn next(&mut self) {
        let (ins, len) = self.instructions[self.instructions_ptr];
        let new_coordinates = match ins {
            Direction::Unknown('F') => self.facing.next_from(self.coordinates, len),
            Direction::Unknown('L') => {
                // Just turn right and another 180 degrees
                if len != 180 {
                    self.facing = self.facing.turn_deegrees(180);
                }

                self.facing = self.facing.turn_deegrees(len);
                self.instructions_ptr += 1;
                return;
            }
            Direction::Unknown('R') => {
                self.facing = self.facing.turn_deegrees(len);
                self.instructions_ptr += 1;
                return;
            }
            direction => direction.next_from(self.coordinates, len as isize),
        };

        self.coordinates = new_coordinates;
        self.instructions_ptr += 1;
    }

    fn next_with_waypoint(&mut self) {
        let (ins, len) = self.instructions[self.instructions_ptr];
        match ins {
            Direction::Unknown('F') => {
                for _ in 0..len {
                    self.coordinates.0 += self.waypoint.0;
                    self.coordinates.1 += self.waypoint.1;
                }
            }
            Direction::Unknown('L') => {
                for _ in 0..len / 90 {
                    self.waypoint = (-self.waypoint.1, self.waypoint.0)
                }
            }
            Direction::Unknown('R') => {
                for _ in 0..len / 90 {
                    self.waypoint = (self.waypoint.1, -self.waypoint.0)
                }
            }
            direction => {
                self.waypoint = direction.next_from(self.waypoint, len);
            }
        };

        self.instructions_ptr += 1;
    }

    fn done(&self) -> bool {
        self.instructions_ptr == self.instructions.len()
    }

    fn manhattan(&self) -> i32 {
        (0i32 - self.coordinates.0 as i32).abs() + (0i32 - self.coordinates.1 as i32).abs()
    }
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "at x: {}, y: {}, facing: {:?}, waypoint x: {}, waypoint y: {}",
            self.coordinates.0, self.coordinates.1, self.facing, self.waypoint.0, self.waypoint.1,
        )
    }
}

pub fn solve() {
    let x = input::file_for_day(12).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i32 {
    let sequence = input
        .lines()
        .filter(|&c| c != "")
        .map(|l| {
            let ins = l.chars().next().unwrap();
            let len = l.get(1..).unwrap().parse::<isize>().unwrap();

            (Direction::from(ins), len)
        })
        .collect::<Vec<(Direction, isize)>>();

    let mut walker = Walker::new(sequence);
    while !walker.done() {
        walker.next();
    }

    walker.manhattan()
}

fn part_two(input: String) -> i32 {
    let sequence = input
        .lines()
        .filter(|&c| c != "")
        .map(|l| {
            let ins = l.chars().next().unwrap();
            let len = l.get(1..).unwrap().parse::<isize>().unwrap();

            (Direction::from(ins), len)
        })
        .collect::<Vec<(Direction, isize)>>();

    let mut walker = Walker::new(sequence);
    while !walker.done() {
        walker.next_with_waypoint();
    }

    walker.manhattan()
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r#"
F10
N3
F7
R90
F11"#;

    static SOLUTION_ONE: i32 = 25;
    static SOLUTION_TWO: i32 = 286;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
