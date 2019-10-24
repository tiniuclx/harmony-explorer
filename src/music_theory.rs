#[allow(unused_imports)]
use pitch_calc::Letter::*;

#[allow(dead_code)]
pub enum Interval {
    Unison = 0,
    Min2nd = 1,
    Maj2nd = 2,
    Min3rd = 3,
    Maj3rd = 4,
    Per4th = 5,
    Dim5th = 6,
    Per5th = 7,
    Min6th = 8,
    Maj6th = 9,
    Min7th = 10,
    Maj7th = 11,
    Octave = 12,
}

#[cfg(test)]
mod tests {
    use super::*;
    use Interval::*;

    #[test]
    fn small_transposition() {
        assert_eq!(G, C + 7);
    }

    #[test]
    fn big_transposition() {
        assert_eq!(C, C + 12);
        assert_eq!(C, C + 24);
        assert_eq!(C, C + 144);
    }

    #[test]
    fn transpose_down() {
        assert_eq!(Db, D - 1);
        assert_eq!(C, G - 7);
        assert_eq!(C, C - 24);
    }

    #[test]
    fn interval_maths() {
        assert_eq!(C, F + Per5th as i64);
        assert_eq!(C, C + Octave as i64);
        assert_eq!(C, C + Octave as i64 + Octave as i64);

        assert_eq!(C, G - Per5th as i64);
        assert_eq!(C, C - Octave as i64);
        assert_eq!(C, C - Per4th as i64 - Per5th as i64);
        assert_eq!(C - Min2nd as i64, C + Maj7th as i64);
    }
}
