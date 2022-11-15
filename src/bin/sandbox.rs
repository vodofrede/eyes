fn main() {
    if let Some((a, b, c)) = eyes::try_parse!("1 2,3", "{} {},{}", u8, u8, u8) {
        assert!(a == 1 && b == 2 && c == 3);
    } else {
        unreachable!("This should not happen, as the pattern is matchable to the input");
    }
}
