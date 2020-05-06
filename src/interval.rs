pub(crate) const SEMITONE: Interval = Interval::new(1);
pub(crate) const TONE: Interval = Interval::new(2);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Interval {
    pub(crate) semitones: usize,
}

impl Interval {
    pub(crate) const fn new(semitones: usize) -> Self {
        Self { semitones }
    }
}
