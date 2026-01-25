#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    sorted_scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.iter().map(|s| *s).collect::<Vec<u32>>(),
            sorted_scores:  { 
                let mut s = scores.iter().map(|s| *s).collect::<Vec<u32>>();
                s.sort();
                s
            }
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        Some(*(self.scores().last()?))
    }

    pub fn personal_best(&self) -> Option<u32> {
        Some(*(self.sorted_scores.last()?))
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let min = if self.scores.len() < 3 {0} else {self.scores.len()-3};
        let mut m: Vec<u32> = self.sorted_scores[min..self.scores.len()].iter().map(|f| *f).collect();
        m.reverse();
        m
    }
}
