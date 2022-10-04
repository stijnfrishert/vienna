macro_rules! gen_unit {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Hash)]
        pub struct $name(fraction::Fraction);

        impl $name {
            pub fn new(numerator: u64, denominator: u64) -> Self {
                Self(fraction::Fraction::new(numerator, denominator))
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

        macro_rules! impl_from {
            ($type:path) => {
                impl From<$type> for $name {
                    fn from(value: $type) -> Self {
                        Self(value.into())
                    }
                }
            };
        }

        impl_from!(i8);
        impl_from!(i16);
        impl_from!(i32);
        impl_from!(i64);
        impl_from!(i128);
        impl_from!(isize);

        impl_from!(u8);
        impl_from!(u16);
        impl_from!(u32);
        impl_from!(u64);
        impl_from!(u128);
        impl_from!(usize);

        impl_from!(fraction::Fraction);

        impl From<$name> for fraction::Fraction {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

pub(crate) use gen_unit;
