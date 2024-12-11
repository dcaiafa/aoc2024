use std::{
  fs::File,
  io::{self, BufRead},
};

use anyhow::{bail, Error};

fn main() -> Result<(), Error> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    bail!("Expected input filename");
  }

  let input_filename: &str = &args[1];
  let file = File::open(input_filename)?;
  let mut reader = io::BufReader::new(file);

  let mut safe_count = 0;
  let mut safe_count2 = 0;

  let mut raw_line = String::new();
  while reader.read_line(&mut raw_line)? > 0 {
    let report: Vec<i32> = raw_line
      .trim()
      .split_whitespace()
      .map(|v| v.parse())
      .collect::<Result<_, _>>()?;
    raw_line.clear();

    if is_report_valid(report.iter().cloned()) {
      safe_count += 1;
      safe_count2 += 1;
    } else {
      let mut is_valid = false;
      for i in 0..report.len() {
        let partial = report[0..i].iter().chain(&report[i+1..]);
        if is_report_valid(partial.cloned()) {
          is_valid = true;
          break;
        }
      }
      if is_valid {
        safe_count2 += 1;
      }
    }
  }

  dbg!(safe_count);
  dbg!(safe_count2);

  Ok(())
}

fn is_report_valid<I>(report: I) -> bool
where
    I: IntoIterator<Item = i32>
{
  let mut prev: i32 = 0;
  let mut asc: Option<bool> = None;
  for (i,v) in report.into_iter().enumerate() {
    if i > 0 {
      let new_asc = prev < v;
      if let Some(asc) = asc {
        if new_asc != asc {
          return false
        }
      } else {
        asc = Some(new_asc)
      }
      let delta = (prev - v).abs();
      if delta < 1 || delta > 3 {
        return false
      }
    }
    prev = v;
  }
  asc.is_some()
}
