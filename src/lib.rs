pub struct Parser<'a> {
    captures: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, template: &'a str) -> Option<Self> {
        // find all patterns in the template
        let patterns = template
            .split("{}")
            .filter(|pat| pat != &"")
            .collect::<Vec<_>>();

        let mut captures = vec![input];

        // recursively split the input into left and right parts, where left is a match and right is processed next iteration
        for (i, pat) in patterns.iter().enumerate() {
            let last = captures.pop()?;

            // we need to match all whitespace, and not just a specific type of whitespace
            let (mut left, mut right) = last.split_once(pat)?;

            // check if pattern is pure whitespace
            if pat.chars().all(|c| c.is_whitespace()) {
                // if it is, we want to remove it so we can match arbitrary whitespace
                right = right.trim_start_matches(|c: char| c.is_whitespace());
            }

            // if the right side of the split doesn't contain the pattern,
            // we don't have to check if we can expand the match
            if right.contains(pat) {
                // here we check if the pattern can be expanded without interfering with other patterns
                let mut pattern_index = right.find(pat)? + left.len();
                let next_pattern_index = right
                    .find(patterns.get(i + 1).unwrap_or(&""))
                    .unwrap_or(pat.len())
                    + left.len();

                while next_pattern_index > pattern_index {
                    // we split two times, so we don't get the pattern in any of the splits
                    let (left_side, _) = input.split_at(pattern_index + 1);
                    left = left_side;
                    let (_, right_side) = input.split_at(pattern_index + 1 + pat.len());
                    right = right_side;

                    pattern_index = right.find(pat).unwrap_or(input.len()) + left.len();
                }
            }

            // if the first chars aren't a placeholder, the first split will be empty. we don't want to add this to the list of captures
            if !left.is_empty() {
                captures.push(left);
            }
            captures.push(right);
        }

        Some(Self { captures })
    }

    pub fn captures(&self) -> Vec<&'a str> {
        self.captures.to_owned()
    }
}

#[macro_export]
macro_rules! parse {
    ($input: expr, $pattern: tt, $($type:ty),*) => {
        {
            let parser = $crate::Parser::new($input, $pattern).unwrap();
            let captures = parser.captures();
            let mut iter = captures.iter();

            ($(iter.next().unwrap().parse::<$type>().unwrap()),*)
        }
    };
}

#[macro_export]
macro_rules! try_parse {
    ($input: expr, $pattern: tt, $($type:ty),*) => {
        {
            if let Some(parser) = $crate::Parser::new($input, $pattern) {
                let captures = parser.captures();
                let mut iter = captures.iter();

                Some(($(iter.next().unwrap().parse::<$type>()),*))
            } else {
                None
            }


        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        // Test where the patterns in the template are all different
        let input = "#lol @ 338,7643: 20.2x24.5";
        let template = "#{} @ {},{}: {}x{}";

        println!("input: '{}'", input);
        println!("pattern: '{}'", template);

        let (id, x, y, w, h) = parse!(input, template, String, isize, isize, f64, f64);

        println!("id: {:?}", id);
        println!("x: {:?}", x);
        println!("y: {:?}", y);
        println!("w: {:?}", w);
        println!("h: {:?}", h);

        assert_eq!((id.as_str(), x, y, w, h), ("lol", 338, 7643, 20.2, 24.5));
    }

    #[test]
    fn tries_to_expand_correctly() {
        let input = "turn off 660,55 through 986,197";
        let template = "{} {},{} through {},{}";

        println!("input: '{}'", input);
        println!("pattern: '{}'", template);

        let (op, x1, y1, x2, y2) =
            try_parse!(input, template, String, usize, usize, usize, usize).unwrap();

        println!("op: {:?}", op);
        println!("p1: {:?}", (&x1, &y1));
        println!("p2: {:?}", (&x2, &y2));

        assert_eq!(
            (op.unwrap().as_str(), x1, y1, x2, y2),
            ("turn off", Ok(660), Ok(55), Ok(986), Ok(197))
        );
    }

    #[test]
    fn works_with_different_length_whitespace() {
        let input = "  775  785    361";
        let template = " {} {} {}";

        println!("input: '{}'", input);
        println!("pattern: '{}'", template);

        let (a, b, c) = try_parse!(input, template, usize, usize, usize).unwrap();

        println!("a: {:?}", a);
        println!("b: {:?}", b);
        println!("c: {:?}", c);

        assert_eq!((a, b, c), (Ok(775), Ok(785), Ok(361)));
    }

    #[test]
    fn short_input() {
        let input = "1x1";
        let template = "{}x{}";

        println!("input: '{}'", input);
        println!("pattern: '{}'", template);

        let (a, b) = try_parse!(input, template, usize, usize).unwrap();

        println!("a: {:?}", a);
        println!("b: {:?}", b);

        assert_eq!((a, b), (Ok(1), Ok(1)))
    }

    #[test]
    fn match_whole_input() {
        let input = "3240955";
        let template = "{}";

        println!("input: '{}'", input);
        println!("pattern: '{}'", template);

        let a = try_parse!(input, template, usize).unwrap();

        println!("a: {:?}", a);

        assert_eq!(a, Ok(3240955))
    }
}
