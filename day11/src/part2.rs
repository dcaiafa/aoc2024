use std::collections::HashMap;

use anyhow::Error;

pub fn eval(input: &str) -> Result<i64, Error> {
    let state: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(str::parse::<i64>)
        .collect::<Result<_, _>>()?;
    
    let n = 75;
    let mut mem: HashMap<(i64,i32),i64> = HashMap::new();
    Ok(state.into_iter().map(|v|eval_v(v, n, &mut mem)).sum())
}

fn eval_v(v: i64, n: i32, mem: &mut HashMap<(i64,i32),i64>) -> i64 {
    if n == 0 {
        return 1;
    }
    if let Some(r) = mem.get(&(v,n)) {
        return *r;
    }
    let r = match v {
        0 => eval_v(1, n-1, mem),
        v if l10(v) % 2 == 0 => {
            let m = 10_i64.pow(l10(v)/2);
            eval_v(v / m, n-1, mem) + eval_v(v % m, n-1, mem)
        },
        v => eval_v(v * 2024, n-1, mem),
    };

    mem.insert((v,n), r);
    return r;
}

fn l10(v: i64) -> u32 {
    assert!(v > 0);
    let mut r = 0;
    let mut v = v.abs();
    while v > 0 {
        v /= 10;
        r += 1;
    }
    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_l10() {
        assert_eq!(l10(1), 1);
        assert_eq!(l10(999), 3);
        assert_eq!(l10(1000), 4);
        assert_eq!(l10(17), 2);
    }
}
