use anyhow::{anyhow, bail, Error};

pub struct Input {
    width: i32,
    height: i32,
    rows: Vec<Vec<u8>>,
}

impl Input {
    pub fn parse(s: &str) -> Result<Input, Error> {
        let rows: Vec<Vec<u8>> = s
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| c.to_digit(10).ok_or(anyhow!("not a digit")))
                    .map(|c| match c {
                        Ok(n) => Ok(n as u8),
                        Err(e) => Err(e),
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        if rows.is_empty() {
            bail!("input is empty");
        }
        let height: i32 = rows.len().try_into()?;
        let width: i32 = rows[0].len().try_into()?;

        if !rows.iter().all(|r| r.len() == (width as usize)) {
            bail!("inconsistent width");
        }

        Ok(Input {
            width,
            height,
            rows,
        })
    }

    pub fn get(&self, x: i32, y: i32) -> u8 {
        self.rows[y as usize][x as usize]
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        for r in &self.rows {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
    }
}
