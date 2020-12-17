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
    pub fn turn_deegrees(&self, degrees: isize) -> Self {
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

    pub fn next_from(&self, coordinates: (isize, isize), distance: isize) -> (isize, isize) {
        let (x, y) = coordinates;

        match self {
            Direction::North => (x - distance, y),
            Direction::West => (x, y - distance),
            Direction::East => (x, y + distance),
            Direction::South => (x + distance, y),
            Direction::Unknown(f) => panic!("Unexpected facing direction: {}", f),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct CubeN {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: Option<i32>,
}

impl Eq for CubeN {}

impl CubeN {
    pub fn neighbors(&self) -> Vec<CubeN> {
        let mut n = Vec::new();
        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                for z in self.z - 1..=self.z + 1 {
                    match self.w {
                        Some(w) => {
                            for w in w - 1..=w + 1 {
                                if self.x == x && self.y == y && self.z == z && self.w == Some(w) {
                                    continue;
                                }

                                n.push(CubeN {
                                    x,
                                    y,
                                    z,
                                    w: Some(w),
                                });
                            }
                        }
                        None => {
                            if self.x == x && self.y == y && self.z == z {
                                continue;
                            }

                            n.push(CubeN { x, y, z, w: None });
                        }
                    };
                }
            }
        }

        n
    }
}
