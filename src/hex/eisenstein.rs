use std::ops::{Add, Mul};


#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct EisensteinInteger {
    a: i32,
    b: i32,
}

impl Add for EisensteinInteger {
    type Output = EisensteinInteger;

    fn add(self, other: EisensteinInteger) -> EisensteinInteger {
        EisensteinInteger {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl Add<i32> for EisensteinInteger {
    type Output = EisensteinInteger;

    fn add(self, other: i32) -> EisensteinInteger {
        EisensteinInteger {
            a: self.a + other,
            b: self.b,
        }
    }
}

impl Add<EisensteinInteger> for i32 {
    type Output = EisensteinInteger;

    fn add(self, other: EisensteinInteger) -> EisensteinInteger {
        EisensteinInteger {
            a: self + other.a,
            b: other.b,
        }
    }
}

impl Mul for EisensteinInteger {
    type Output = EisensteinInteger;

    fn mul(self, other: EisensteinInteger) -> EisensteinInteger {
        let a = self.a * other.a - self.b * other.b;
        let b = self.a * other.b + self.b * other.a - self.b * other.b;
        EisensteinInteger { a, b }
    }
}

impl Mul<i32> for EisensteinInteger {
    type Output = EisensteinInteger;

    fn mul(self, other: i32) -> EisensteinInteger {
        EisensteinInteger {
            a: self.a * other,
            b: self.b * other,
        }
    }
}

impl Mul<EisensteinInteger> for i32 {
    type Output = EisensteinInteger;

    fn mul(self, other: EisensteinInteger) -> EisensteinInteger {
        EisensteinInteger {
            a: self * other.a,
            b: self * other.b,
        }
    }
}

impl EisensteinInteger {

    pub fn new(a: i32, b: i32) -> EisensteinInteger {
        EisensteinInteger { a, b }
    }

    pub fn zero() -> EisensteinInteger {
        EisensteinInteger { a: 0, b: 0 }
    }

    pub fn one() -> EisensteinInteger {
        EisensteinInteger { a: 1, b: 0 }
    }

    pub fn omega() -> EisensteinInteger {
        EisensteinInteger { a: 0, b: 1 }
    }

}

#[cfg(test)]
mod test_eisenstein {
    use super::*;

    #[test]
    fn test_eisenstein_integers_add() {
        
        let x = EisensteinInteger::new(1, 2);
        let y = EisensteinInteger::new(3, 4);
        assert_eq!(x + y, EisensteinInteger::new(4, 6));

        let x = EisensteinInteger::new(-1, -2);
        let y = EisensteinInteger::new(-3, -4);
        assert_eq!(x + y, EisensteinInteger::new(-4, -6));
        
    }

    #[test]
    fn test_eisenstein_integers_multiply() {

        assert_eq!(EisensteinInteger::omega() * EisensteinInteger::omega(), EisensteinInteger::new(-1, -1));
        
        let x = EisensteinInteger::new(1, 2);

        assert_eq!(x * EisensteinInteger::zero(), EisensteinInteger::zero());
        assert_eq!(x * EisensteinInteger::one(), x);

        let y = EisensteinInteger::new(3, 4);

        let expected_a = 1 * 3 - 2 * 4;
        let expected_b = 1 * 4 + 2 * 3 - 2 * 4;
        let expected = EisensteinInteger::new(expected_a, expected_b);
        assert_eq!(x * y, expected);

    }

}