use crate::{sample::Sample, second::Second, utils::gen_unit};
use fraction::Fraction;

gen_unit!(Beat);

impl Beat {
    pub const ZERO: Beat = Beat(Fraction::new_raw(0, 1));
    pub const ONE: Beat = Beat(Fraction::new_raw(1, 1));
    pub const TWO: Beat = Beat(Fraction::new_raw(2, 1));

    pub const WHOLE: Beat = Beat(Fraction::new_raw(4, 1));
    pub const HALF: Beat = Beat(Fraction::new_raw(2, 1));
    pub const HALF_TRIPLET: Beat = Beat(Fraction::new_raw(4, 3));
    pub const QUARTER: Beat = Self::ONE;
    pub const QUARTER_TRIPLET: Beat = Beat(Fraction::new_raw(2, 3));
    pub const EIGHTH: Beat = Beat(Fraction::new_raw(1, 2));
    pub const EIGHTH_TRIPLET: Beat = Beat(Fraction::new_raw(1, 3));
    pub const SIXTEENTH: Beat = Beat(Fraction::new_raw(1, 4));
    pub const SIXTEENTH_TRIPLET: Beat = Beat(Fraction::new_raw(1, 6));

    pub const BPM_80: Beat = Beat(Fraction::new_raw(80, 1));
    pub const BPM_100: Beat = Beat(Fraction::new_raw(100, 1));
    pub const BPM_120: Beat = Beat(Fraction::new_raw(120, 1));
    pub const BPM_128: Beat = Beat(Fraction::new_raw(128, 1));

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

    #[test]
    fn to_second() {
        assert_eq!(Beat::HALF.to_second(Beat::BPM_120), Second::ONE);
        assert_eq!(Beat::QUARTER.to_second(Beat::BPM_120), Second::HALF);
        assert_eq!(Beat::EIGHTH.to_second(Beat::BPM_120), Second::QUARTER);
    }

    #[test]
    fn to_sample() {
        assert_eq!(
            Beat::HALF.to_sample(Beat::BPM_120, Sample::SR_44100),
            Sample::SR_44100
        );
        assert_eq!(
            Beat::QUARTER.to_sample(Beat::BPM_120, Sample::SR_44100),
            Sample::SR_22050
        );
        assert_eq!(
            Beat::EIGHTH.to_sample(Beat::BPM_120, Sample::SR_44100),
            Sample::SR_11025
        );
    }
}
