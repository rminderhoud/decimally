//! A 32-bit floating point decimal using IEEE-754 encoding
use crate::decimal::Decimal;
use crate::dpd::digits_from_dpd;
use crate::error::DecimalStorageError;

/// Lookup table for converting a 5-bit combination field to the 2 most significant bits of the
/// exponent
const COMB_EXP_LOOKUP: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 1, 1, 2, 2, 3, 3,
];

/// Lookup table for converting a 5-bit combination field to the most significand digit of the
/// coeffecient in BCD format (4-bits per digit)
const COMB_DIG_LOOKUP: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 9, 8, 9, 0, 1,
];

const SIGN_MASK: u32 = 0x8000_0000;
const COMBINATION_MASK: u32 = 0x7c00_0000;
const EXPONENT_MASK: u32 = 0x03f0_0000;
const COEFFECIENT_MASK: u32 = 0x000f_ffff;

const PRECISION: usize = 7;
const EXPONENT_BIAS: i8 = 101;

/// Minimum exponent value
pub const EXPONENT_MIN: i8 = -95;

/// Maximum exponent value
pub const EXPONENT_MAX: i8 = 96;

/// Maximum coeffecient (significand) value
pub const COEFFECIENT_MAX: u32 = 9_999_999; // 10 ^ PRECISION - 1

/// Zero decimal (0E1)
pub const ZERO: u32 = 0x6000_0000;

// Encodes an exponent's 2 most significant bits and a coeffecient's most significant digit in BCD
// (4-bit) into a 5-bit combination field
fn encode_combination_field(exp_msb: u8, coeff_msd: u8) -> u8 {
    let mut comb: u8 = 0;
    if coeff_msd <= 7 {
        comb |= (exp_msb << 3) | (coeff_msd & 0x7);
    } else {
        comb |= 0x18 | (exp_msb << 1) | (coeff_msd & 0x1);
    }
    comb
}

/// A 32-bit floating point decimal using IEEE-754 encoding
pub struct Decimal32 {
    pub bits: u32,
}

impl Decimal32 {
    /// Gets the 5-bit combination field
    #[inline]
    fn combination_field(&self) -> u8 {
        ((self.bits & COMBINATION_MASK) >> 26) as u8
    }

    /// Sets the 5-bit combination field
    #[inline]
    fn set_combination_field(&mut self, comb: u8) {
        self.bits &= !COMBINATION_MASK;
        self.bits |= (u32::from(comb)) << 26;
    }

    /// Gets the 2-bit exponent MSB from the combination field using a lookup table
    #[inline]
    fn exponent_msb(&self) -> u8 {
        COMB_EXP_LOOKUP[self.combination_field() as usize]
    }

    /// Gets the 6-bit exponent continuation
    #[inline]
    fn exponent_cont(&self) -> u8 {
        ((self.bits & EXPONENT_MASK) >> 20) as u8
    }

    /// Sets the 6-bit exponent continutation
    #[inline]
    fn set_exponent_cont(&mut self, cont: u8) {
        self.bits &= !EXPONENT_MASK;
        self.bits |= (u32::from(cont)) << 20;
    }

    /// Gets the 4-bit (BCD) coeffecient MSB from the combination field using a lookup table
    #[inline]
    fn coeffecient_msd(&self) -> u8 {
        COMB_DIG_LOOKUP[self.combination_field() as usize]
    }

    /// Gets the 20-bit (DPD encoded) coeffecient continuaton
    #[inline]
    fn coeffecient_cont(&self) -> u32 {
        self.bits & COEFFECIENT_MASK
    }

    /// Sets the 20-bit (DPD encoded) coeffecient continuation
    #[inline]
    fn set_coeffecient_cont(&mut self, cont: u32) {
        self.bits &= !COEFFECIENT_MASK;
        self.bits |= cont;
    }
}

impl Decimal for Decimal32 {
    type Coeffecient = u32;
    type Exponent = i8;

    fn new() -> Decimal32 {
        Decimal32 { bits: ZERO }
    }

    fn sign(&self) -> bool {
        (self.bits >> 31) > 0
    }

    fn set_sign(&mut self, sign: bool) {
        let sign: u32 = if sign { 1 } else { 0 };
        self.bits &= !SIGN_MASK;
        self.bits |= sign << 31;
    }

    fn exponent(&self) -> Self::Exponent {
        // Get exponent parts (2-bit msb & 6-bit continuation)
        let exp_msb = self.exponent_msb();
        let exp_cont = self.exponent_cont();

        // Encoded exponent as u8
        let encoded_exp = (exp_msb << 6) + (exp_cont as u8);

        // Adjust encoded exponent with bias
        // Note: Uses intermediate i16 to prevent u8 underflow
        let exp = i16::from(encoded_exp) - i16::from(EXPONENT_BIAS);

        exp as i8
    }

    fn set_exponent(&mut self, exp: Self::Exponent) -> Result<(), DecimalStorageError> {
        if exp > EXPONENT_MAX {
            return Err(DecimalStorageError::ExponentTooLarge);
        }

        if exp < EXPONENT_MIN {
            return Err(DecimalStorageError::ExponentTooSmall);
        }

        // Add the exponent bias
        // Note: Uses intermediate i16 to prevent u8 underflow
        let exp = (i16::from(exp) + i16::from(EXPONENT_BIAS)) as u8;

        // Set new exponent msb in combination field
        let exp_msb = exp >> 6;
        let coeff_msd = self.coeffecient_msd() as u8;
        let combination_field = encode_combination_field(exp_msb, coeff_msd);
        self.set_combination_field(combination_field);

        // Set new exponent continuation bits
        let exp_cont = exp & 0x6f;
        self.set_exponent_cont(exp_cont);

        Ok(())
    }

    fn coeffecient(&self) -> Self::Coeffecient {
        let coeff_msd = self.coeffecient_msd();
        let coeff_cont = self.coeffecient_cont();

        // Unpack coeffecient digits from DPD
        if coeff_msd > 0 {
            let coeff = (u32::from(coeff_msd) << 20) | coeff_cont;
            return digits_from_dpd(coeff, 3);
        }

        if coeff_cont == 0 {
            return 0;
        }

        if coeff_cont == 0x000f_fc00 {
            return digits_from_dpd(coeff_cont, 2);
        }

        digits_from_dpd(coeff_cont, 1)
    }

    fn set_coeffecient(&mut self, coeff: Self::Coeffecient) -> Result<(), DecimalStorageError> {
        if coeff > COEFFECIENT_MAX {
            return Err(DecimalStorageError::CoeffecientTooLarge);
        }

        // TODO:
        // - Encode coeffecient into dpd
        // - Get MSD + EXP MSB
        // - Set MSD into combo field
        // - Set coeffecient cont

        Ok(())
    }

    fn from_u8(num: u8) -> Self {
        let mut d = Self::new();
        d.set_coeffecient(u32::from(num)).unwrap();
        d
    }

    fn from_u16(num: u16) -> Self {
        let mut d = Self::new();
        d.set_coeffecient(u32::from(num)).unwrap();
        d
    }

    fn from_u32(num: u32) -> Self {
        let mut d = Self::new();
        // TODO: How to handle error, clamp or infinity
        // Spec seems to indicate that the number should be rounded based on user preference
        // So 4,294,967,295 would need to be truncated to 7 digits by rounding to 4,294,967,000
        // using the specified rounding system and then representing with different exponent
        // Question that is raised: Should this be handled implicity or provided to function
        // by function just like every operation?
        // Answer, should use context precision UNLESS it's greater than implementation precision,
        // then use implementation precision

        d.set_coeffecient(u32::from(num)).unwrap();
        d
    }

    fn from_u8_checked(num: u8) -> Option<Self> {
        Some(Self::from_u8(num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal32_sign() {
        let pos = Decimal32 { bits: 0x0000_0000 };
        assert_eq!(pos.sign(), false);
        assert_eq!(pos.is_sign_positive(), true);
        assert_eq!(pos.is_sign_negative(), false);

        let neg = Decimal32 { bits: 0x8000_0000 };
        assert_eq!(neg.sign(), true);
        assert_eq!(neg.is_sign_positive(), false);
        assert_eq!(neg.is_sign_negative(), true);

        for dec in &mut [pos, neg] {
            for signed in &[false, true, false, true] {
                let signed = *signed;
                dec.set_sign(signed);
                assert_eq!(dec.sign(), signed);
                assert_eq!(dec.is_sign_positive(), !signed);
                assert_eq!(dec.is_sign_negative(), signed);
            }
        }
    }

    #[test]
    fn test_decimal32_exponent() {
        let pos_exp = Decimal32 { bits: 0xA260_03D0 };
        let neg_exp = Decimal32 { bits: 0xA230_03D0 };

        assert_eq!(pos_exp.exponent(), 1);
        assert_eq!(neg_exp.exponent(), -2);

        let mut dec = Decimal32::new();
        assert_eq!(dec.set_exponent(EXPONENT_MIN - 1).is_err(), true);
        assert_eq!(dec.set_exponent(EXPONENT_MAX + 1).is_err(), true);

        for exp in &[EXPONENT_MIN, -5, 0, 5, EXPONENT_MAX] {
            let exp = *exp;
            dec.set_exponent(exp).unwrap();
            assert_eq!(exp, dec.exponent());
        }
    }

    #[test]
    fn test_decimal32_coeffecient() {
        let mut dec = Decimal32 { bits: 0xA260_03D0 };
        assert_eq!(dec.coeffecient(), 750);

        assert_eq!(dec.set_coeffecient(COEFFECIENT_MAX + 1).is_err(), true);

        for coeff in &[0, 5, 999, 99999, COEFFECIENT_MAX] {
            let coeff = *coeff;
            dec.set_coeffecient(coeff).unwrap();
            assert_eq!(coeff, dec.coeffecient());
        }
    }
}
