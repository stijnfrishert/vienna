use crate::{second::Second, utils::gen_unit};
use fraction::Fraction;

gen_unit!(Beat);

pub const WHOLE: Beat = Beat(Fraction::new_raw(4, 1));
pub const HALF: Beat = Beat(Fraction::new_raw(2, 1));
pub const HALF_TRIPLET: Beat = Beat(Fraction::new_raw(4, 3));
pub const QUARTER: Beat = Beat(Fraction::new_raw(1, 1));
pub const QUARTER_TRIPLET: Beat = Beat(Fraction::new_raw(2, 3));
pub const EIGHTH: Beat = Beat(Fraction::new_raw(1, 2));
pub const EIGHTH_TRIPLET: Beat = Beat(Fraction::new_raw(1, 3));
pub const SIXTEENTH: Beat = Beat(Fraction::new_raw(1, 4));
pub const SIXTEENTH_TRIPLET: Beat = Beat(Fraction::new_raw(1, 6));

impl Beat {
    pub fn to_second(&self, bpm: Beat) -> Second {
        Second::from(Fraction::new(60u64, 1u64) / bpm.0 * self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::second::HALF_SECOND;

    #[test]
    fn to_second() {
        assert_eq!(QUARTER.to_second(Beat::new(120, 1)), HALF_SECOND);
    }
}
