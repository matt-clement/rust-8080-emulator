#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

use Sign::*;

impl Sign {
    pub fn get_sign(value: u8) -> Self {
        match value & 0x80 {
            0x00 => Positive,
            0x80 => Negative,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Sign {
    fn from(value: u8) -> Self {
        match value & 0x1 {
            0x0 => Positive,
            0x1 => Negative,
            _ => unreachable!(),
        }
    }
}

impl From<Sign> for u8 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 0,
            Negative => 1,
        }
    }
}

mod test {
    #[allow(unused)] use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Sign::from(0), Positive);
        assert_eq!(Sign::from(1), Negative);
    }

    #[test]
    fn test_into() {
        assert_eq!(Into::<u8>::into(Positive), 0u8);
        assert_eq!(Into::<u8>::into(Negative), 1u8);
    }
}