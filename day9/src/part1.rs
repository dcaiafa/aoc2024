use anyhow::{anyhow, Error};

type Block = Option<u16>;

pub fn eval(input: &str) -> Result<i64, Error> {
    let mut file_id: u16 = 0;
    let mut blocks: Vec<Block> = Vec::new();

    let mut empty = false;
    for c in input.chars().map(|c| c.to_digit(10)) {
        let c = c.ok_or(anyhow!("invalid block character"))?;
        let block = match empty {
            true => None,
            false => {
                let id = file_id;
                file_id += 1;
                Some(id)
            }
        };
        empty = !empty;
        blocks.extend(std::iter::repeat(block).take(c as usize));
    }

    compact(&mut blocks);

    Ok(checksum(&blocks))
}

fn compact(bs: &mut Vec<Block>) {
    let mut e: usize = 0;
    let mut o: usize = bs.len() - 1;
    while e < o {
        while e < o && bs[e].is_some() {
            e += 1;
        }
        while e < o && bs[o].is_none() {
            o -= 1;
        }
        bs.swap(e, o);
    }
}

fn checksum(bs: &[Block]) -> i64 {
    bs.into_iter().take_while(|b| b.is_some())
        .enumerate()
        .map(|(i,b)| (i as i64) * (b.unwrap() as i64))
        .sum()
}

#[allow(dead_code)]
fn dump_blocks(bs: &[Block]) {
    for b in bs {
        let id = match b {
            None => ".".to_string(),
            Some(id) => id.to_string(),
        };
        print!("{}", id);
    }
    println!("");
}
