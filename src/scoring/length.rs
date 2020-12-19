use crate::scoring::ScoreVisitor;

pub struct Length {
    line_length: usize,
}

impl Default for Length {
    fn default() -> Self {
        Self { line_length: 0 }
    }
}

impl ScoreVisitor for Length {
    fn visit_line_length(&mut self, length: usize) {
        self.line_length = length;
    }

    fn visit_first_line(&mut self, _: usize) {}

    fn visit_indent(&mut self, _: usize) {}

    fn visit_same(&mut self, _: usize) {}

    fn visit_dedent(&mut self, _: usize) {}

    fn score(&self) -> f32 {
        self.line_length as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::{score, ScoreVisitor};
    use approx::*;

    #[test]
    fn length_only_uses_file_length() {
        let mut scorer: Box<dyn ScoreVisitor> = Box::new(Length::default());

        assert!(abs_diff_eq!(
            score(&mut scorer, &vec![0, 2, 4, 6, 8, 10, 8, 6, 4, 2, 0]),
            11.0,
            epsilon = 0.0001
        ));
    }
}
