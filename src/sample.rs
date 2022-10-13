use crate::{beat::Beat, second::Second, utils::gen_unit};
use fraction::Fraction;

gen_unit!(Sample);

impl Sample {
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

    #[test]
    fn to_second() {
        assert_eq!(Sample::SR_44100.to_second(Sample::SR_44100), Second::ONE);
        assert_eq!(Sample::SR_22050.to_second(Sample::SR_44100), Second::HALF);
        assert_eq!(
            Sample::SR_11025.to_second(Sample::SR_44100),
            Second::QUARTER
        );
    }

    #[test]
    fn to_beat() {
        assert_eq!(
            Sample::SR_44100.to_beat(Sample::SR_44100, Beat::BPM_120),
            Beat::HALF
        );
        assert_eq!(
            Sample::SR_22050.to_beat(Sample::SR_44100, Beat::BPM_120),
            Beat::QUARTER
        );
        assert_eq!(
            Sample::SR_11025.to_beat(Sample::SR_44100, Beat::BPM_120),
            Beat::EIGHTH
        );
    }
}
