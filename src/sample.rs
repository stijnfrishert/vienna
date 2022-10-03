use crate::{second::Second, utils::gen_unit};
use fraction::Fraction;

gen_unit!(Sample);

pub const SR_8000: Sample = Sample(Fraction::new_raw(8000, 1));
pub const SR_11025: Sample = Sample(Fraction::new_raw(11025, 1));
pub const SR_16000: Sample = Sample(Fraction::new_raw(16000, 1));
pub const SR_22050: Sample = Sample(Fraction::new_raw(22050, 1));
pub const SR_44100: Sample = Sample(Fraction::new_raw(44100, 1));
pub const SR_48000: Sample = Sample(Fraction::new_raw(48000, 1));
pub const SR_88200: Sample = Sample(Fraction::new_raw(88200, 1));
pub const SR_96000: Sample = Sample(Fraction::new_raw(96000, 1));
pub const SR_176400: Sample = Sample(Fraction::new_raw(176400, 1));
pub const SR_192000: Sample = Sample(Fraction::new_raw(192000, 1));
pub const SR_352800: Sample = Sample(Fraction::new_raw(352800, 1));
pub const SR_384000: Sample = Sample(Fraction::new_raw(384000, 1));

impl Sample {
    pub fn to_second(&self, sample_rate: Sample) -> Second {
        Second::from(self.0 / sample_rate.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::second::HALF_SECOND;

    #[test]
    fn to_second() {
        assert_eq!(SR_22050.to_second(SR_44100), HALF_SECOND);
    }
}
