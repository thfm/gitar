#[derive(Debug, Eq, PartialEq)]
pub struct Interval {
    pub(crate) semitones: usize,
}

impl Interval {
    pub(crate) fn new(semitones: usize) -> Self {
        Self { semitones }
    }
}
