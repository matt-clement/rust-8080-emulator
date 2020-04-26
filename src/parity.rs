#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Parity {
    Even,
    Odd,
}

use Parity::*;

impl From<u8> for Parity {
    fn from(value: u8) -> Self {
        match parity(value) & 0x01 {
            0 => Odd,
            1 => Even,
            _ => unreachable!(),
        }
    }
}

impl From<Parity> for u8 {
    fn from(value: Parity) -> Self {
        match value {
            Odd => 0,
            Even => 1,
        }
    }
}

const fn parity(x: u8) -> u8 {
    let mut p: u8 = x ^ x >> 1;
    p ^= p >> 2;
    p ^= p >> 4;
    (p & 0x01) ^ 0x01
}

mod test {
    #[allow(unused)] use super::*;

    #[test]
    fn test_parity() {
        assert_eq!(parity(0), 1);
        assert_eq!(parity(1), 0);
        assert_eq!(parity(2), 0);
        assert_eq!(parity(3), 1);
        assert_eq!(parity(4), 0);
        assert_eq!(parity(5), 1);
        assert_eq!(parity(8), 0);
        assert_eq!(parity(16), 0);
        assert_eq!(parity(127), 0);
        assert_eq!(parity(128), 0);
        assert_eq!(parity(129), 1);
        assert_eq!(parity(254), 0);
        assert_eq!(parity(255), 1);
    }

    #[test]
    fn test_from() {
        assert_eq!(Parity::from(0), Even);
        assert_eq!(Parity::from(1), Odd);
        assert_eq!(Parity::from(255), Even);
    }

    #[test]
    fn test_into() {
        assert_eq!(Into::<u8>::into(Parity::Odd), 0u8);
        assert_eq!(Into::<u8>::into(Parity::Even), 1u8);
    }
}