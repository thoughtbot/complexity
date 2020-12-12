use crate::scoring::ScoreVisitor;

pub struct Standard {
    acc: f32,
    line_length: usize,
    base_value: f32,
    previous_score: f32,
    multiplier: f32,
    minimum_line_length_of_file: usize,
}

impl Default for Standard {
    fn default() -> Self {
        Self {
            acc: 0.0,
            line_length: 0,
            base_value: 1.0,
            previous_score: 0.0,
            multiplier: 0.5,
            minimum_line_length_of_file: 2,
        }
    }
}

impl ScoreVisitor for Standard {
    fn visit_line_length(&mut self, length: usize) {
        self.line_length = length;
    }

    fn visit_first_line(&mut self, _: usize) {
        self.acc += self.base_value;
    }

    fn visit_indent(&mut self, _: usize) {
        self.acc += self.base_value;
        self.previous_score = self.base_value;
    }

    fn visit_same(&mut self, _: usize) {
        let new_score = self.previous_score * self.multiplier;

        self.acc += new_score;
        self.previous_score = new_score;
    }

    fn visit_dedent(&mut self, _: usize) {}

    fn score(&self) -> f32 {
        if self.line_length <= self.minimum_line_length_of_file {
            0.0
        } else {
            self.acc.powf(2.0) / self.line_length as f32
        }
    }
}
