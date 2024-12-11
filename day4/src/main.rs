use anyhow::{bail, Error};
use std::io::{BufRead, Read};

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("expected input filename");
    }
    let input = &args[1];
    let input_file = std::fs::File::open(input)?;
    let input_reader = std::io::BufReader::new(input_file);
    let grid = Grid::new(input_reader)?;

    println!("eval1 = {}", eval1(&grid));
    println!("eval2 = {}", eval2(&grid));

    Ok(())
}

fn eval1(g: &Grid) -> i32 {
    let mut count = 0;
    for j in 0..g.height() {
        for i in 0..g.width() {
            count += find1(g, b"XMAS", i, j, -1, -1).unwrap_or(0)
                + find1(g, b"XMAS", i, j, 0, -1).unwrap_or(0)
                + find1(g, b"XMAS", i, j, 1, -1).unwrap_or(0)
                + find1(g, b"XMAS", i, j, -1, 0).unwrap_or(0)
                + find1(g, b"XMAS", i, j, 1, 0).unwrap_or(0)
                + find1(g, b"XMAS", i, j, -1, 1).unwrap_or(0)
                + find1(g, b"XMAS", i, j, 0, 1).unwrap_or(0)
                + find1(g, b"XMAS", i, j, 1, 1).unwrap_or(0);
        }
    }
    count
}

fn find1(g: &Grid, s: &[u8], i: i32, j: i32, di: i32, dj: i32) -> Option<i32> {
    assert!(s.len() > 0);

    let expected = s[0];
    let actual = g.get(i, j)?;

    if expected != actual {
        return None;
    }

    let s = &s[1..];
    if s.len() == 0 {
        return Some(1);
    }

    find1(g, s, i + di, j + dj, di, dj)
}

fn eval2(g: &Grid) -> i32 {
    let mut count = 0;
    for j in 0..g.height() {
        for i in 0..g.width() {
            count += find2(g, i, j).unwrap_or(0)
        }
    }
    count
}

fn find2(g: &Grid, i: i32, j: i32) -> Option<i32> {
    let s = b"MAS";

    if !find1(g, s, i - 1, j - 1, 1, 1).is_some() && !find1(g, s, i + 1, j + 1, -1, -1).is_some() {
        return None;
    }

    if !find1(g, s, i + 1, j - 1, -1, 1).is_some() && !find1(g, s, i - 1, j + 1, 1, -1).is_some() {
        return None;
    }

    Some(1)
}

struct Grid {
    rows: Vec<Vec<u8>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new<R>(mut reader: R) -> Result<Grid, Error>
    where
        R: Read + BufRead,
    {
        let mut rows: Vec<Vec<u8>> = Vec::new();
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            let row = line.into_bytes();
            if rows.len() != 0 {
                if row.len() != rows[0].len() {
                    bail!("inconsistent row lengths");
                }
            }
            line = String::with_capacity(row.len());
            rows.push(row);
        }
        if rows.len() == 0 {
            bail!("grid is empty");
        }
        let width: i32 = rows[0].len().try_into().unwrap();
        let height: i32 = rows.len().try_into().unwrap();
        Ok(Grid {
            rows,
            width,
            height,
        })
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn get(&self, i: i32, j: i32) -> Option<u8> {
        if i >= self.width || j >= self.height {
            return None;
        }
        let ui: usize = i.try_into().ok()?;
        let uj: usize = j.try_into().ok()?;

        Some(self.rows[uj][ui])
    }
}
