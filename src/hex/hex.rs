use std::ops::Add;

use crate::hex::eisenstein;

use super::eisenstein::EisensteinInteger;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Hex(EisensteinInteger);

impl Add for Hex {
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        Hex(self.0 + other.0)
    }
}

impl Add<&Hex> for Hex {
    type Output = Hex;

    fn add(self, other: &Hex) -> Hex {
        Hex(self.0 + other.0)
    }
}

impl Add<Hex> for &Hex {
    type Output = Hex;

    fn add(self, other: Hex) -> Hex {
        Hex(self.0 + other.0)
    }
}

impl Hex {

    pub fn from_eisenstein_integer(a: i32, b: i32) -> Hex {
        Hex(EisensteinInteger::new(a, b))
    }

    pub fn origin() -> Hex {
        Hex::from_eisenstein_integer(0, 0)
    }

    pub fn get_neighbor(&self, direction: HexDirection) -> Self {
        match direction {
            HexDirection::Above => self + Hex::from_eisenstein_integer(1, 2),
            HexDirection::Below => self + Hex::from_eisenstein_integer(-1, -2),
            HexDirection::LeftAbove => self + Hex::from_eisenstein_integer(-1, 1),
            HexDirection::LeftBelow => self + Hex::from_eisenstein_integer(-2, -1),
            HexDirection::RightAbove => self + Hex::from_eisenstein_integer(2, 1),
            HexDirection::RightBelow => self + Hex::from_eisenstein_integer(1, -1),
        }
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        self.0.get_coordinates()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HexDirection {
    Above,
    Below,
    LeftAbove,
    LeftBelow,
    RightAbove,
    RightBelow,
}


#[cfg(test)]
mod test_hex {
    use super::*;

    #[test]
    fn test_hexes_add() {
        
        let h1 = Hex::from_eisenstein_integer(1, 2);
        let h2 = Hex::from_eisenstein_integer(3, 4);

        assert_eq!(h1 + h2, Hex::from_eisenstein_integer(4, 6));
    }

    #[test]
    fn test_hex_gets_neighbors() {
        
        let center = Hex::origin();

        assert_eq!(center.get_neighbor(HexDirection::Above), Hex::from_eisenstein_integer(1, 2));
        assert_eq!(center.get_neighbor(HexDirection::Below), Hex::from_eisenstein_integer(-1, -2));
        assert_eq!(center.get_neighbor(HexDirection::LeftAbove), Hex::from_eisenstein_integer(-1, 1));
        assert_eq!(center.get_neighbor(HexDirection::LeftBelow), Hex::from_eisenstein_integer(-2, -1));
        assert_eq!(center.get_neighbor(HexDirection::RightAbove), Hex::from_eisenstein_integer(2, 1));
        assert_eq!(center.get_neighbor(HexDirection::RightBelow), Hex::from_eisenstein_integer(1, -1));
    }
}