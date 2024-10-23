use super::*;
use anyhow::Result;
use extend::ext;
use std::fmt::Debug;

#[ext(name=MatcherExtClose)]
/// Matcher Extensions for types that implement `CloseTo`: `f32`, `f64`, `Vec3`, etc.
pub impl<T: CloseTo + Copy + Debug> Matcher<T>
// where
// U: CloseTo + std::fmt::Debug + Copy,
{
    fn to_be_close_to(&self, expected: impl Into<T>) -> Result<()> {
        let received = self.value;
        let expected = expected.into();
        let result = T::is_close(received, expected);
        let expected = format!("close to {:?}", expected);
        self.assert_correct_with_received(result, &expected, &received)
    }
    fn to_be_close_to_with_epsilon(&self, expected: impl Into<T>, epsilon: T) -> Result<()> {
        let received = self.value;
        let expected = expected.into();
        let result = T::is_close_with_epsilon(received, expected, epsilon);
        let expected = format!("close to {:?}", expected);
        self.assert_correct_with_received(result, &expected, &received)
    }
}
