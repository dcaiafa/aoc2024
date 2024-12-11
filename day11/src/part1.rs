use anyhow::Error;

pub fn eval(input: &str) -> Result<i64, Error> {
    let mut state: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(str::parse::<i64>)
        .collect::<Result<_, _>>()?;
    let n = 25;
    for _ in 0..n {
        state = state
            .into_iter()
            .map(|v| match v {
                0 => vec![1],
                v if l10(v) % 2 == 0 => {
                    let m = 10_i64.pow(l10(v) / 2);
                    vec![v / m, v % m]
                }
                v => vec![v * 2024],
            })
            .flatten()
            .collect();
    }
    Ok(state.len() as i64)
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
