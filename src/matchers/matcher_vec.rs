use super::*;
use anyhow::Result;
use std::fmt::Debug;

impl<T: Debug> Matcher<&Vec<T>> {
    pub fn to_be_empty(&self) -> Result<()> {
        let result = self.value.is_empty();
        let expected = format!("to be empty");
        self.assert_correct(result, &expected)
    }
    pub fn any(&self, func: impl FnMut(&T) -> bool) -> Result<()> {
        let result = self.value.iter().any(func);
        let expected = format!("any to match predicate");
        self.assert_correct(result, &expected)
    }
}

impl<T: Debug + PartialEq> Matcher<&Vec<T>> {
    pub fn to_contain(&self, other: &T) -> Result<()> {
        let result = self.value.contains(other);
        let expected = format!("to contain {:?}", other);
        self.assert_correct(result, &expected)
    }
}
