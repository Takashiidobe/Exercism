#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.clone().into_iter().last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.clone().into_iter().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut copy = self.scores.clone();
        copy.sort();
        copy.into_iter().rev().take(3).collect()
    }
}
