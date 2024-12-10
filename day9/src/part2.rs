use anyhow::{anyhow, Error};

#[derive(Debug, Clone)]
struct Chunk {
    len: usize,
    id: Option<u16>,
}

pub fn eval(input: &str) -> Result<i64, Error> {
    let mut file_id: u16 = 0;
    let mut chunks = Vec::new();

    let mut empty = false;
    for c in input.chars().map(|c| c.to_digit(10)) {
        let c = c.ok_or(anyhow!("invalid block character"))?;
        let id = match empty {
            true => None,
            false => {
                let id = file_id;
                file_id += 1;
                Some(id)
            }
        };
        empty = !empty;
        chunks.push(Chunk {
            len: c as usize,
            id,
        });
    }
    compact(&mut chunks);
    Ok(checksum(&chunks))
}

fn compact(cs: &mut Vec<Chunk>) {
    let mut f = (cs.len() as i32) - 1;
    while f >= 0 {
        if cs[f as usize].id.is_none() {
            f -= 1;
            continue;
        }
        let fc = &cs[f as usize];
        let ec = cs
            .iter()
            .enumerate()
            .take_while(|(i,_)|*i < (f as usize))
            .find(|(_, c)| c.id.is_none() && c.len >= fc.len);
        match ec {
            None => (),
            Some((i, c)) => {
                let d = c.len - fc.len;
                cs[i] = fc.clone();
                cs[f as usize].id = None;
                if d > 0 {
                    cs.insert(i + 1, Chunk { len: d, id: None });
                    f += 1;
                }
            }
        }
        f -= 1;
    }
}

fn checksum(cs: &[Chunk]) -> i64 {
    cs.into_iter()
        .map(|c| match c.id {
            Some(id) => std::iter::repeat(Some(id)).take(c.len),
            None => std::iter::repeat(None).take(c.len),
        })
        .flatten()
        .enumerate()
        .map(|(i, b)| match b {
            Some(id) => (id as i64) * (i as i64),
            None => 0,
        })
        .sum()
}

#[allow(dead_code)]
fn dump(cs: &[Chunk]) {
    cs.into_iter()
        .map(|c| match c.id {
            Some(id) => std::iter::repeat(id.to_string().chars().next().unwrap()).take(c.len),
            None => std::iter::repeat('.').take(c.len),
        })
        .flatten()
        .for_each(|c| print!("{}", c));
    println!("");
}
