// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

type Length = i32;

mod dir;
mod point;
pub use dir::Dir;
pub use point::Point;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Map {
    data: Vec<u8>,
    width: Length,
    height: Length,
    has_border: bool,
}

pub struct MapIterator<'a> {
    map: &'a Map,
    pos: Point,
}

impl<'a> MapIterator<'a> {
    pub fn new(map: &'a Map) -> Self {
        Self {
            map,
            pos: Point { x: 0, y: 0 },
        }
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (Point, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x >= self.map.get_width() {
            self.pos.x = 0;
            self.pos.y += 1;
        }
        if self.pos.y >= self.map.get_height() {
            None
        } else {
            let pos = self.pos;
            self.pos.x += 1;
            Some((pos, self.map.get_at(pos)))
        }
    }
}

pub struct MapNeighborIterator<'a> {
    map: &'a Map,
    pos: Point,
    dir: Dir,
}

impl<'a> MapNeighborIterator<'a> {
    pub fn new(map: &'a Map, pos: Point) -> Self {
        Self {
            map,
            pos,
            dir: Dir::North,
        }
    }
}

impl<'a> Iterator for MapNeighborIterator<'a> {
    type Item = (Point, Dir, u8);

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

impl Map {
    pub fn get_width(&self) -> Length {
        self.width
    }

    pub fn get_height(&self) -> Length {
        self.height
    }

    fn get_index_for(&self, pos: Point) -> usize {
        usize::try_from(pos.x + pos.y * self.width).expect("Positive index")
    }

    pub fn get_at(&self, pos: Point) -> u8 {
        self.data[self.get_index_for(pos)]
    }

    pub fn set_at(&mut self, pos: Point, val: u8) {
        let index = self.get_index_for(pos);
        self.data[index] = val
    }

    pub fn new(width: Length, height: Length) -> Self {
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

        let width = Length::try_from(width).expect("Positive width");
        let height = Length::try_from(height).expect("Positive height");
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

        let width = Length::try_from(width).expect("Positive width");
        let height = Length::try_from(height).expect("Positive height");
        Self {
            data,
            width,
            height,
            has_border: true,
        }
    }

    pub fn print_with_overlay<F>(&self, mut f: F)
    where
        F: FnMut(Point, u8) -> Option<u8>,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Point { x, y };
                let mut c = self.get_at(pos);
                if let Some(new_c) = f(pos, c) {
                    c = new_c;
                }
                print!("{}", char::from(c));
            }
            println!();
        }
    }

    pub fn print(&self) {
        self.print_with_overlay(|_, _| None);
    }

    pub fn iter(&self) -> MapIterator {
        MapIterator::new(self)
    }

    pub fn neighbors(&self, pos: Point) -> MapNeighborIterator {
        MapNeighborIterator::new(self, pos)
    }

    pub fn transform_area<F>(&mut self, from: Point, to: Point, mut f: F)
    where
        F: FnMut(&Self, Point, u8) -> u8,
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
        F: FnMut(&Self, Point, u8) -> u8,
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

    pub fn is_inside_map(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.get_width() && pos.y < self.get_height()
    }

    /// moves pos in dir
    ///
    /// It stops when the next point in that direction is outside of the map or causes f to return
    /// false
    pub fn walk_until<F>(&self, pos: Point, dir: Dir, mut f: F) -> Point
    where
        F: FnMut(Point, u8) -> bool,
    {
        let mut pos = pos;
        loop {
            let new_pos = pos.walk(dir);
            if !self.is_inside_map(new_pos) || f(new_pos, self.get_at(new_pos)) {
                break;
            }
            pos = new_pos;
        }
        pos
    }

    /// flood fill the map from point pos with val
    /// Only fills north, south, east and west of each position
    pub fn flood_cardinal(&mut self, pos: Point, empty: u8, val: u8) {
        if self.get_at(pos) != empty {
            // Nothing to fill here
            return;
        }
        let min_pos = self.walk_until(pos, Dir::West, |_, c| c != empty);
        let max_pos = self.walk_until(pos, Dir::East, |_, c| c != empty);

        let mut pos = min_pos;
        while pos.x <= max_pos.x {
            self.set_at(pos, val);
            pos = pos.walk(Dir::East);
        }
        pos = min_pos;
        while pos.x <= max_pos.x {
            pos.y -= 1;
            if pos.y > 0 {
                self.flood_cardinal(pos, empty, val);
            }
            pos.y += 2;
            if pos.y <= self.get_height() - 1 {
                self.flood_cardinal(pos, empty, val);
            }
            pos.y -= 1;
            pos = pos.walk(Dir::East);
        }
    }

    pub fn flood_cardinal_with<F>(&mut self, pos: Point, f: &mut F)
    where
        F: FnMut(Point, u8) -> Option<u8>,
    {
        if f(pos, self.get_at(pos)) == None {
            // Nothing to fill here
            return;
        }
        let min_pos = self.walk_until(pos, Dir::West, |pos, c| f(pos, c) == None);
        let max_pos = self.walk_until(pos, Dir::East, |pos, c| f(pos, c) == None);

        let mut pos = min_pos;
        while pos.x <= max_pos.x {
            let val = f(pos, self.get_at(pos)).expect("value");
            self.set_at(pos, val);
            pos = pos.walk(Dir::East);
        }
        pos = min_pos;
        while pos.x <= max_pos.x {
            pos.y -= 1;
            if pos.y >= 0 {
                self.flood_cardinal_with(pos, f);
            }
            pos.y += 2;
            if pos.y <= self.get_height() - 1 {
                self.flood_cardinal_with(pos, f);
            }
            pos.y -= 1;
            pos = pos.walk(Dir::East);
        }
    }
}
