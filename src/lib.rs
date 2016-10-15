extern crate num;

pub mod prelude {
    pub use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
    pub use std::cmp::{PartialEq, Eq, PartialOrd, Ord};
    pub use std::convert::From;
    pub use num::{Zero, One, Num, Bounded, Saturating};
    pub use num::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};
    pub use std::fmt;
}

#[macro_export]
macro_rules! fixed_point_impl {
    ($name:ident: $itype:ty, $bigitype:ty, $fbits:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name {
            pub base: $itype,
        }

        impl $name {
            pub fn new(base: $itype) -> Self {
                $name { base: base }
            }

            pub fn from_int(n: $itype) -> Option<Self> {
                if n < Self::min_value().to_int() ||
                    n > Self::max_value().to_int() {
                    None
                } else {
                    Some($name {
                        base: (n as $itype) << $fbits,
                    })
                }
            }

            pub fn from_float(n: f64) -> Option<Self> {
                if n < Self::min_value().to_float() ||
                    n > Self::max_value().to_float() {
                    None
                } else {
                    Some($name {
                        base: (n * ((1 << $fbits) as f64)) as $itype,
                    })
                }
            }

            pub fn to_int(&self) -> $itype {
                (self.base >> $fbits) as $itype
            }

            pub fn to_float(&self) -> f64 {
                (self.base as f64) / ((1 << $fbits) as f64)
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                $name {
                    base: self.base + rhs.base
                }
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                $name {
                    base: self.base - rhs.base
                }
            }
        }

        impl Mul for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                $name {
                    base: ((self.base as $bigitype * rhs.base as $bigitype) >> $fbits) as $itype,
                }
            }
        }

        impl Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                $name {
                    base: (((self.base as $bigitype) << $fbits) / (rhs.base as $bigitype)) as $itype,
                }
            }
        }

        impl Rem for $name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                $name {
                    base: self.base % rhs.base,
                }
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.base += rhs.base;
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.base -= rhs.base;
            }
        }

        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                self.base = ((self.base as $bigitype * rhs.base as $bigitype) >> $fbits) as $itype;
            }
        }

        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                self.base = (((self.base as $bigitype) << $fbits) / (rhs.base as $bigitype)) as $itype;
            }
        }

        impl RemAssign for $name {
            fn rem_assign(&mut self, rhs: Self) {
                self.base %= rhs.base;
            }
        }

        impl Zero for $name {
            fn zero() -> Self {
                $name {
                    base: 0,
                }
            }

            fn is_zero(&self) -> bool {
                self.base == 0
            }
        }

        impl One for $name {
            fn one() -> Self {
                $name {
                    base: 1 << $fbits,
                }
            }
        }

        impl Num for $name {
            type FromStrRadixErr = ();
            fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                unimplemented!();
            }
        }

        impl From<$itype> for $name {
            fn from(n: $itype) -> Self {
                Self::from_int(n).unwrap()
            }
        }

        impl From<f64> for $name {
            fn from(n: f64) -> Self {
                Self::from_float(n).unwrap()
            }
        }

        impl Into<$itype> for $name {
            fn into(self) -> $itype {
                self.to_int()
            }
        }

        impl Into<f64> for $name {
            fn into(self) -> f64 {
                self.to_float()
            }
        }

        impl Bounded for $name {
            fn min_value() -> Self {
                $name {
                    base: Bounded::min_value(),
                }
            }

            fn max_value() -> Self {
                $name {
                    base: Bounded::max_value(),
                }
            }
        }

        impl Saturating for $name {
            fn saturating_add(self, rhs: Self) -> Self {
                $name {
                    base: self.base.saturating_add(rhs.base),
                }
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                $name {
                    base: self.base.saturating_sub(rhs.base),
                }
            }
        }

        impl CheckedAdd for $name {
            fn checked_add(&self, rhs: &Self) -> Option<Self> {
                self.base.checked_add(rhs.base).map(|b| $name {
                    base: b,
                })
            }
        }

        impl CheckedSub for $name {
            fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                self.base.checked_sub(rhs.base).map(|b| $name {
                    base: b,
                })
            }
        }

        impl CheckedMul for $name {
            fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                (self.base as $bigitype).checked_mul(rhs.base as $bigitype).and_then(|mut base_double| {
                    base_double = base_double >> $fbits;
                    let $name { base: max_base } = Self::max_value();
                    let $name { base: min_base } = Self::min_value();
                    if base_double > max_base as $bigitype || base_double < min_base as $bigitype {
                        None
                    } else {
                        Some($name {
                            base: base_double as $itype
                        })
                    }
                })
            }
        }

        impl CheckedDiv for $name {
            fn checked_div(&self, rhs: &Self) -> Option<Self> {
                ((self.base as $bigitype) << $fbits).checked_div(rhs.base as $bigitype)
                    .and_then(|base_double| {
                        let $name { base: max_base } = Self::max_value();
                        let $name { base: min_base } = Self::min_value();
                        if base_double > max_base as $bigitype || base_double < min_base as $bigitype {
                            None
                        } else {
                            Some($name {
                                base: base_double as $itype
                            })
                        }
                    })
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.to_float().fmt(f)
            }
        }

        impl fmt::Binary for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.base.fmt(f)
            }
        }

        impl fmt::UpperHex for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.base.fmt(f)
            }
        }

        impl fmt::LowerHex for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.base.fmt(f)
            }
        }
    };
}

#[test]
fn test() {
    use self::prelude::*;
    fixed_point_impl!(Fixed: i32, i64, 4);

    let mut num = Fixed::from(5);
    num += Fixed::from(5);
    num = num * Fixed::from(5) / Fixed::from(9);
    println!("{}", num < Fixed::from(7));
}
