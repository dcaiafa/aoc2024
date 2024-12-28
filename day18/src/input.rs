#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V(pub i32, pub i32);

impl V {
  pub fn x(&self) -> i32 {
    self.0
  }
  pub fn y(&self) -> i32 {
    self.1
  }
}

impl std::ops::Add for V {
  type Output = V;

  fn add(self, rhs: Self) -> Self::Output {
    V(self.x() + rhs.x(), self.y() + rhs.y())
  }
}

impl std::ops::Sub for V {
  type Output = V;

  fn sub(self, rhs: Self) -> Self::Output {
    V(self.x() - rhs.x(), self.y() - rhs.y())
  }
}

pub struct Input {
  pub coords: Vec<V>,
}

impl Input {
  pub fn parse(s: &str) -> Input {
    // Input string is a list of coordinates:
    //  5,4
    //  4,2
    //  ...
    let coords: Vec<V> = s
      .lines()
      .map(|l| l.trim().split(",").map(|e| e.parse::<i32>()))
      .map(|mut e| match (e.next(), e.next(), e.next()) {
        (Some(Ok(x)), Some(Ok(y)), None) => Some(V(x, y)),
        _ => None,
      })
      .collect::<Option<Vec<V>>>()
      .unwrap();

    Input {coords}
  }
}
