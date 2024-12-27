mod input;
mod part1;
mod part2;
mod enc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2);
    let filename = &args[1];
    let filedata = std::fs::read_to_string(filename).unwrap();
    println!("part1={}", part1::eval(&filedata));
    //println!("part2={}", part2::eval(&filedata).unwrap());
    part2::eval(&filedata);
    enc::run();
}
