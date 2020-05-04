use nom::{branch::alt, bytes::complete::tag, combinator::map};
use std::{fmt, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Note(u32);

impl Note {
    pub fn new(value: u32) -> Self {
        Note(value)
    }
}

impl FromStr for Note {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, name) = alt((
            map(tag("C"), |_| 0),
            map(tag("Db"), |_| 1),
            map(tag("D"), |_| 2),
            map(tag("Eb"), |_| 3),
            map(tag("E"), |_| 4),
            map(tag("F"), |_| 5),
            map(tag("Gb"), |_| 6),
            map(tag("G"), |_| 7),
            map(tag("Ab"), |_| 8),
            map(tag("A"), |_| 9),
            map(tag("Bb"), |_| 10),
            map(tag("B"), |_| 11),
        ))(s)
        .map_err(|_: nom::Err<(&str, nom::error::ErrorKind)>| {
            anyhow::anyhow!("failed to parse note name")
        })?;

        let octave = u32::from_str(s)?;

        Ok(Self::new(name + octave * 12))
    }
}

#[cfg(test)]
#[test]
fn from_str() {
    assert_eq!(Note::from_str("C0").unwrap(), Note(0));
    assert_eq!(Note::from_str("Db3").unwrap(), Note(37));
    assert_eq!(Note::from_str("Bb10").unwrap(), Note(130));
    assert!(Note::from_str("Ab").is_err()); // No octave
    assert!(Note::from_str("Cb2").is_err()); // Invalid note name
    assert!(Note::from_str("Gb-2").is_err()); // Invalid octave
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.0 % 12 {
            0 => "C",
            1 => "Db",
            2 => "D",
            3 => "Eb",
            4 => "E",
            5 => "F",
            6 => "Gb",
            7 => "G",
            8 => "Ab",
            9 => "A",
            10 => "Bb",
            11 => "B",
            _ => unreachable!(),
        };
        let octave = self.0 / 12;

        write!(f, "{}{}", name, octave)
    }
}

#[cfg(test)]
#[test]
fn to_str() {
    assert_eq!(Note(0).to_string(), "C0");
    assert_eq!(Note(37).to_string(), "Db3");
    assert_eq!(Note(76).to_string(), "E6");
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

pub struct NoteIter {
    note: Note,
    first: bool,
}

impl Iterator for NoteIter {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        // Returns the original note if this is the first iteration
        if self.first {
            self.first = false;
            Some(self.note)
        } else {
            self.note.0 += 1;
            Some(self.note)
        }
    }
}
