use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

/// A valid quest number of advent (i.e. an integer in range 1 to 25).
///
/// # Display
/// This value displays as a two digit number.
///
/// ```
/// # use shared::quest::Quest;
/// const QUEST: Quest = Quest::try_new(8).unwrap();
/// assert_eq!(QUEST.to_string(), "08")
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quest(u8);

impl Quest {
    /// Creates a [`Quest`] from the provided value if it's in the valid range,
    /// returns [`None`] otherwise.
    #[must_use]
    pub const fn try_new(quest: u8) -> Option<Self> {
        if quest == 0 || quest > 25 {
            return None;
        }

        Some(Quest(quest))
    }

    // Not part of the public API
    #[must_use]
    #[doc(hidden)]
    pub const fn new_unchecked(quest: u8) -> Self {
        Self(quest)
    }

    /// Converts the [`Quest`] into an [`u8`].
    #[must_use]
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl PartialEq<u8> for Quest {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u8> for Quest {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl FromStr for Quest {
    type Err = QuestFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let quest = s.parse().map_err(|_| QuestFromStrError {})?;
        Self::try_new(quest).ok_or(QuestFromStrError {})
    }
}

/// An error which can be returned when parsing a [`Quest`].
#[expect(clippy::module_name_repetitions, reason = "Name clarity")]
#[derive(Debug)]
pub struct QuestFromStrError {}

impl Error for QuestFromStrError {}

impl Display for QuestFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("expecting a quest number between 1 and 25")
    }
}

/// Creates a [`Quest`] value in a `const` context.
#[macro_export]
macro_rules! quest {
    ($quest:literal) => {{
        $crate::quest::Quest::try_new($quest).expect(concat!(
            "invalid quest number `",
            $quest,
            "`, expecting a value between 1 and 25"
        ))
    }};
}
