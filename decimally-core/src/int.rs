use core::ops::{Add, Div, Mul, Neg, Rem, Shl, Shr, Sub};

pub trait Integer<Rhs = Self, Output = Self>:
    Sized
    + PartialEq
    + PartialOrd
    + Ord
    + Eq
    + Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
    + Shr<Rhs, Output = Output>
    + Shl<Rhs, Output = Output>
{
}

impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for usize {}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for isize {}

pub trait UnsignedInteger: Integer {}

impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}
impl UnsignedInteger for u128 {}
impl UnsignedInteger for usize {}

pub trait SignedInteger<Rhs = Self, Output = Self>: Integer + Neg<Output = Self> {}

impl SignedInteger for i8 {}
impl SignedInteger for i16 {}
impl SignedInteger for i32 {}
impl SignedInteger for i64 {}
impl SignedInteger for i128 {}
impl SignedInteger for isize {}
