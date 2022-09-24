pub fn parse_file(input: &str) -> Vec<usize> {
    if input.len() == 0 {
        vec![]
    } else {
        let mut result = vec![];
        let mut whitespace = 0;
        let mut space = true;

        for c in input.chars() {
            if (c == ' ' || c == '\t') && space {
                whitespace += 1;
            } else if c == '\n' {
                if !space {
                    result.push(whitespace);
                }
                whitespace = 0;
                space = true;
            } else {
                space = false;
            }
        }

        if !space {
            result.push(whitespace);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_whitespace_correctly() {
        assert_eq!(parse_file("    def full_name"), vec![4]);
        assert_eq!(parse_file("class Person; end"), vec![0]);
        assert!(parse_file("").is_empty());
    }

    #[test]
    fn parses_a_file() {
        assert_eq!(
            parse_file("\n\nclass Person;end\n\nclass Dog;end\n\n"),
            vec![0, 0]
        )
    }
}
