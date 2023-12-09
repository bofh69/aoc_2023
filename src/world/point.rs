type Length = i32;

use super::dir::Dir;

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Point<T = Length> {
    pub x: T,
    pub y: T,
}

impl Point {
    pub fn walk(self, dir: Dir) -> Self {
        match dir {
            Dir::None => Self {
                x: self.x,
                y: self.y,
            },
            Dir::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Dir::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Dir::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Dir::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Dir::NorthEast => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Dir::NorthWest => Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Dir::SouthEast => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Dir::SouthWest => Self {
                x: self.x - 1,
                y: self.y + 1,
            },
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
