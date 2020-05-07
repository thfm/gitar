use minstrel::{Interval, Note};
use std::{fmt, str::FromStr};

pub struct Luthier {
    num_frets: usize,
    tuning: Vec<Note>,
    strings: Vec<GuitarString>,
}

impl Luthier {
    pub fn new(num_frets: usize) -> Self {
        Self {
            num_frets,
            tuning: Vec::new(),
            strings: Vec::new(),
        }
    }

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
            .map(|open_note| {
                GuitarString::new(*open_note + Interval::new(fret_number), self.num_frets)
            })
            .collect();
        self
    }

    pub fn build(self) -> Guitar {
        Guitar {
            num_frets: self.num_frets,
            strings: self.strings,
        }
    }
}

/// A guitar with any number of strings.
#[derive(Debug)]
pub struct Guitar {
    num_frets: usize,
    strings: Vec<GuitarString>,
}

impl Guitar {
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

pub struct FretboardDiagram<'g> {
    guitar: &'g Guitar,
    locations: Vec<FretboardLocation>,
}

impl<'g> FretboardDiagram<'g> {
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
    pub string_number: usize,
    pub fret_number: usize,
}

impl FretboardLocation {
    pub fn new(string_number: usize, fret_number: usize) -> Self {
        Self {
            string_number,
            fret_number,
        }
    }
}
