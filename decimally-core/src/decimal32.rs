use crate::decimal::Decimal;
use crate::dpd::digits_from_dpd;

/// Lookup table for converting a 5-bit combination field to the 2 most significant bits of the
/// exponent
const COMB_EXP_LOOKUP: [u16; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 1, 1, 2, 2, 3, 3,
];

/// Lookup table for converting a 5-bit combination field to the most significand digit of the
/// coeffecient in BCD format (4-bits per digit)
const COMB_DIG_LOOKUP: [u32; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 9, 8, 9, 0, 1,
];

const COMBINATION_MASK: u32 = 0x1f;
const EXPONENT_MASK: u32 = 0x3f;
const COEFFECIENT_MASK: u32 = 0x000fffff;

const BIAS: i16 = 101;

/// Decimal encoded as a 32-bit unsigned integer
/// 1 bit sign
/// 5 bit combination field
/// 6 bit exponent continuation field
/// 20 bit coeffecient continuation field
struct Decimal32 {
    bytes: u32,
}

impl Decimal for Decimal32 {
    type Coeffecient = u32;
    type Exponent = i16;

    fn sign(&self) -> bool {
        (self.bytes >> 31) == 1
    }

    fn exponent(&self) -> Self::Exponent {
        // 5-bit combination field
        let combination_field = (self.bytes >> 26) & COMBINATION_MASK;

        // 2-bit exponent MSB from the combination field using lookup table
        let exp_msb = COMB_EXP_LOOKUP[combination_field as usize];

        // 6-bit exponent continuation
        let exp_cont = (self.bytes >> 20) & EXPONENT_MASK;

        // Exponent
        ((exp_msb << 6) + (exp_cont as u16)) as i16 - BIAS
    }

    fn coeffecient(&self) -> Self::Coeffecient {
        // 5-bit combination field
        let combination_field = (self.bytes >> 26) & COMBINATION_MASK;

        // 3-bit MSD (digit) from the combination field using lookup table
        let coeff_msd = COMB_DIG_LOOKUP[combination_field as usize];

        // 20-bit coeffecient continuation
        let coeff_cont = self.bytes & COEFFECIENT_MASK;

        // Unpack coeffecient digits from DPD
        if coeff_msd > 0 {
            let coeff = (coeff_msd << 20) | coeff_cont;
            return digits_from_dpd(coeff, 3);
        }

        if coeff_cont == 0 {
            return 0;
        }

        if coeff_cont == 0x000ffc00 {
            return digits_from_dpd(coeff_cont, 2);
        }

        return digits_from_dpd(coeff_cont, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal32_sign() {
        let pos = Decimal32 { bytes: 0x8000_0000 };
        let neg = Decimal32 { bytes: 0x0000_0000 };

        assert_eq!(pos.sign(), true);
        assert_eq!(neg.sign(), false);
    }

    #[test]
    fn test_decimal32_exponent() {
        let pos_exp = Decimal32 { bytes: 0xA260_03D0 };
        let neg_exp = Decimal32 { bytes: 0xA230_03D0 };

        assert_eq!(pos_exp.exponent(), 1);
        assert_eq!(neg_exp.exponent(), -2);
    }

    #[test]
    fn test_decimal32_coeffecient() {
        let d = Decimal32 { bytes: 0xA260_03D0 };
        assert_eq!(d.coeffecient(), 750);
    }
}
