use std::{io::Error, ops::{Add, Sub}};

pub fn unimplemented_err() -> Result<(), Error> {
    Err(Error::new(std::io::ErrorKind::Other, "Not implemented"))
}

pub fn safe_sub<T>(a: T, lower_bound: T) -> T
where
    T: Sub<Output = T> + PartialEq + Copy + From<usize>,
{
    if a != lower_bound {
        a - T::from(1)
    } else {
        T::from(0)
    }
}

pub fn safe_add<T>(a: T, upper_bound: T) -> T
where
    T: Add<Output = T> + PartialEq + Copy + From<usize>,
{
    if a != upper_bound {
        a + T::from(1)
    } else {
        upper_bound
    }
}

pub enum Lexicon {
    SearchPlaceholder,
    NoEntries,
    NothingFound,
}

impl Lexicon {
    pub fn str(self) -> &'static str {
        match self {
            Lexicon::SearchPlaceholder => "  Press <S> to search...",
            Lexicon::NoEntries => "No bitches?\nPress <N> for new",
            Lexicon::NothingFound => "Nothing found...",
        }
    }
}
