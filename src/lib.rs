pub struct Parser<'a> {
    _input: &'a str,
    _pattern: &'a str,
    captures: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, pattern: &'a str) -> Self {
        let splits = pattern
            .split("{}")
            .filter(|pat| pat != &"")
            .collect::<Vec<_>>();
        let mut captures = vec![input];

        println!("splits: {:?}", splits);

        for pat in splits {
            captures = captures
                .iter()
                .map(|sub| sub.split(pat))
                .flatten()
                .collect();
        }

        Self {
            _input: input,
            _pattern: pattern,
            captures: captures[1..].to_vec(),
        }
    }

    pub fn captures(&self) -> Vec<&'a str> {
        self.captures.to_owned()
    }
}

#[macro_export]
macro_rules! parse {
    ($input: expr, $pattern: tt, $($type:ty),*) => {
        {
            let mut parser = eyes::Parser::new($input, $pattern);
            let mut captures = parser.captures();
            captures.reverse();
            println!("caps: {:?}", captures);

            (
            $({
                    captures.pop().unwrap().parse::<$type>().unwrap()
            },)*
            )
        }
    };
}

#[macro_export]
macro_rules! try_parse {
    ($input: expr, $pattern: tt, $($type:ty),*) => {
        {
            let mut parser = eyes::Parser::new($input, $pattern);
            let mut captures = parser.captures();
            captures.reverse();
            println!("caps: {:?}", captures);

            (
            $({
                    captures.pop().unwrap().parse::<$type>()
            },)*
            )
        }
    };
}
