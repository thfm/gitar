use crate::{Note, NoteName};
use std::fmt;

/// A guitar with any number of strings.
pub struct Guitar {
    pub strings: Vec<GuitarString>,
}

impl Guitar {
    /// Creates a new guitar.
    ///
    /// The number of strings is detemined by the length of the `tuning` vector,
    /// which holds note values for each open string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gitar::{Guitar, Note, NoteName, standard_tuning};
    ///
    /// fn main() {
    ///     // Creates a guitar with standard tuning (probably an electric,
    ///     // given the number of frets)
    ///     let electric_guitar = Guitar::new(22, standard_tuning());
    ///
    ///     // Has the same intervals as standard tuning, but every note
    ///     // is dropped down a whole tone
    ///     let d_tuning = vec![
    ///         Note::new(NoteName::D, 2),
    ///         Note::new(NoteName::G, 2),
    ///         Note::new(NoteName::C, 3),
    ///         Note::new(NoteName::F, 3),
    ///         Note::new(NoteName::A, 3),
    ///         Note::new(NoteName::D, 4),
    ///     ];
    ///
    ///     // Creates a guitar with the custom tuning
    ///     let acoustic_guitar = Guitar::new(20, d_tuning);
    /// }
    /// ```
    pub fn new(num_frets: usize, tuning: Vec<Note>) -> Self {
        Self {
            strings: tuning
                .iter()
                .rev()
                .map(|open_note| GuitarString::new(*open_note, num_frets))
                .collect(),
        }
    }

    /// Returns the fretboard locations of the given note.
    pub fn locations(&self, note: Note) -> Vec<FretboardLocation> {
        let mut locations = Vec::new();
        for (string_idx, string) in self.strings.iter().enumerate() {
            for (fret_idx, fret) in string.frets.iter().enumerate() {
                if *fret == note {
                    locations.push(FretboardLocation::new(string_idx + 1, fret_idx));
                }
            }
        }

        locations
    }
}

/// Standard, six-string guitar tuning.
pub fn standard_tuning() -> Vec<Note> {
    vec![
        Note::new(NoteName::E, 2),
        Note::new(NoteName::A, 2),
        Note::new(NoteName::D, 3),
        Note::new(NoteName::G, 3),
        Note::new(NoteName::B, 3),
        Note::new(NoteName::E, 4),
    ]
}

/// A location on a fretboard.
///
/// A `fret_number` of 0 indicates an open string.
pub struct FretboardLocation {
    string_number: usize,
    fret_number: usize,
}

impl FretboardLocation {
    pub fn new(string_number: usize, fret_number: usize) -> Self {
        Self {
            string_number,
            fret_number,
        }
    }
}

impl fmt::Display for FretboardLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.fret_number > 0 {
            write!(
                f,
                "String {}, fret {}",
                self.string_number, self.fret_number
            )
        } else {
            write!(f, "Open {} string", self.string_number)
        }
    }
}

/// A single guitar string, holding the note values for each of its frets.
pub struct GuitarString {
    pub frets: Vec<Note>,
}

impl GuitarString {
    pub fn new(open_note: Note, num_frets: usize) -> Self {
        Self {
            // 1 is added to `num_frets` to include the open string
            frets: open_note.into_iter().take(num_frets + 1).collect(),
        }
    }
}
