
pub fn run() {
  println!("encode:");
  encode(0, 5);
}

fn encode(e: i64, v:i64) -> Option<i64> {
  for b in 0..0b11 {
    let x = (e << 3) | b;
    let b2 = b^5;
    //let c = 0;
    let skip = b2 as u32;
    let c = x >> skip & 0b111;
    let b3 = b2 ^ c;
    let b4 = b3 ^ 6;
    if b4 == v {
      return Some((e << 3) | b);
    }
  }
  None
}
