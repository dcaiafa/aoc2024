use crate::input::Input;

struct VM {
  a: i64,
  b: i64,
  c: i64,
  i: usize,
  prog: Vec<i32>,
  out: Vec<i32>,
}

impl VM {
  pub fn new(prog: &[i32], a: i64) -> VM {
    VM {
      a,
      b: 0,
      c: 0,
      i: 0,
      prog: prog.into(),
      out: Vec::new(),
    }
  }

  pub fn run(&mut self) {
    loop {
      if self.i + 1 >= self.prog.len() {
        break;
      }
      let op = self.prog[self.i];
      let operand = self.prog[self.i + 1];
      self.i += 2;

      match op {
        0 => {
          // adv
          let d = 2_i64.pow(self.combo(operand) as u32);
          self.a = self.a / d;
        }
        1 => {
          // bxl
          self.b = self.b ^ (operand as i64);
        }
        2 => {
          // bst
          self.b = self.combo(operand) % 8;
        }
        3 => {
          // jnz
          if self.a != 0 {
            self.i = operand as usize;
          }
        }
        4 => {
          // bxc
          self.b = self.b ^ self.c;
        }
        5 => {
          // out
          self.out.push((self.combo(operand) % 8) as i32);
        }
        6 => {
          // bdv
          let d = 2_i64.pow(self.combo(operand) as u32);
          self.b = self.a / d;
        }
        7 => {
          // cdv
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

pub fn eval(prog: &[i32], a: i64) -> Vec<u8> {
  let mut vm = VM::new(prog, a);
  vm.run();
  vm.out.into_iter().map(|e|e as u8).collect()
}
