use crate::{sample::Sample, second::Second, utils::gen_unit};

gen_unit!(Beat);

impl Beat {
    pub const ZERO: Beat = Beat::new_raw(0, 1);

    pub const WHOLE: Beat = Beat::new_raw(4, 1);
    pub const HALF_DOTTED: Beat = Beat::new_raw(3, 1);
    pub const HALF: Beat = Beat::new_raw(2, 1);
    pub const HALF_TRIPLET: Beat = Beat::new_raw(4, 3);
    pub const QUARTER_DOTTED: Beat = Beat::new_raw(3, 2);
    pub const QUARTER: Beat = Beat::new_raw(1, 1);
    pub const QUARTER_TRIPLET: Beat = Beat::new_raw(2, 3);
    pub const EIGHTH_DOTTED: Beat = Beat::new_raw(3, 4);
    pub const EIGHTH: Beat = Beat::new_raw(1, 2);
    pub const EIGHTH_TRIPLET: Beat = Beat::new_raw(1, 3);
    pub const SIXTEENTH_DOTTED: Beat = Beat::new_raw(3, 8);
    pub const SIXTEENTH: Beat = Beat::new_raw(1, 4);
    pub const SIXTEENTH_TRIPLET: Beat = Beat::new_raw(1, 6);

    pub const BPM_80: Beat = Beat::new_raw(80, 1);
    pub const BPM_100: Beat = Beat::new_raw(100, 1);
    pub const BPM_120: Beat = Beat::new_raw(120, 1);
    pub const BPM_128: Beat = Beat::new_raw(128, 1);

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
    fn constants() {
        // triplets
        assert_eq!(Beat::HALF_TRIPLET * 3, Beat::WHOLE);
        assert_eq!(Beat::QUARTER_TRIPLET * 3, Beat::HALF);
        assert_eq!(Beat::EIGHTH_TRIPLET * 3, Beat::QUARTER);
        assert_eq!(Beat::SIXTEENTH_TRIPLET * 3, Beat::EIGHTH);

        // dotted
        assert_eq!(Beat::HALF_DOTTED, Beat::QUARTER * 3);
        assert_eq!(Beat::QUARTER_DOTTED, Beat::EIGHTH * 3);
        assert_eq!(Beat::EIGHTH_DOTTED, Beat::SIXTEENTH * 3);
        assert_eq!(Beat::SIXTEENTH_DOTTED, Beat::SIXTEENTH * 3 / 2);
    }

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
