// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Dir {
    None,
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Dir {
    pub fn turn_right(self) -> Self {
        use Dir::*;
        match self {
            None => self,
            North => NorthEast,
            South => SouthWest,
            East => SouthEast,
            West => NorthWest,
            NorthEast => East,
            NorthWest => North,
            SouthEast => South,
            SouthWest => West,
        }
    }

    pub fn turn_left(self) -> Self {
        use Dir::*;
        match self {
            None => self,
            North => NorthWest,
            South => SouthEast,
            East => NorthEast,
            West => SouthWest,
            NorthEast => North,
            NorthWest => West,
            SouthEast => East,
            SouthWest => South,
        }
    }
}
