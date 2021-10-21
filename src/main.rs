use eyes::*;

fn main() {
    let input = "#loll1 @ 338,764: 20x24";
    let pattern = "#{} @ {},{}: {}x{}";

    println!("input: {}", input);
    println!("pattern: {}", pattern);

    let (id, x, y, w, h) = parse!(input, pattern, String, f64, f32, usize, isize);

    println!("id: {:?}", id);
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("w: {:?}", w);
    println!("h: {:?}", h);
}
