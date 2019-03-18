use crate::int::{SignedInteger, UnsignedInteger};

pub trait Decimal {
    type Exponent: SignedInteger;
    type Coeffecient: UnsignedInteger;

    fn sign(&self) -> bool;
    fn exponent(&self) -> Self::Exponent;
    fn coeffecient(&self) -> Self::Coeffecient;

    fn is_negative(&self) -> bool {
        self.sign()
    }

    fn is_positive(&self) -> bool {
        !self.sign()
    }
}
