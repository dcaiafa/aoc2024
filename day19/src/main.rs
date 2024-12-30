mod towel_trie;
mod part1;
mod part2;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  println!("part1={}", part1::eval(&input));
  println!("part2={}", part2::eval(&input));
}
