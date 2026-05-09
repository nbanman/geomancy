use crate::dir::Dir;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PointStorage {
    One(Dir),
    Two(Dir),
    Three,
}
