#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct NameGenerator {
    counter: usize,
}

impl NameGenerator {
    pub const fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Iterator for NameGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.counter % 26;

        let mut alphabet = 'a'..='z';

        match alphabet.advance_by(offset) {
            Ok(()) => (),
            Err(_) => unreachable!(),
        }

        let letter: char = alphabet
            .next()
            .expect("Since offset is mod 26, we shouldn't ever hit the end of the range");

        let cycle = self.counter / 26;

        self.counter = self.counter.saturating_add(1);

        if cycle == 0 {
            Some(letter.to_string())
        } else {
            Some(format!("{letter}{cycle}"))
        }
    }
}
