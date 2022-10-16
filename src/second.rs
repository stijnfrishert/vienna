use crate::{utils::gen_unit, Beat, Sample};

gen_unit!(Second);

impl Second {
    pub const HALF: Second = Second::new_raw(1, 2);
    pub const QUARTER: Second = Second::new_raw(1, 4);

    pub const DAY: Second = Second::new_raw(60 * 60 * 24, 1);
    pub const HOUR: Second = Second::new_raw(60 * 60, 1);
    pub const MINUTE: Second = Second::new_raw(60, 1);
    pub const MILLI_SECOND: Second = Second::new_raw(1, 1000);
    pub const MICRO_SECOND: Second = Second::new_raw(1, 1000 * 1000);
    pub const NANO_SECOND: Second = Second::new_raw(1, 1000 * 1000 * 1000);

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
    use crate::Time;

    #[test]
    fn to_beat() {
        assert_eq!(Second::ONE.to_beat(Beat::BPM_120), Beat::HALF);
        assert_eq!(Second::HALF.to_beat(Beat::BPM_120), Beat::QUARTER);
        assert_eq!(Second::QUARTER.to_beat(Beat::BPM_120), Beat::EIGHTH);
    }

    #[test]
    fn to_sample() {
        assert_eq!(Second::ONE.to_sample(Sample::SR_44100), Sample::SR_44100);
        assert_eq!(Second::HALF.to_sample(Sample::SR_44100), Sample::SR_22050);
        assert_eq!(
            Second::QUARTER.to_sample(Sample::SR_44100),
            Sample::SR_11025
        );
    }
}
