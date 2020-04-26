#[derive(Debug, PartialEq, Eq)]
enum Parity {
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

pub fn parity(x: u8) -> u8 {
    let mut p: u8 = x ^ x.checked_shr(1).unwrap_or(0);
    p ^= p.checked_shr(2).unwrap_or(0);
    p ^= p.checked_shr(4).unwrap_or(0);
    p ^= p.checked_shr(8).unwrap_or(0);
    if (p & 0x01) == 1 { 0 } else { 1 }
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