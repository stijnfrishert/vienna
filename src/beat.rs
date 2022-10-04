use crate::{sample::Sample, second::Second, utils::gen_unit};
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

pub const BPM_80: Beat = Beat(Fraction::new_raw(80, 1));
pub const BPM_100: Beat = Beat(Fraction::new_raw(100, 1));
pub const BPM_120: Beat = Beat(Fraction::new_raw(120, 1));
pub const BPM_128: Beat = Beat(Fraction::new_raw(128, 1));

impl Beat {
    pub fn to_second(&self, bpm: Beat) -> Second {
        let frac = Beat::from(60) / bpm;
        Second::from(frac.0 * self.0)
    }

    pub fn to_sample(&self, bpm: Beat, sample_rate: Sample) -> Sample {
        self.to_second(bpm).to_sample(sample_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        sample::{SR_11025, SR_22050, SR_44100},
        second::{HALF_SECOND, QUARTER_SECOND, SECOND},
    };

    #[test]
    fn to_second() {
        assert_eq!(HALF.to_second(BPM_120), SECOND);
        assert_eq!(QUARTER.to_second(BPM_120), HALF_SECOND);
        assert_eq!(EIGHTH.to_second(BPM_120), QUARTER_SECOND);
    }

    #[test]
    fn to_sample() {
        assert_eq!(HALF.to_sample(BPM_120, SR_44100), SR_44100);
        assert_eq!(QUARTER.to_sample(BPM_120, SR_44100), SR_22050);
        assert_eq!(EIGHTH.to_sample(BPM_120, SR_44100), SR_11025);
    }
}
