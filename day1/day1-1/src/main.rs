use anyhow::{bail, Error};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::{env, io};

fn main() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    bail!("Expect input filename");
  }
  let input_filename = &args[1];

  let file = File::open(input_filename)?;
  let mut reader = io::BufReader::new(file);

  let mut list1 = Vec::new();
  let mut list2 = Vec::new();

  let mut raw_line = String::new();
  while reader.read_line(&mut raw_line)? > 0 {
    let line = raw_line.trim();
    let values: Vec<&str> = line.split_whitespace().collect();
    if values.len() != 2 {
      bail!("Input line does not contain two values");
    }

    let v1: i32 = values[0].parse()?;
    list1.push(v1);
    let v2: i32 = values[1].parse()?;
    list2.push(v2);

    raw_line.clear();
  }

  list1.sort();
  list2.sort();

  let mut dist_sum = 0;
  for i in 0..list1.len() {
    let v1 = list1[i];
    let v2 = list2[i];
    let dist = (v1-v2).abs();
    dist_sum += dist;
  }

  dbg!(dist_sum);

  let mut m: HashMap<i32,i32> = HashMap::new();
  for v in &list1 {
    m.insert(*v, 0);
  }
  for v in list2 {
    if let Some(c) = m.get(&v) {
      m.insert(v, c+1);
    }
  }

  let mut sim_score = 0;
  for v in list1 {
    sim_score += v * m.get(&v).unwrap_or(&0);
  }

  dbg!(sim_score);

  Ok(())
}
