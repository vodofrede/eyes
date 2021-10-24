use eyes::*;

fn main() {
    let input = "turn off 660,55 through 986,197";
    let template = "{} {},{} through {},{}";

    println!("input: '{}'", input);
    println!("pattern: '{}'", template);

    let (op, x1, y1, x2, y2) = try_parse!(input, template, String, usize, usize, usize, usize);

    println!("op: {:?}", op);
    println!("p1: {:?}", (x1, y1));
    println!("p2: {:?}", (x2, y2));
}
