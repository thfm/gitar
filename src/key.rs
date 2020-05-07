use crate::{Interval, Note, SEMITONE, TONE};
use arrayvec::ArrayVec;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display as EnumDisplay, EnumIter, EnumString};

#[derive(Copy, Clone)]
pub struct Key {
    notes: [Note; 7],
    mode: Mode,
}

impl Key {
    pub fn new(root_note: Note, mode: Mode) -> Self {
        let mut notes = ArrayVec::<[Note; 7]>::new();
        notes.push(root_note);
        for (i, interval) in mode.into_iter().enumerate() {
            let previous_note = notes[i];
            notes.push(previous_note + interval);
        }

        Self {
            notes: notes.into_inner().unwrap(),
            mode,
        }
    }

    fn notes_disregarding_octave(mut self) -> [Note; 7] {
        for note in &mut self.notes {
            *note = note.disregard_octave();
        }
        self.notes
    }
}

#[cfg(test)]
#[test]
fn construction() {
    assert_eq!(
        Key::new(Note::new(0), Mode::Ionian).notes,
        [
            Note::new(0),
            Note::new(2),
            Note::new(4),
            Note::new(5),
            Note::new(7),
            Note::new(9),
            Note::new(11)
        ]
    );

    assert_eq!(
        Key::new(Note::new(3), Mode::Aeolian).notes,
        [
            Note::new(3),
            Note::new(5),
            Note::new(6),
            Note::new(8),
            Note::new(10),
            Note::new(11),
            Note::new(13)
        ]
    );
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let num_notes = self.notes.len();
            for (i, note) in self.notes.iter().enumerate() {
                write!(f, "{:#}", note)?;
                // Only prints a space if this is not the final note
                // (to avoid having a trailing space)
                if i != num_notes - 1 {
                    f.write_str(" ")?;
                }
            }

            Ok(())
        } else {
            write!(f, "{:#} {}", self.notes[0], self.mode)
        }
    }
}

#[cfg(test)]
#[test]
fn display() {
    // D Dorian
    assert_eq!(
        Key::new(Note::new(2), Mode::Dorian).to_string(),
        "D E F G A B C"
    );

    // Ab Mixolydian
    assert_eq!(
        Key::new(Note::new(8), Mode::Mixolydian).to_string(),
        "Ab Bb C D Eb F G"
    );
}

pub fn guess_key(notes: Vec<Note>, root_note: Option<Note>) -> Vec<Key> {
    let key_filter = |root_note, key_candidates: &mut Vec<_>| {
        for mode in Mode::iter() {
            let key = Key::new(root_note, mode);
            if notes
                .iter()
                .all(|note| key.notes_disregarding_octave().contains(note))
            {
                key_candidates.push(key);
            }
        }
    };

    let mut key_candidates = Vec::new();
    if let Some(root_note) = root_note {
        key_filter(root_note, &mut key_candidates);
    } else {
        for root_note in Note::new(0).into_iter().take(12) {
            key_filter(root_note, &mut key_candidates);
        }
    }
    key_candidates
}

#[derive(EnumString, EnumIter, EnumDisplay, Debug, Copy, Clone)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Mixolydian,
    Lydian,
    Aeolian,
    Locrian,
}

impl IntoIterator for Mode {
    type Item = Interval;
    type IntoIter = ModeIntervalIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            intervals: match self {
                Mode::Ionian => [TONE, TONE, SEMITONE, TONE, TONE, TONE],
                Mode::Dorian => [TONE, SEMITONE, TONE, TONE, TONE, SEMITONE],
                Mode::Phrygian => [SEMITONE, TONE, TONE, TONE, SEMITONE, TONE],
                Mode::Mixolydian => [TONE, TONE, TONE, SEMITONE, TONE, TONE],
                Mode::Lydian => [TONE, TONE, SEMITONE, TONE, TONE, SEMITONE],
                Mode::Aeolian => [TONE, SEMITONE, TONE, TONE, SEMITONE, TONE],
                Mode::Locrian => [SEMITONE, TONE, TONE, SEMITONE, TONE, TONE],
            },
            index: 0,
        }
    }
}

pub struct ModeIntervalIter {
    intervals: [Interval; 6],
    index: usize,
}

impl Iterator for ModeIntervalIter {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        let interval = self.intervals.get(self.index).copied();
        self.index += 1;
        interval
    }
}
