use crate::input::Input;

struct VM {
  a: i64,
  b: i64,
  c: i64,
  i: usize,
  prog: Vec<i32>,
  match_count: usize,
  max_match: usize,
}

impl VM {
  pub fn new(input: &Input) -> VM {
    VM {
      a: 0,
      b: 0,
      c: 0,
      i: 0,
      prog: input.prog.clone(),
      match_count: 0,
      max_match: 0,
    }
  }

  pub fn run(&mut self, a: i64) -> bool {
    println!("run {}", a);
    self.a = a;
    self.b = 0;
    self.c = 0;
    self.i = 0;
    self.match_count = 0;

    loop {
      println!("{} a: {}, b: {}, c: {}", self.match_count, self.a, self.b, self.c);
      if self.i + 1 >= self.prog.len() {
        return self.match_count == self.prog.len();
      }
      let op = self.prog[self.i];
      let operand = self.prog[self.i + 1];
      self.i += 2;

      match op {
        0 => {
          // adv
          print!("adv ");
          let d = 2_i64.pow(self.combo(operand) as u32);
          self.a = self.a / d;
        }
        1 => {
          // bxl
          print!("bxl ");
          self.b = self.b ^ (operand as i64);
        }
        2 => {
          // bst
          print!("bst ");
          self.b = self.combo(operand) % 8;
        }
        3 => {
          // jnz
          print!("jnz ");
          if self.a != 0 {
            self.i = operand as usize;
          }
        }
        4 => {
          // bxc
          print!("bxc ");
          self.b = self.b ^ self.c;
        }
        5 => {
          // out
          if self.match_count == self.prog.len() {
            return false;
          }
          let out = self.combo(operand) % 8;
          println!("out: {}", out);
          if self.prog[self.match_count] != (out as i32) {
            return false;
          }
          self.match_count += 1;
          if self.match_count >= self.max_match {
            self.max_match = self.match_count;
          }
        }
        6 => {
          // bdv
          print!("bdv ");
          let d = 2_i64.pow(self.combo(operand) as u32);
          self.b = self.a / d;
        }
        7 => {
          // cdv
          println!("discard {}", self.combo(operand));
          print!("cdv ");
          let d = 2_i64.pow(self.combo(operand) as u32);
          self.c = self.a / d;
        }
        _ => panic!(),
      }
    }
  }

  fn combo(&self, operand: i32) -> i64 {
    match operand {
      0..=3 => operand as i64,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Invalid operand {}", operand),
    }
  }
}

pub fn eval(s: &str) -> Option<i64> {
  let input = Input::parse(s);
  let mut vm = VM::new(&input);

  let nums = [
    4491610522, 4491611905, 4491611913, 4491637513, 4491676058, 4491677441, 4491677449, 4541942170,
    4541943553, 4541943561, 4541969161,
  ];
  for n in nums {
    vm.run(n);
  }

  /*
  for i in 0..i64::MAX {
      if vm.run(i) {
          return Some(i);
      }
  }
  */

  None
}
