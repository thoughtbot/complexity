pub struct ScoreConfig {
    pub base_value: f32,
    pub multiplier: f32,
    pub minimum_line_length_of_file: usize,
}

impl Default for ScoreConfig {
    fn default() -> Self {
        Self {
            base_value: 1.0,
            multiplier: 0.5,
            minimum_line_length_of_file: 2,
        }
    }
}

pub fn score(config: ScoreConfig, input: &[usize]) -> f32 {
    let line_length = input.len();

    if line_length < config.minimum_line_length_of_file {
        return 0.0;
    }

    let mut acc = 0.0;
    let mut previous_line_whitespace = (None, config.base_value);

    for current_whitespace in input {
        match previous_line_whitespace {
            (None, score) => {
                acc += score;
            }
            (Some(previous_whitespace), previous_score) => {
                if previous_whitespace < *current_whitespace {
                    previous_line_whitespace.1 = config.base_value;
                    acc += previous_line_whitespace.1;
                } else if previous_whitespace == *current_whitespace {
                    let new_score = previous_score * config.multiplier;
                    previous_line_whitespace.1 = new_score;
                    acc += previous_line_whitespace.1;
                }
            }
        }

        previous_line_whitespace.0 = Some(*current_whitespace);
    }

    acc.powf(2.0) / line_length as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn simple_case() {
        assert!(abs_diff_eq!(
            score(ScoreConfig::default(), &vec![0, 2, 0]),
            1.33333,
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
        assert!(abs_diff_eq!(
            score(ScoreConfig::default(), &lines),
            2.2578,
            epsilon = 0.0001
        ));
    }
}
