use crate::{Guitar, GuitarString};
use minstrel::Note;

/// A `Guitar` builder.
#[derive(Debug)]
pub struct Luthier {
    num_frets: usize,
    tuning: Vec<Note>,
    strings: Vec<GuitarString>,
}

impl Luthier {
    /// Creates a new `Luthier` to build a `Guitar` with the given number
    /// of frets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let luthier = gitar::Luthier::new(21);
    /// ```
    pub fn new(num_frets: usize) -> Self {
        Self {
            num_frets,
            tuning: Vec::new(),
            strings: Vec::new(),
        }
    }

    /// Strings the luthier's `Guitar` with the given `tuning` (a vector
    /// of open string note values).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minstrel::Note;
    /// use std::str::FromStr;
    ///
    /// // Standard tuning for a bass guitar
    /// let tuning = vec![
    ///     Note::from_str("E1").unwrap(),
    ///     Note::from_str("A1").unwrap(),
    ///     Note::from_str("D2").unwrap(),
    ///     Note::from_str("G2").unwrap(),
    /// ];
    ///
    /// let luthier = gitar::Luthier::new(24).string(tuning);
    /// ```
    pub fn string(mut self, tuning: Vec<Note>) -> Self {
        self.tuning = tuning;
        self.strings = self
            .tuning
            .iter()
            .rev()
            .map(|open_note| GuitarString::new(*open_note, self.num_frets))
            .collect();
        self
    }

    /// Puts a 'capo' on the luthier's `Guitar`. This essentially shifts the
    /// fret values of each string up by `fret_number` of semitones. It also
    /// reduces the number of frets on the guitar by the same amount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Because of the capo, the resulting guitar will only have 18 functional
    /// // frets, even though it was initialised with 22
    /// let luthier = gitar::Luthier::new(22)
    ///     .string(gitar::standard_tuning())
    ///     .add_capo(4);
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the given `fret_number` is greater than the
    /// number of frets on the luthier's `Guitar`, or if that `Guitar` has
    /// not been strung.
    pub fn add_capo(mut self, fret_number: usize) -> Self {
        if fret_number > self.num_frets {
            panic!("the capo fret number exceeded the number of frets on the guitar");
        }

        if self.tuning.is_empty() {
            panic!("the guitar must be strung before a capo is added");
        }

        self.num_frets -= fret_number;
        self.strings = self
            .tuning
            .iter()
            .rev()
            .map(|open_note| GuitarString::new(*open_note + fret_number, self.num_frets))
            .collect();
        self
    }

    /// Returns the constructed `Guitar`, consuming the `Luthier`.
    pub fn build(self) -> Guitar {
        Guitar {
            num_frets: self.num_frets,
            strings: self.strings,
        }
    }
}
