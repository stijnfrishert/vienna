use crate::{beat::Beat, second::Second, utils::gen_unit};
use fraction::Fraction;

gen_unit!(Sample);

pub const ZERO: Sample = Sample(Fraction::new_raw(0, 1));
pub const ONE: Sample = Sample(Fraction::new_raw(1, 1));

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

    pub fn to_beat(&self, sample_rate: Sample, bpm: Beat) -> Beat {
        self.to_second(sample_rate).to_beat(bpm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        beat::{BPM_120, EIGHTH, HALF, QUARTER},
        second::{HALF_SECOND, QUARTER_SECOND, SECOND},
    };

    #[test]
    fn to_second() {
        assert_eq!(SR_44100.to_second(SR_44100), SECOND);
        assert_eq!(SR_22050.to_second(SR_44100), HALF_SECOND);
        assert_eq!(SR_11025.to_second(SR_44100), QUARTER_SECOND);
    }

    #[test]
    fn to_beat() {
        assert_eq!(SR_44100.to_beat(SR_44100, BPM_120), HALF);
        assert_eq!(SR_22050.to_beat(SR_44100, BPM_120), QUARTER);
        assert_eq!(SR_11025.to_beat(SR_44100, BPM_120), EIGHTH);
    }
}
