use eyes::*;

fn main() {
    let input = "#1 @ 338,764: 20x24";
    let pattern = "#{} @ {},{}: {}x{}";

    println!("input: {}", input);
    println!("pattern: {}", pattern);

    let (id, x, y, w, h) = parse!(input, pattern, usize, isize, isize, usize, usize);

    println!("id: {:?}", id);
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("w: {:?}", w);
    println!("h: {:?}", h);
}
