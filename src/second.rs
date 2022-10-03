use crate::{beat::Beat, utils::gen_unit};
use fraction::Fraction;

gen_unit!(Second);

pub const DAY: Second = Second(Fraction::new_raw(60 * 60 * 24, 1));
pub const HOUR: Second = Second(Fraction::new_raw(60 * 60, 1));
pub const MINUTE: Second = Second(Fraction::new_raw(60, 1));
pub const SECOND: Second = Second(Fraction::new_raw(1, 1));
pub const HALF_SECOND: Second = Second(Fraction::new_raw(1, 2));
pub const QUARTER_SECOND: Second = Second(Fraction::new_raw(1, 3));
pub const MILLI_SECOND: Second = Second(Fraction::new_raw(1, 1000));
pub const MICRO_SECOND: Second = Second(Fraction::new_raw(1, 1000 * 1000));
pub const NANO_SECOND: Second = Second(Fraction::new_raw(1, 1000 * 1000 * 1000));

impl Second {
    pub fn to_beat(&self, bpm: Beat) -> Beat {
        Beat::from(Fraction::new(60u64, 1u64) / bpm.into() / self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beat::QUARTER;

    #[test]
    fn to_second() {
        assert_eq!(HALF_SECOND.to_beat(Beat::new(120, 1)), QUARTER);
    }
}
