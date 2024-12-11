use crate::input::Input;

pub fn eval(input: &Input) -> i32 {
    let mut n = 0;
    for y in 0..input.height() {
        for x in 0..input.width() {
            if input.get(x, y) == 0 {
                n += trails(&input, x, y, 0);
            }
        }
    }
    n
}

fn trails(
    input: &Input,
    x: i32,
    y: i32,
    e: u8,
) -> i32 {
    if x < 0 || x >= input.width() || y < 0 || y >= input.height() {
        return 0;
    }
    let v = input.get(x, y);
    if v != e {
        return 0;
    }
    if v == 9 {
        return 1;
    }

    let e = e + 1;
    trails(input, x + 1, y, e) +
    trails(input, x, y + 1, e) +
    trails(input, x - 1, y, e) +
    trails(input, x, y - 1, e) 
}
