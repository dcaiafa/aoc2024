mod part1;
mod part2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected <filename>");
    }
    let filename = &args[1];
    let filedata = std::fs::read_to_string(filename).unwrap();
    println!("part1={}", part1::eval(&filedata));
    println!("part2={}", part2::eval(&filedata));
}
