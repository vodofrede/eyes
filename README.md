# Eyes

## A simpler way to parse using human-readable templates

Eyes was made for the primary purpose of parsing challenge inputs for [Advent of Code](https://adventofcode.com) challenges.

It currently provides limited functionality, but more options may be added provided they are useful additions for parsing slightly more complicated formats.

Eyes does not have any dependencies, as I wanted to keep it simple to and lightweight in design. Good performance is not guaranteed, as the library isn't well tested yet.

I was told this functionality is similar to `scanf` from C.

### Examples:

```rust
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

```

**Eyes** will try to expand its captures, so that the following example also works as expected:

```rust
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
```
