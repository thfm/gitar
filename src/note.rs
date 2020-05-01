use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map},
    sequence::pair,
};
use std::{fmt, str::FromStr};

/// Holds the name (C, Db, D etc.) and octave number of a single note.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Note {
    pub name: NoteName,
    pub octave: u32,
}

impl Note {
    pub fn new(name: NoteName, octave: u32) -> Self {
        Self { name, octave }
    }

    /// Attempts to parse a `Note` from a string slice.
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        let (s, (name, octave)) = pair(NoteName::parse, digit1)(s)?;
        let octave = u32::from_str(octave).unwrap();

        Ok((s, Self::new(name, octave)))
    }
}

impl FromStr for Note {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Interprets the result from parsing
        match all_consuming(Note::parse)(s) {
            Ok((_, note)) => Ok(note),
            Err(_) => anyhow::bail!("failed to parse note"),
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.name, self.octave)
    }
}

impl IntoIterator for Note {
    type Item = Self;
    type IntoIter = NoteIter;

    fn into_iter(self) -> Self::IntoIter {
        NoteIter {
            note: self,
            first: true,
        }
    }
}

/// An iterator over a `Note`.
pub struct NoteIter {
    note: Note,
    first: bool,
}

impl Iterator for NoteIter {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        // Returns the original note if this is the first call to `next`
        if self.first {
            self.first = false;
            Some(self.note)
        } else {
            // Moves the note up by one semitone
            let (name, octave) = match (self.note.name, self.note.octave) {
                (NoteName::C, oct) => (NoteName::Db, oct),
                (NoteName::Db, oct) => (NoteName::D, oct),
                (NoteName::D, oct) => (NoteName::Eb, oct),
                (NoteName::Eb, oct) => (NoteName::E, oct),
                (NoteName::E, oct) => (NoteName::F, oct),
                (NoteName::F, oct) => (NoteName::Gb, oct),
                (NoteName::Gb, oct) => (NoteName::G, oct),
                (NoteName::G, oct) => (NoteName::Ab, oct),
                (NoteName::Ab, oct) => (NoteName::A, oct),
                (NoteName::A, oct) => (NoteName::Bb, oct),
                (NoteName::Bb, oct) => (NoteName::B, oct),
                (NoteName::B, oct) => (NoteName::C, oct + 1),
            };

            self.note.name = name;
            self.note.octave = octave;

            Some(self.note)
        }
    }
}

/// The names for all twelve notes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NoteName {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl NoteName {
    /// Attempts to parse a `NoteName` from a string slice.
    fn parse(s: &str) -> nom::IResult<&str, Self> {
        alt((
            map(tag("C"), |_| Self::C),
            map(tag("Db"), |_| Self::Db),
            map(tag("D"), |_| Self::D),
            map(tag("Eb"), |_| Self::Eb),
            map(tag("E"), |_| Self::E),
            map(tag("F"), |_| Self::F),
            map(tag("Gb"), |_| Self::Gb),
            map(tag("G"), |_| Self::G),
            map(tag("Ab"), |_| Self::Ab),
            map(tag("A"), |_| Self::A),
            map(tag("Bb"), |_| Self::Bb),
            map(tag("B"), |_| Self::B),
        ))(s)
    }
}
