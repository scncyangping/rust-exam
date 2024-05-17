use std::ops::Deref;

use chrono::Local;
use serde::Deserialize;

/// Alias for [`chrono::DateTime<Local>`](chrono::DateTime).
type LocalDateTime = chrono::DateTime<Local>;

/// A wrapper type for [`chrono::DateTime<Local>`](chrono::DateTime).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct DateTime(LocalDateTime);

impl From<LocalDateTime> for DateTime {
    fn from(value: LocalDateTime) -> Self {
        DateTime(value)
    }
}

impl From<DateTime> for LocalDateTime {
    fn from(value: DateTime) -> Self {
        value.0
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<Local>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}