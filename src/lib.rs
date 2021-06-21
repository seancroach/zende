//! # Zende
//!
//! Zende is a minimal library for the ***z***igzag ***en***coding and
//! ***de***coding of integers.

#![doc(html_root_url = "https://docs.rs/zende/0.1.3")]

use std::mem;

const BITS_PER_BYTE: usize = 8;

/// An extension trait implemented for all Rust integers to encode and decode
/// between signed and unsigned variants using
/// [zigzag encoding][wiki].
///
/// # Examples
///
/// Encoding a signed integer:
///
/// ```
/// use zende::Zigzag;
///
/// assert_eq!(0i8.zigzag(), 0u8);
/// assert_eq!((-1i8).zigzag(), 1u8);
/// assert_eq!(1i8.zigzag(), 2u8);
///
/// assert_eq!(i8::MIN.zigzag(), u8::MAX);
/// assert_eq!(i16::MIN.zigzag(), u16::MAX);
/// assert_eq!(i32::MIN.zigzag(), u32::MAX);
/// assert_eq!(i64::MIN.zigzag(), u64::MAX);
/// assert_eq!(i128::MIN.zigzag(), u128::MAX);
///
/// assert_eq!(isize::MIN.zigzag(), usize::MAX);
/// ```
///
/// Decoding an unsigned integer:
///
/// ```
/// use zende::Zigzag;
///
/// assert_eq!(0u8.zigzag(), 0i8);
/// assert_eq!(1u8.zigzag(), -1i8);
/// assert_eq!(2u8.zigzag(), 1i8);
///
/// assert_eq!(u8::MAX.zigzag(), i8::MIN);
/// assert_eq!(u16::MAX.zigzag(), i16::MIN);
/// assert_eq!(u32::MAX.zigzag(), i32::MIN);
/// assert_eq!(u64::MAX.zigzag(), i64::MIN);
/// assert_eq!(u128::MAX.zigzag(), i128::MIN);
///
/// assert_eq!(usize::MAX.zigzag(), isize::MIN);
/// ```
///
/// [wiki]: https://en.wikipedia.org/wiki/Variable-length_quantity#Zigzag_encoding
pub trait Zigzag<T>: private::Sealed {
    /// Converts signed integers to unsigned integers and vice versa using
    /// [zigzag encoding][wiki].
    ///
    /// # Examples
    ///
    /// ```
    /// use zende::Zigzag;
    ///
    /// assert_eq!(0i8.zigzag(), 0u8);
    /// assert_eq!((-1i8).zigzag(), 1u8);
    /// assert_eq!(1i8.zigzag(), 2u8);
    ///
    /// assert_eq!(0u8.zigzag(), 0i8);
    /// assert_eq!(1u8.zigzag(), -1i8);
    /// assert_eq!(2u8.zigzag(), 1i8);
    /// ```
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Variable-length_quantity#Zigzag_encoding
    fn zigzag(self) -> T;
}

macro_rules! impl_zigzag {
    ($(($signed:ty, $unsigned:ty)),*) => {
        $(
            impl Zigzag<$unsigned> for $signed {
                #[inline]
                fn zigzag(self) -> $unsigned {
                    const TYPE_BITS: usize = mem::size_of::<$unsigned>() * BITS_PER_BYTE;
                    (self >> TYPE_BITS - 1) as $unsigned ^ (self << 1) as $unsigned
                }
            }

            impl Zigzag<$signed> for $unsigned {
                #[inline]
                fn zigzag(self) -> $signed {
                    (self >> 1) as $signed ^ -((self & 1) as $signed)
                }
            }
        )*
    }
}

impl_zigzag!(
    (i8, u8),
    (i16, u16),
    (i32, u32),
    (i64, u64),
    (i128, u128),
    (isize, usize)
);

mod private {
    pub trait Sealed {}

    macro_rules! impl_sealed {
        ($($t:ty),*) => {
            $(
                impl Sealed for $t {}
            )*
        }
    }

    impl_sealed!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
