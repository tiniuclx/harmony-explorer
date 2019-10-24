#[allow(dead_code, non_upper_case_globals)]
pub mod interval {
    type Interval = i64;
    pub const Unison: Interval = 0;
    pub const Min2nd: Interval = 1;
    pub const Maj2nd: Interval = 2;
    pub const Min3rd: Interval = 3;
    pub const Maj3rd: Interval = 4;
    pub const Per4th: Interval = 5;
    pub const Dim5th: Interval = 6;
    pub const Per5th: Interval = 7;
    pub const Min6th: Interval = 8;
    pub const Maj6th: Interval = 9;
    pub const Min7th: Interval = 10;
    pub const Maj7th: Interval = 11;
    pub const Octave: Interval = 12;
}
#[cfg(test)]
mod tests {
    use super::*;
    use interval::*;
    #[allow(unused_imports)]
    use pitch_calc::Letter::*;

    #[test]
    fn interval_maths() {
        assert_eq!(C, F + Per5th);
        assert_eq!(C, C + Octave);
        assert_eq!(C, C + Octave + Octave);

        assert_eq!(C, G - Per5th);
        assert_eq!(C, C - Octave);
        assert_eq!(C, C - Per4th - Per5th);
        assert_eq!(C - Min2nd, C + Maj7th);
    }
}
