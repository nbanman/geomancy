use crate::dir::Dir;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn go(&self, dir: Dir) -> Self {
        match dir {
            Dir::North => Point {
                y: self.y - 1,
                ..*self
            },
            Dir::NorthEast => Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Dir::East => Point {
                x: self.x + 1,
                ..*self
            },
            Dir::SouthEast => Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Dir::South => Point {
                y: self.y + 1,
                ..*self
            },
            Dir::SouthWest => Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Dir::West => Point {
                x: self.x - 1,
                ..*self
            },
            Dir::NorthWest => Point {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }
}
