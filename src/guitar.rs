use {
    crate::Note,
    std::{fmt, str::FromStr},
};

/// A guitar with any number of strings.
#[derive(Debug)]
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
    /// use gitar::{Guitar, Note, standard_tuning};
    /// use std::str::FromStr;
    ///
    /// // Creates a guitar with standard tuning (probably an electric,
    /// // given the number of frets)
    /// let electric_guitar = Guitar::new(22, standard_tuning());
    ///
    /// // Open D tuning
    /// let d_tuning = vec![
    ///     Note::from_str("D2").unwrap(),
    ///     Note::from_str("A2").unwrap(),
    ///     Note::from_str("D3").unwrap(),
    ///     Note::from_str("Gb3").unwrap(),
    ///     Note::from_str("A3").unwrap(),
    ///     Note::from_str("D4").unwrap(),
    /// ];
    ///
    /// // Creates a guitar with the custom tuning
    /// let acoustic_guitar = Guitar::new(20, d_tuning);
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

/// A single guitar string, holding the note values for each of its frets.
#[derive(Debug)]
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

/// Standard, six-string guitar tuning.
pub fn standard_tuning() -> Vec<Note> {
    vec![
        Note::from_str("E2").unwrap(),
        Note::from_str("A2").unwrap(),
        Note::from_str("D3").unwrap(),
        Note::from_str("G3").unwrap(),
        Note::from_str("B3").unwrap(),
        Note::from_str("E4").unwrap(),
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
