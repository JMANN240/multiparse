use std::ops::Deref;

use num_traits::{Num, PrimInt};

pub trait Multiparse {
    fn multiparse<T: PrimInt>(&self) -> Result<T, <T as Num>::FromStrRadixErr>;
}

impl<D: Deref<Target = str>> Multiparse for D {
    fn multiparse<T: PrimInt>(&self) -> Result<T, <T as Num>::FromStrRadixErr> {
        let value = self.trim().to_ascii_lowercase();

        if let Some(binary_string) = value.strip_prefix("0b") {
            T::from_str_radix(binary_string, 2)
        } else if let Some(octal_string) = value.strip_prefix("0o") {
            T::from_str_radix(octal_string, 8)
        } else if let Some(hex_string) = value.strip_prefix("0x") {
            T::from_str_radix(hex_string, 16)
        } else {
            T::from_str_radix(&value, 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_binary() {
        assert!("0b0".multiparse::<u8>().is_ok_and(|value| value == 0));
        assert!("0b1".multiparse::<u8>().is_ok_and(|value| value == 1));
        assert!("0b10".multiparse::<u8>().is_ok_and(|value| value == 2));
        assert!("0b11".multiparse::<u8>().is_ok_and(|value| value == 3));
        assert!("0b11111111".multiparse::<u8>().is_ok_and(|value| value == 255));
    }

    #[test]
    fn test_parse_octal() {
        assert!("0o0".multiparse::<u8>().is_ok_and(|value| value == 0));
        assert!("0o1".multiparse::<u8>().is_ok_and(|value| value == 1));
        assert!("0o2".multiparse::<u8>().is_ok_and(|value| value == 2));
        assert!("0o10".multiparse::<u8>().is_ok_and(|value| value == 8));
        assert!("0o11".multiparse::<u8>().is_ok_and(|value| value == 9));
        assert!("0o100".multiparse::<u8>().is_ok_and(|value| value == 64));
        assert!("0o101".multiparse::<u8>().is_ok_and(|value| value == 65));
        assert!("0o107".multiparse::<u8>().is_ok_and(|value| value == 71));
    }

    #[test]
    fn test_parse_decimal() {
        assert!("0".multiparse::<u8>().is_ok_and(|value| value == 0));
        assert!("1".multiparse::<u8>().is_ok_and(|value| value == 1));
        assert!("10".multiparse::<u8>().is_ok_and(|value| value == 10));
        assert!("100".multiparse::<u8>().is_ok_and(|value| value == 100));
        assert!("-1".multiparse::<i8>().is_ok_and(|value| value == -1));
        assert!("-10".multiparse::<i8>().is_ok_and(|value| value == -10));
        assert!("-100".multiparse::<i16>().is_ok_and(|value| value == -100));
    }

    #[test]
    fn test_parse_hexadecimal() {
        assert!("0x0".multiparse::<u8>().is_ok_and(|value| value == 0));
        assert!("0x1".multiparse::<u8>().is_ok_and(|value| value == 1));
        assert!("0xA".multiparse::<u8>().is_ok_and(|value| value == 10));
        assert!("0xF".multiparse::<u8>().is_ok_and(|value| value == 15));
        assert!("0x10".multiparse::<u8>().is_ok_and(|value| value == 16));
        assert!("0x11".multiparse::<u8>().is_ok_and(|value| value == 17));
        assert!("0xFF".multiparse::<u8>().is_ok_and(|value| value == 255));
        assert!("0x100".multiparse::<u16>().is_ok_and(|value| value == 256));
        assert!("0x10F".multiparse::<u16>().is_ok_and(|value| value == 271));
        assert!("0x110".multiparse::<u16>().is_ok_and(|value| value == 272));
    }
}
