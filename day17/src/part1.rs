use crate::input::Input;

struct VM {
  a: i32,
  b: i32,
  c: i32,
  i: usize,
  prog: Vec<i32>,
  out: Vec<i32>,
}

impl VM {
  pub fn new(input: &Input) -> VM {
    VM {
      a: input.a,
      b: input.b,
      c: input.c,
      i: 0,
      prog: input.prog.clone(),
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
        0 => { // adv
          let d = 2_i32.pow(self.combo(operand) as u32);
          self.a = self.a / d;
        },
        1 => { // bxl
            self.b = self.b ^ operand;
        },
        2 => { // bst
            self.b = self.combo(operand) % 8;
        },
        3 => { // jnz
            if self.a != 0 {
                self.i = operand as usize;
            }
        },
        4 => { // bxc
            self.b = self.b ^ self.c;
        },
        5 => { // out
            self.out.push(self.combo(operand) % 8);
        },
        6 => { // bdv
          let d = 2_i32.pow(self.combo(operand) as u32);
          self.b = self.a / d;
        },
        7 => { // cdv
          let d = 2_i32.pow(self.combo(operand) as u32);
          self.c = self.a / d;
        },
        _ => panic!()
      }
    }
  }

  fn combo(&self, operand: i32) -> i32 {
    match operand {
      0..=3 => operand,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("Invalid operand {}", operand),
    }
  }
}

pub fn eval(s: &str) -> String {
    let input = Input::parse(s);
    let mut vm = VM::new(&input);
    vm.run();
    vm.out.iter().map(|v|v.to_string()).collect::<Vec<String>>().join(",")
}
