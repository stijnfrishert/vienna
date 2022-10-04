use crate::{beat::Beat, sample::Sample, utils::gen_unit};
use fraction::Fraction;

gen_unit!(Second);

pub const DAY: Second = Second(Fraction::new_raw(60 * 60 * 24, 1));
pub const HOUR: Second = Second(Fraction::new_raw(60 * 60, 1));
pub const MINUTE: Second = Second(Fraction::new_raw(60, 1));
pub const SECOND: Second = Second(Fraction::new_raw(1, 1));
pub const HALF_SECOND: Second = Second(Fraction::new_raw(1, 2));
pub const QUARTER_SECOND: Second = Second(Fraction::new_raw(1, 4));
pub const MILLI_SECOND: Second = Second(Fraction::new_raw(1, 1000));
pub const MICRO_SECOND: Second = Second(Fraction::new_raw(1, 1000 * 1000));
pub const NANO_SECOND: Second = Second(Fraction::new_raw(1, 1000 * 1000 * 1000));

impl Second {
    pub fn to_beat(&self, bpm: Beat) -> Beat {
        let frac = bpm / Beat::from(60);
        Beat::from(self.0 * frac.0)
    }

    pub fn to_sample(&self, sample_rate: Sample) -> Sample {
        Sample::from(self.0 * sample_rate.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        beat::{BPM_120, EIGHTH, HALF, QUARTER},
        sample::{SR_11025, SR_22050, SR_44100},
    };

    #[test]
    fn to_beat() {
        assert_eq!(SECOND.to_beat(BPM_120), HALF);
        assert_eq!(HALF_SECOND.to_beat(BPM_120), QUARTER);
        assert_eq!(QUARTER_SECOND.to_beat(BPM_120), EIGHTH);
    }

    #[test]
    fn to_sample() {
        assert_eq!(SECOND.to_sample(SR_44100), SR_44100);
        assert_eq!(HALF_SECOND.to_sample(SR_44100), SR_22050);
        assert_eq!(QUARTER_SECOND.to_sample(SR_44100), SR_11025);
    }
}
