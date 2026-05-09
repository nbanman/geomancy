use enum_ordinalize::Ordinalize;

#[derive(Clone, Copy, PartialEq, Eq, Ordinalize)]
pub enum Dir {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Dir {
    pub fn left(&self) -> Self {
        let new_ord = self.ordinal() - 1;
        if new_ord == -1 {
            Self::NorthWest
        } else {
            Self::from_ordinal(new_ord).unwrap()
        }
    }

    pub fn right(&self) -> Self {
        let new_ord = self.ordinal() + 1;
        if new_ord == 8 {
            Self::North
        } else {
            Self::from_ordinal(new_ord).unwrap()
        }
    }
}
