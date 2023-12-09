use super::dir::Dir;
use num::{One, PrimInt, Signed};

pub trait Length: Signed + PrimInt + Sized + TryFrom<usize> + std::ops::AddAssign {}

impl Length for i16 {}
impl Length for i32 {}
impl Length for i64 {}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Point<T: Length> {
    pub x: T,
    pub y: T,
}

impl<T: Length> Point<T> {
    pub fn walk(self, dir: Dir) -> Self {
        match dir {
            Dir::None => Self {
                x: self.x,
                y: self.y,
            },
            Dir::North => Self {
                x: self.x,
                y: self.y - One::one(),
            },
            Dir::South => Self {
                x: self.x,
                y: self.y + One::one(),
            },
            Dir::East => Self {
                x: self.x + One::one(),
                y: self.y,
            },
            Dir::West => Self {
                x: self.x - One::one(),
                y: self.y,
            },
            Dir::NorthEast => Self {
                x: self.x + One::one(),
                y: self.y - One::one(),
            },
            Dir::NorthWest => Self {
                x: self.x - One::one(),
                y: self.y - One::one(),
            },
            Dir::SouthEast => Self {
                x: self.x + One::one(),
                y: self.y + One::one(),
            },
            Dir::SouthWest => Self {
                x: self.x - One::one(),
                y: self.y + One::one(),
            },
        }
    }
}

impl<T: Length> std::ops::Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
