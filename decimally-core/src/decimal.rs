use crate::error::DecimalStorageError;
use crate::int::{SignedInteger, UnsignedInteger};

pub trait Decimal: Sized {
    type Exponent: SignedInteger;
    type Coeffecient: UnsignedInteger;

    /// Create default empty decimal
    fn new() -> Self;

    /// Get the sign of the decimal. True indicates a negative sign
    fn sign(&self) -> bool;

    /// Set the sign of the decimal. True indicates a negative sign
    fn set_sign(&mut self, sign: bool);

    /// Get the decimal exponent
    fn exponent(&self) -> Self::Exponent;

    /// Set the decimal exponent
    fn set_exponent(&mut self, exp: Self::Exponent) -> Result<(), DecimalStorageError>;

    /// Get the decimal coeffecient (significand)
    fn coeffecient(&self) -> Self::Coeffecient;

    /// Set the decimal coeffecient (signficand)
    fn set_coeffecient(&mut self, coeff: Self::Coeffecient) -> Result<(), DecimalStorageError>;

    /*
    fn to_scientific_string() {}
    fn to_engineering_string() {}
    fn to_string() {}
    fn from_string() {}
    */
    fn is_sign_positive(&self) -> bool {
        !self.sign()
    }

    fn is_sign_negative(&self) -> bool {
        !self.is_sign_positive()
    }

    fn set_sign_positive(&mut self) {
        self.set_sign(false);
    }

    fn set_sign_negative(&mut self) {
        self.set_sign(true);
    }

    // ----------------------------------------------------
    // Primitive Type Conversions
    // ----------------------------------------------------

    /// Create decimal from `u8` with potential precision loss
    fn from_u8(num: u8) -> Self;

    /// Create decimal from `u16` with potential precision loss
    fn from_u16(num: u16) -> Self;

    /// Create decimal from `u32` with potential precision loss
    fn from_u32(num: u32) -> Self;

    /*
        /// Create decimal from `u64` with potential precision loss
        fn from_u64(num: u64) -> Self;

        /// Create decimal from `u128` with potential precision loss
        fn from_u128(num: u128) -> Self;

        /// Create decimal from `i8` with potential precision loss
        fn from_i8(num: i8) -> Self;

        /// Create decimal from `i16` with potential precision loss
        fn from_i16(num: i16) -> Self;

        /// Create decimal from `i32` with potential precision loss
        fn from_i32(num: i32) -> Self;

        /// Create decimal from `i64` with potential precision loss
        fn from_i64(num: i64) -> Self;

        /// Create decimal from `i128` with potential precision loss
        fn from_i128(num: i128) -> Self;
    */
    /// Create decimal from `u8` only if possible without precision loss
    fn from_u8_checked(i: u8) -> Option<Self> {
        None
    }
}
