use parry2d::na::Vector2;
use rand::distributions::{Distribution, WeightedIndex};

use std::collections::HashMap;
use std::f32::consts::FRAC_1_SQRT_2;

#[derive(Eq, PartialEq)]
pub enum TileDirection {
    Constant(Direction),
    Intersection(HashMap<Direction, Vec<Cardinal>>),
}

impl TileDirection {
    pub fn as_dir(&self, cur_dir: Direction) -> Direction {
        match self {
            Self::Constant(dir) => *dir,
            Self::Intersection(dirs) => {
                let possible = &dirs[&cur_dir];

                let out_dir = cur_dir.out_dir();

                let weights =
                    possible
                        .iter()
                        .map(|&dir| if dir == out_dir { 3 } else { 1 })
                        .collect::<Vec<_>>();

                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = rand::thread_rng();

                let new_dir = possible[dist.sample(&mut rng)];

                if new_dir == out_dir {
                    Direction::Straight(new_dir)
                } else {
                    Direction::Turn(out_dir, new_dir)
                }
            }
        }
    }

    pub fn degrees(&self) -> f32 {
        match self {
            Self::Constant(dir) => dir.degrees(),
            Self::Intersection(_) => 0.0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Straight(Cardinal),
    Turn(Cardinal, Cardinal),
}

impl Direction {
    pub fn in_dir(&self) -> Cardinal {
        match self {
            Self::Straight(dir) => *dir,
            Self::Turn(dir, _)  => *dir,
        }
    }

    pub fn out_dir(&self) -> Cardinal {
        match self {
            Self::Straight(dir) => *dir,
            Self::Turn(_, dir)  => *dir,
        }
    }

    pub fn vector(&self) -> Vector2<f32> {
        match self {
            Self::Straight(dir) => dir.vector(),
            Self::Turn(in_dir, out_dir) => {
                (in_dir.vector() + out_dir.vector()) * FRAC_1_SQRT_2
            }
        }
    }

    pub fn degrees(&self) -> f32 {
        match self {
            Self::Straight(dir) => dir.degrees(None),
            Self::Turn(in_dir, out_dir) => {
                (in_dir.degrees(Some(out_dir)) + out_dir.degrees(Some(in_dir))) / 2.0
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Cardinal {
    Up,
    Down,
    Left,
    Right,
}

impl Cardinal {
    pub fn rotate(&mut self) -> Self {
        match self {
            Self::Up    => Self::Right,
            Self::Right => Self::Down,
            Self::Down  => Self::Left,
            Self::Left  => Self::Up,
        }
    }

    pub fn vector(&self) -> Vector2<f32> {
        match self {
            Self::Up    => Vector2::new( 0.0, -1.0),
            Self::Down  => Vector2::new( 0.0,  1.0),
            Self::Left  => Vector2::new(-1.0,  0.0),
            Self::Right => Vector2::new( 1.0,  0.0),
        }
    }

    pub fn degrees(&self, other: Option<&Cardinal>) -> f32 {
        match self {
            Self::Up    => if other.is_some_and(|&dir| dir == Cardinal::Left) { 360.0 } else { 0.0 },
            Self::Down  => 180.0,
            Self::Left  => 270.0,
            Self::Right => 90.0,
        }
    }

    pub fn on_axis(&self, axis: &Axis) -> bool {
        let right_axis =
            match self {
                Self::Up    => Axis::Vertical,
                Self::Down  => Axis::Vertical,
                Self::Left  => Axis::Horizontal,
                Self::Right => Axis::Horizontal,
            };

        axis == &right_axis
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn flip(&self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical   => Self::Horizontal,
        }
    }

    pub fn degrees(&self) -> f32 {
        match self {
            Self::Horizontal => 0.0,
            Self::Vertical   => 90.0,
        }
    }
}
