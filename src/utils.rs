macro_rules! gen_unit {
    ($name:ident) => {
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(transparent)
        )]
        pub struct $name(pub(crate) fraction::Fraction);

        impl $name {
            pub fn new(numerator: u64, denominator: u64) -> Self {
                Self(fraction::Fraction::new(numerator, denominator))
            }

            pub(crate) const fn new_raw(numerator: u64, denominator: u64) -> Self {
                Self(fraction::Fraction::new_raw(numerator, denominator))
            }
        }

        impl crate::time::Time for $name {
            const ZERO: $name = $name::new_raw(0, 1);
            const ONE: $name = $name::new_raw(1, 1);

            fn round(&self) -> Self {
                Self(self.0.round())
            }

            fn floor(&self) -> Self {
                Self(self.0.floor())
            }

            fn ceil(&self) -> Self {
                Self(self.0.ceil())
            }

            fn to_integer(&self) -> i64 {
                fraction::ToPrimitive::to_i64(&self.0).unwrap()
            }

            fn to_f32(&self) -> f32 {
                fraction::ToPrimitive::to_f32(&self.0).unwrap()
            }

            fn to_f64(&self) -> f64 {
                fraction::ToPrimitive::to_f64(&self.0).unwrap()
            }

            fn min<'a>(&'a self, rhs: &'a Self) -> &'a Self {
                if self < rhs {
                    self
                } else {
                    rhs
                }
            }

            fn max<'a>(&'a self, rhs: &'a Self) -> &'a Self {
                if self > rhs {
                    self
                } else {
                    rhs
                }
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        macro_rules! gen_impls {
            ($type:path) => {
                impl From<$type> for $name {
                    fn from(value: $type) -> Self {
                        Self(value.into())
                    }
                }

                impl<'a> From<&'a $type> for $name {
                    fn from(value: &'a $type) -> Self {
                        Self((*value).into())
                    }
                }

                impl std::ops::Add<$type> for $name {
                    type Output = Self;

                    fn add(self, rhs: $type) -> Self::Output {
                        Self(self.0 + $name::from(rhs).0)
                    }
                }

                impl std::ops::Add<$name> for $type {
                    type Output = $name;

                    fn add(self, rhs: $name) -> Self::Output {
                        $name($name::from(self).0 + rhs.0)
                    }
                }

                impl std::ops::Sub<$type> for $name {
                    type Output = Self;

                    fn sub(self, rhs: $type) -> Self::Output {
                        Self(self.0 - $name::from(rhs).0)
                    }
                }

                impl std::ops::Sub<$name> for $type {
                    type Output = $name;

                    fn sub(self, rhs: $name) -> Self::Output {
                        $name($name::from(self).0 - rhs.0)
                    }
                }

                impl std::ops::Mul<$type> for $name {
                    type Output = Self;

                    fn mul(self, rhs: $type) -> Self::Output {
                        Self(self.0 * $name::from(rhs).0)
                    }
                }

                impl std::ops::Mul<$name> for $type {
                    type Output = $name;

                    fn mul(self, rhs: $name) -> Self::Output {
                        $name($name::from(self).0 * rhs.0)
                    }
                }

                impl std::ops::Div<$type> for $name {
                    type Output = Self;

                    fn div(self, rhs: $type) -> Self::Output {
                        Self(self.0 / $name::from(rhs).0)
                    }
                }

                impl std::ops::Div<$name> for $type {
                    type Output = $name;

                    fn div(self, rhs: $name) -> Self::Output {
                        $name($name::from(self).0 / rhs.0)
                    }
                }
            };
        }

        gen_impls!(i8);
        gen_impls!(i16);
        gen_impls!(i32);
        gen_impls!(i64);
        gen_impls!(i128);
        gen_impls!(isize);

        gen_impls!(u8);
        gen_impls!(u16);
        gen_impls!(u32);
        gen_impls!(u64);
        gen_impls!(u128);
        gen_impls!(usize);

        gen_impls!(f32);
        gen_impls!(f64);

        impl From<$name> for fraction::Fraction {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<fraction::Fraction> for $name {
            fn from(value: fraction::Fraction) -> Self {
                Self(value)
            }
        }

        impl TryFrom<$name> for f32 {
            type Error = crate::OutOfRange;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                fraction::ToPrimitive::to_f32(&value.0).ok_or(crate::OutOfRange)
            }
        }

        impl TryFrom<$name> for f64 {
            type Error = crate::OutOfRange;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                fraction::ToPrimitive::to_f64(&value.0).ok_or(crate::OutOfRange)
            }
        }
    };
}

pub(crate) use gen_unit;
