mod dir;
mod point;
pub use dir::Dir;
pub use point::Point;
pub use point::Length;

use num::{One, Zero};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Map<T: point::Length = i32> {
    data: Vec<u8>,
    width: T,
    height: T,
    has_border: bool,
}

pub struct MapIterator<'a, T: point::Length> {
    map: &'a Map<T>,
    pos: Point<T>,
}

impl<'a, T: point::Length> MapIterator<'a, T> {
    pub fn new(map: &'a Map<T>) -> Self {
        Self {
            map,
            pos: Point {
                x: Zero::zero(),
                y: Zero::zero(),
            },
        }
    }
}

impl<'a, T: point::Length> Iterator for MapIterator<'a, T> {
    type Item = (Point<T>, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x >= self.map.get_width() {
            self.pos.x = Zero::zero();
            self.pos.y += One::one();
        }
        if self.pos.y >= self.map.get_height() {
            None
        } else {
            let pos = self.pos;
            self.pos.x += One::one();
            Some((pos, self.map.get_at(pos)))
        }
    }
}

pub struct MapNeighborIterator<'a, T: point::Length> {
    map: &'a Map<T>,
    pos: Point<T>,
    dir: Dir,
}

impl<'a, T: point::Length> MapNeighborIterator<'a, T> {
    pub fn new(map: &'a Map<T>, pos: Point<T>) -> Self {
        Self {
            map,
            pos,
            dir: Dir::North,
        }
    }
}

impl<'a, T: point::Length> Iterator for MapNeighborIterator<'a, T> {
    type Item = (Point<T>, Dir, u8);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.dir == Dir::None {
                return None;
            } else {
                let dir = self.dir;
                self.dir = self.dir.turn_right();
                if self.dir == Dir::North {
                    self.dir = Dir::None;
                }

                let pos = self.pos.walk(dir);
                if self.map.is_inside_map(pos) {
                    return Some((pos, dir, self.map.get_at(pos)));
                }
            }
        }
    }
}

impl<T: point::Length> Map<T> {
    pub fn get_width(&self) -> T {
        self.width
    }

    pub fn get_height(&self) -> T {
        self.height
    }

    fn get_index_for(&self, pos: Point<T>) -> usize {
        usize::try_from(pos.x + pos.y * self.width).expect("Positive index")
    }

    pub fn get_at(&self, pos: Point<T>) -> u8 {
        self.data[self.get_index_for(pos)]
    }

    pub fn set_at(&mut self, pos: Point<T>, val: u8) {
        let index = self.get_index_for(pos);
        self.data[index] = val
    }

    pub fn new(width: T, height: T) -> Self {
        let mut data =
            Vec::with_capacity(usize::try_from(width * height).expect("Positive number"));
        data.resize_with(
            usize::try_from(width * height).expect("Positive size"),
            || b'.',
        );
        Self {
            data,
            width,
            height,
            has_border: false,
        }
    }

    pub fn from_string(s: &str) -> Self {
        let height = s.lines().count();
        let width = s.lines().next().expect("At least one line").len();
        let mut data = Vec::with_capacity(height * width);
        for c in s.chars() {
            if c != '\n' {
                data.push(u8::try_from(c).expect("Ascii char"));
            }
        }

        let width = T::try_from(width).expect("Positive width");
        let height = T::try_from(height).expect("Positive height");
        Self {
            data,
            width,
            height,
            has_border: false,
        }
    }

    pub fn from_string_with_border(s: &str) -> Self {
        let height = s.lines().count() + 2;
        let width = s.lines().next().expect("At least one line").len() + 2;
        let mut data = Vec::with_capacity(height * width);
        data.push(b'+');
        for _x in 0..width - 2 {
            data.push(b'-');
        }
        data.push(b'+');
        data.push(b'|');
        for c in s.chars() {
            if c != '\n' {
                data.push(u8::try_from(c).expect("Ascii char"));
            } else {
                data.push(b'|');
                data.push(b'|');
            }
        }
        data.push(b'|');
        data.push(b'+');
        for _x in 0..width - 2 {
            data.push(b'-');
        }
        data.push(b'+');

        let width = T::try_from(width).expect("Positive width");
        let height = T::try_from(height).expect("Positive height");
        Self {
            data,
            width,
            height,
            has_border: true,
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", char::from(self.get_at(Point { x, y })));
            }
            println!();
        }
    }

    pub fn iter(&self) -> MapIterator<T> {
        MapIterator::new(self)
    }

    pub fn neighbors(&self, pos: Point<T>) -> MapNeighborIterator<T> {
        MapNeighborIterator::new(self, pos)
    }

    pub fn transform_area<F>(&mut self, from: Point<T>, to: Point<T>, mut f: F)
    where
        F: FnMut(&Self, Point<T>, u8) -> u8,
    {
        let mut new_map = Map::new(self.width, self.height);
        for (pos, c) in self.iter() {
            if pos.x >= from.x && pos.y >= from.y && pos.x < to.x && pos.y < to.y {
                new_map.set_at(pos, f(self, pos, c));
            }
        }
        for (pos, c) in new_map.iter() {
            if pos.x >= from.x && pos.y >= from.y && pos.x < to.x && pos.y < to.y {
                self.set_at(pos, c);
            }
        }
    }

    pub fn transform<F>(&mut self, f: F)
    where
        F: FnMut(&Self, Point<T>, u8) -> u8,
    {
        if self.has_border {
            self.transform_area(
                Point { x: 1, y: 1 },
                Point {
                    x: self.width - 1,
                    y: self.height - 1,
                },
                f,
            )
        } else {
            self.transform_area(
                Point { x: 0, y: 0 },
                Point {
                    x: self.width,
                    y: self.height,
                },
                f,
            )
        }
    }

    pub fn is_inside_map(&self, pos: Point<T>) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.get_width() && pos.y < self.get_height()
    }

    /// moves pos in dir
    ///
    /// It stops when the next point in that direction is outside of the map or causes f to return
    /// false
    pub fn walk_until<F>(&self, pos: Point<T>, dir: Dir, mut f: F) -> Point<T>
    where
        F: FnMut(u8) -> bool,
    {
        let mut pos = pos;
        loop {
            let new_pos = pos.walk(dir);
            if !self.is_inside_map(new_pos) || f(self.get_at(new_pos)) {
                break;
            }
            pos = new_pos;
        }
        pos
    }

    /// flood fill the map from point pos with val
    /// Only fills north, south, east and west of each position
    pub fn flood_cardinal(&mut self, pos: Point<T>, empty: u8, val: u8) {
        if self.get_at(pos) != empty {
            // Nothing to fill here
            return;
        }
        let min_pos = self.walk_until(pos, Dir::West, |c| c != empty);
        let max_pos = self.walk_until(pos, Dir::East, |c| c != empty);

        let mut pos = min_pos;
        while pos.x <= max_pos.x {
            self.set_at(pos, val);
            pos = pos.walk(Dir::East);
        }
        pos = min_pos;
        if pos.y > 0 {
            while pos.x <= max_pos.x {
                pos.y -= 1;
                self.flood_cardinal(pos, empty, val);
                pos.y += 2;
                self.flood_cardinal(pos, empty, val);
                pos.y -= 1;
                pos = pos.walk(Dir::East);
            }
        }
    }
}
