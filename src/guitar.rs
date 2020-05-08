use minstrel::Note;
use std::{fmt, str::FromStr};

/// A guitar with any number of strings.
#[derive(Debug)]
pub struct Guitar {
    pub(crate) num_frets: usize,
    pub(crate) strings: Vec<GuitarString>,
}

impl Guitar {
    /// Returns the fretboard locations of the given note.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gitar::FretboardLocation;
    /// use minstrel::Note;
    /// use std::str::FromStr;
    ///
    /// let luthier = gitar::Luthier::new(20).string(gitar::standard_tuning());
    /// let guitar = luthier.build();
    ///
    /// let locations = guitar.locations(Note::from_str("E3").unwrap());
    /// assert_eq!(
    ///     locations,
    ///     vec![
    ///         FretboardLocation::new(4, 2),
    ///         FretboardLocation::new(5, 7),
    ///         FretboardLocation::new(6, 12)
    ///     ]
    /// );
    ///
    /// ```
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

/// A single guitar string, represented as the note values of
/// each of its frets.
#[derive(Debug)]
pub(crate) struct GuitarString {
    frets: Vec<Note>,
}

impl GuitarString {
    /// Creates a new `GuitarString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gitar::GuitarString;
    /// use minstrel::Note;
    /// use std::str::FromStr;
    ///
    /// let e_string = GuitarString::new(Note::from_str("E2"), 20);
    /// ```
    pub(crate) fn new(open_note: Note, num_frets: usize) -> Self {
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

/// A diagram of a `Guitar` fretboard, depicting the locations of certain notes.
pub struct FretboardDiagram<'g> {
    guitar: &'g Guitar,
    locations: Vec<FretboardLocation>,
}

impl<'g> FretboardDiagram<'g> {
    /// Creates a new `FretboardDiagram` based on the given `guitar` and
    /// fretboard `locations`.
    pub fn new(guitar: &'g Guitar, locations: Vec<FretboardLocation>) -> Self {
        Self { guitar, locations }
    }
}

impl<'g> fmt::Display for FretboardDiagram<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The values can be unwrapped because the case in which there are no fretboard
        // locations has already been handled (see the above `match` statement)
        let fret_numbers = self.locations.iter().map(|loc| loc.fret_number);
        let lowest_fret_num = fret_numbers.clone().min().unwrap();
        let highest_fret_num = fret_numbers.max().unwrap();

        // Draws a fretboard diagram showing all of the note locations
        for fret_idx in lowest_fret_num..=highest_fret_num {
            for string_num in (1..=self.guitar.strings.len()).rev() {
                let current_loc = FretboardLocation::new(string_num, fret_idx);
                if self.locations.contains(&current_loc) {
                    f.write_str("∗")?;
                } else if fret_idx == 0 {
                    f.write_str("-")?;
                } else {
                    f.write_str("│")?;
                }
            }

            writeln!(f, " {}", fret_idx)?;
        }

        Ok(())
    }
}

/// A location on a fretboard.
///
/// A `fret_number` of 0 indicates an open string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FretboardLocation {
    string_number: usize,
    fret_number: usize,
}

impl FretboardLocation {
    fn new(string_number: usize, fret_number: usize) -> Self {
        Self {
            string_number,
            fret_number,
        }
    }
}
