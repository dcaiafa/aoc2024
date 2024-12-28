mod input;
mod part1;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  println!("part1={}", part1::eval(&input, 71, 71, 1024).unwrap());
}
