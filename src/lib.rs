pub struct Parser<'a> {
    captures: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, template: &'a str) -> Self {
        let patterns = template
            .split("{}")
            .filter(|pat| pat != &"")
            .collect::<Vec<_>>();
        let mut captures = vec![input];

        for (i, pat) in patterns.iter().enumerate() {
            let last = captures.pop().unwrap();
            let (mut left, mut right) = last.split_once(pat).unwrap();

            // if the right side of the split doesn't contain the pattern,
            // we don't have to check if we can expand the match
            if right.contains(pat) {
                let mut pattern_index = right.find(pat).unwrap() + left.len();
                let next_pattern_index = right.find(patterns[i + 1]).unwrap() + left.len();

                while next_pattern_index > pattern_index {
                    let (left_side, _) = input.split_at(pattern_index + 1);
                    left = left_side;
                    let (_, right_side) = input.split_at(pattern_index + 1 + pat.len());
                    right = right_side;

                    pattern_index = right.find(pat).unwrap_or(input.len()) + left.len();
                }
            }

            if !left.is_empty() {
                captures.push(left);
            }
            captures.push(right);
        }

        Self { captures }
    }

    pub fn captures(&self) -> Vec<&'a str> {
        self.captures.to_owned()
    }
}

#[macro_export]
macro_rules! parse {
    ($input: expr, $pattern: tt, $($type:ty),*) => {
        {
            let parser = $crate::Parser::new($input, $pattern);
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
            let parser = $crate::Parser::new($input, $pattern);
            let captures = parser.captures();
            let mut iter = captures.iter();

            ($(iter.next().unwrap().parse::<$type>()),*)
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

        let (op, x1, y1, x2, y2) = try_parse!(input, template, String, usize, usize, usize, usize);

        println!("op: {:?}", op);
        println!("p1: {:?}", (&x1, &y1));
        println!("p2: {:?}", (&x2, &y2));

        assert_eq!(
            (op.unwrap().as_str(), x1, y1, x2, y2),
            ("turn off", Ok(660), Ok(55), Ok(986), Ok(197))
        );
    }
}
