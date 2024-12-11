use anyhow::{bail, Error};
use std::{cmp::Ordering, collections::HashMap, io::BufRead, time::Instant};

fn main() -> Result<(), Error> {
    let start = Instant::now();
    let r = real_main();
    let duration = start.elapsed();
    dbg!(duration);
    r
}

fn real_main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Missing <filename>")
    }

    let filename = &args[1];
    let file = std::fs::File::open(filename)?;
    let mut reader = std::io::BufReader::new(file);

    let mut rules = Rules::new();

    let mut raw_line = String::new();
    while {
        raw_line.clear();
        reader.read_line(&mut raw_line)? > 0
    } {
        let line = raw_line.trim();
        if line.len() == 0 {
            break;
        }
        let parts: Vec<i32> = line
            .split("|")
            .map(|p| p.parse())
            .collect::<Result<_, _>>()?;
        if parts.len() != 2 {
            bail!("Invalid rule")
        }
        rules.add(parts[0], parts[1]);
    }

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    while {
        raw_line.clear();
        reader.read_line(&mut raw_line)? > 0
    } {
        let line = raw_line.trim();
        if line.len() == 0 {
            continue;
        }
        let report: Vec<i32> = line
            .split(",")
            .map(|p| p.parse())
            .collect::<Result<_, _>>()?;
        if report.len() < 2 && report.len() % 2 != 1 {
            bail!("Invalid report");
        }
        if let Some(n) = process1(&rules, &report) {
            sum1 += n;
        } else {
            sum2 += process2(&rules, &report);
        }
    }

    dbg!(sum1);
    dbg!(sum2);

    Ok(())
}

fn process1(rules: &Rules, report: &[i32]) -> Option<i32> {
    for i in 0..report.len() {
        for j in i+1..report.len() {
            match rules.cmp(report[i], report[j]) {
                Ordering::Less | Ordering::Equal => continue,
                Ordering::Greater => return None,
            }
        }
    }
    Some(report[report.len()/2])
}

fn process2(rules: &Rules, report: &[i32]) -> i32 {
    let mut r: Vec<i32> = report.into();
    for i in 0..r.len() {
        for j in i+1..r.len() {
            if rules.cmp(r[i], r[j]) == Ordering::Greater {
                r.swap(i, j);
            }
        }
    }
    r[r.len()/2]
}

struct Rules {
    rules: HashMap<(i32, i32), Ordering>,
}

impl Rules {
    fn new() -> Rules {
        Rules {
            rules: HashMap::new(),
        }
    }
    fn add(&mut self, left: i32, right: i32) {
        assert!(!self.rules.contains_key(&(left, right)));
        self.rules.insert((left, right), Ordering::Less);
        self.rules.insert((right, left), Ordering::Greater);
    }
    fn cmp(&self, left: i32, right: i32) -> Ordering {
        match self.rules.get(&(left, right)) {
            Some(o) => *o,
            None => Ordering::Equal,
        }
    }
}
