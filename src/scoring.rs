mod length;
mod standard;

pub use length::Length;
pub use standard::Standard;

pub trait ScoreVisitor {
    fn visit_line_length(&mut self, length: usize);
    fn visit_first_line(&mut self, depth: usize);
    fn visit_indent(&mut self, depth: usize);
    fn visit_same(&mut self, depth: usize);
    fn visit_dedent(&mut self, depth: usize);
    fn score(&self) -> f32;
}

pub fn score(visitor: &mut Box<dyn ScoreVisitor>, input: &[usize]) -> f32 {
    let line_length = input.len();

    visitor.visit_line_length(line_length);
    let mut previous_line_whitespace = None;

    for current_whitespace in input {
        match previous_line_whitespace {
            None => {
                visitor.visit_first_line(*current_whitespace);
            }
            Some(previous_whitespace) => {
                if previous_whitespace < *current_whitespace {
                    visitor.visit_indent(*current_whitespace);
                } else if previous_whitespace == *current_whitespace {
                    visitor.visit_same(*current_whitespace);
                } else {
                    visitor.visit_dedent(*current_whitespace);
                }
            }
        }

        previous_line_whitespace = Some(*current_whitespace);
    }

    visitor.score()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::Standard;
    use approx::*;

    #[test]
    fn simple_case() {
        let mut scorer: Box<dyn ScoreVisitor> = Box::new(Standard::default());

        assert!(abs_diff_eq!(
            score(&mut scorer, &vec![0, 2, 0]),
            1.33333,
            epsilon = 0.0001
        ));
    }

    #[test]
    fn empty_file() {
        let mut scorer: Box<dyn ScoreVisitor> = Box::new(Standard::default());

        assert!(abs_diff_eq!(
            score(&mut scorer, &vec![]),
            0.0,
            epsilon = 0.0001
        ));
    }

    #[test]
    fn allows_separate_configurations() {
        struct Custom {
            acc: f32,
            line_length: usize,
        }

        impl ScoreVisitor for Custom {
            fn visit_line_length(&mut self, length: usize) {
                self.line_length = length;
            }

            fn visit_first_line(&mut self, _: usize) {
                self.acc += 10.0;
            }

            fn visit_indent(&mut self, _: usize) {
                self.acc += 2.0;
            }

            fn visit_same(&mut self, _: usize) {
                self.acc += 1.0;
            }

            fn visit_dedent(&mut self, _: usize) {
                self.acc -= 1.0;
            }

            fn score(&self) -> f32 {
                self.acc / self.line_length as f32
            }
        }

        let mut visitor: Box<dyn ScoreVisitor> = Box::new(Custom {
            acc: 5.0,
            line_length: 0,
        });
        assert!(abs_diff_eq!(
            score(&mut visitor, &vec![0, 2, 4, 4, 4, 2, 2, 2, 2, 0]),
            2.2,
            epsilon = 0.0001
        ));
    }

    #[test]
    fn complex_case() {
        let lines = vec![
            0, // base score
            2, // + base score
            2, // + base score * multiplier
            2, // + (base score * multiplier ^ 2)
            4, // + base_score
            4, // + base_score * multiplier
            2, // + 0
            0, // + 0
        ];

        let mut scorer: Box<dyn ScoreVisitor> = Box::new(Standard::default());
        assert!(abs_diff_eq!(
            score(&mut scorer, &lines),
            2.2578,
            epsilon = 0.0001
        ));
    }
}
