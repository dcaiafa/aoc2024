use crate::vm;

const PROG: &[i32] = &[2, 4, 1, 5, 7, 5, 0, 3, 4, 0, 1, 6, 5, 5, 3, 0];
// Notes:
//
//  2 4  bst  B = A % 8    // Use first 3 bits of A.
//  1 5  bxl  B = B ^ 5
//  7 5  cdv  C = A >> B   // Use 3 bits 0-7 bits from start.
//  0 3  adv  A = A >> 3   // Discard 3 bits from A.
//  4 0  bxc  B = B ^ C
//  1 6  bxl  B = B ^ 6
//  5 5  out  B % 8
//  3 0  jnz  loop
//
// Each output is the first 3 bits (B) xor'ed with 3 bits (C) that are 0-7 bits
// from the beginning of the input.

pub fn eval() -> i64 {
  let mut r: Vec<u8> = PROG.iter().map(|&e| e as u8).collect();
  r.reverse();
  encode(&r, 0, &[]).unwrap()
}

fn encode(vs: &[u8], a: i64, exp: &[u8]) -> Option<i64> {
  if vs.is_empty() {
    return Some(a);
  }

  let v = vs[0];
  let vs = &vs[1..];

  let mut new_exp = Vec::new();
  new_exp.push(v);
  new_exp.extend_from_slice(exp);

  let mut lowest = None;
  for b in 0..0b111 {
    let b2 = b ^ 5;
    let b3 = b2 ^ 6;
    let c = b3 ^ v;
    let x = (a << 3) | (b as i64);
    let x = x | ((c as i64) << b2);

    let res = vm::eval(PROG, x);
    if !res.is_empty() && res == new_exp {
      if let Some(a) = encode(vs, x, &new_exp) {
        if lowest.is_none() || a < lowest.unwrap() {
          lowest = Some(a);
        }
      }
    }
  }
  lowest
}
