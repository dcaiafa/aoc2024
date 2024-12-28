use crate::vm;

const PROG: &[i32] = &[2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0];

pub fn run() {
  let mut r: Vec<u8> = PROG.iter().map(|&e|e as u8).collect();
  r.reverse();
  println!("{}", encode(&r, 0, &[]).unwrap());
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

  for b in 0..0b111 {
    let b2 = b ^ 5;
    let b3 = b2 ^ 6;
    let c = b3 ^ v;
    let x = (a << 3) | (b as i64);
    let x = x | ((c as i64) << b2);

    let res = vm::eval(PROG, x);
    println!("res={:?}", res);
    if !res.is_empty() && res == new_exp {
      if let Some(a) = encode(vs, x, &new_exp) {
        return Some(a);
      }
    }
  }

  None
}
