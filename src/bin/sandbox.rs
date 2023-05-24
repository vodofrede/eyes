use eyes::parse;

fn main() {
    if let Some((a, b, c)) = eyes::try_parse!("1 2,3", "{} {},{}", u8, u8, u8) {
        assert!(a == 1 && b == 2 && c == 3);
    } else {
        unreachable!("This should not happen, as the pattern is matchable to the input");
    }

    let input = "1,2, 3";
    parse!(input, "{},{}, {}", i64, i64, i64);
}
