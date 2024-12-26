mod input;
mod part1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2);
    let filename = &args[1];
    let filedata = std::fs::read_to_string(filename).unwrap();
    println!("part1={}", part1::eval(&filedata));
}
