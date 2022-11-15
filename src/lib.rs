#![allow(clippy::needless_question_mark)]
#![warn(clippy::all, clippy::cargo)]
#![deny(missing_docs, unsafe_code)]
#![doc = include_str!("../README.md")]

/// A list of captures, created by calling [`Captures::new()`] with the input and template strings.
///
/// An easier way to use this struct is with the [`eyes::parse`] and [`eyes::try_parse`] macros, which allow for automatic type conversion of captures.
pub struct Captures<'a> {
    captures: Vec<&'a str>,
}

impl<'a> Captures<'a> {
    /// Create a new list of captures from input and template strings.
    ///
    /// The input and template strings must live as long as the list of captures itself, as the captures list borrows from them.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use eyes::Captures;
    /// let captures = Captures::new("haystack|needle|haystack", "haystack|{}|haystack");
    /// assert_eq!(captures.unwrap().to_inner()[0], "needle");
    /// ```
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

    /// Get the internal representation of the captures as an owned value, which allow usage of standard [`Vec`] methods.
    pub fn to_inner(&self) -> Vec<&'a str> {
        self.captures.to_owned()
    }

    /// Get the internal representation of the captures as a reference, which allow usage of standard [`Vec`] methods.
    pub fn as_inner(&self) -> &Vec<&'a str> {
        &self.captures
    }
}

/// Parse an input and template, and convert the captures to the specified types.
/// This version returns an option, indicating whether the input matched the template by returning None in the negative case.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # #[macro_use] extern crate eyes;
/// if let Some((a, b, c)) = eyes::try_parse!("1 2,3", "{} {},{}", u8, u8, u8) {
///     assert!(a == 1 && b == 2 && c == 3);
/// } else {
///     unreachable!("This should not happen, as the pattern is matchable to the input");
/// }
/// ```
#[macro_export]
macro_rules! try_parse {
    ($input: expr, $pattern: tt, $($type:ty),*) => {
        {
            #[allow(clippy::all)]
            $crate::Captures::new($input, $pattern)
                .and_then(|c| {
                    let mut iter = c.as_inner().iter();

                    Some(($(iter.next()?.parse::<$type>().ok()?),*))
                })
        }
    };
}

/// Parse an input and template, and convert the captures to the specified types.
///
/// ## Panics
///
/// This macro unwraps the parse result, causing a panic in any of the following cases:
/// - The template does not match the input.
/// - The capture could not be converted to the specified type.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # #[macro_use] extern crate eyes;
/// # fn main() {
/// let (a, b, c) = eyes::parse!("1 2,3", "{} {},{}", u8, u8, u8);
/// assert!(a == 1 && b == 2 && c == 3);
/// # }
/// ```
#[macro_export]
macro_rules! parse {
    ($input: expr, $pattern: tt, $($type:ty),*) => {
        try_parse!($input, $pattern, $($type),*).unwrap()
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
            (op.as_str(), x1, y1, x2, y2),
            ("turn off", 660, 55, 986, 197)
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

        assert_eq!((a, b, c), (775, 785, 361));
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

        assert_eq!((a, b), (1, 1))
    }

    #[test]
    fn match_whole_input() {
        let input = "3240955";
        let template = "{}";

        println!("input: '{}'", input);
        println!("pattern: '{}'", template);

        let a = try_parse!(input, template, usize).unwrap();

        println!("a: {:?}", a);

        assert_eq!(a, 3240955)
    }
}
