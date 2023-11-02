#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl HighScores<'_> {
    pub fn new(scores: &[u32]) -> HighScores {
        HighScores(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.0.to_vec();
        scores.sort_unstable_by(|a, b| b.cmp(a));
        scores.truncate(3);
        scores
    }
}
