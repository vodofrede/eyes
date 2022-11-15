# eyes

A simpler way to parse using human-readable templates.

The crate's primary purpose is parsing challenge inputs for [Advent of Code](https://adventofcode.com) challenges. It currently provides limited functionality, but more options may be added provided they are useful additions for parsing slightly more complicated formats.

This crate does not have any dependencies, as I wanted to keep it simple to and lightweight in design.

## Syntax

The only special characters in templates are curly brackets ('{}'). These act as stand-ins for where the extracted values are in the input strings.

## Examples:

```rust
#[macro_use] extern crate eyes; // not normally necessary
fn main() {
    let input = "#lol @ 338,7643: 20.2x24.5";
    let template = "#{} @ {},{}: {}x{}";

    let (id, x, y, w, h) = eyes::parse!(input, template, String, isize, isize, f64, f64);

    assert_eq!((id.as_str(), x, y, w, h), ("lol", 338, 7643, 20.2, 24.5));
}
```

**eyes** will try to expand capture groups, so that the following example also works as expected:

```rust
#[macro_use] extern crate eyes; // not normally necessary
fn main() {
    let input = "turn off 660,55 through 986,197";
    let template = "{} {},{} through {},{}";

    let (op, x1, y1, x2, y2) = eyes::parse!(input, template, String, usize, usize, usize, usize);

    assert_eq!(
        (op.as_str(), x1, y1, x2, y2),
        ("turn off", 660, 55, 986, 197)
    );
}
```

Notice that "turn off" is captured correctly, even though it contains a space.
