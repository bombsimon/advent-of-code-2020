#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Unknown(char),
}

impl From<char> for Direction {
    fn from(s: char) -> Self {
        match s {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            n => Direction::Unknown(n),
        }
    }
}

impl Direction {
    pub fn turn_deegrees(&self, degrees: usize) -> Self {
        match self {
            Direction::North => match degrees {
                90 => Direction::East,
                180 => Direction::South,
                270 => Direction::West,
                n => panic!("not implemented for {}", n),
            },
            Direction::South => match degrees {
                90 => Direction::West,
                180 => Direction::North,
                270 => Direction::East,
                n => panic!("not implemented for {}", n),
            },
            Direction::East => match degrees {
                90 => Direction::South,
                180 => Direction::West,
                270 => Direction::North,
                n => panic!("not implemented for {}", n),
            },
            Direction::West => match degrees {
                90 => Direction::North,
                180 => Direction::East,
                270 => Direction::South,
                n => panic!("not implemented for {}", n),
            },
            _ => panic!("Nope"),
        }
    }

    pub fn next_from(&self, x: usize, y: usize, distance: usize) -> (usize, usize) {
        match self {
            Direction::North => (x - distance, y),
            Direction::West => (x, y - distance),
            Direction::East => (x, y + distance),
            Direction::South => (x + distance, y),
            Direction::Unknown(f) => panic!("Unexpected facing direction: {}", f),
        }
    }
}
