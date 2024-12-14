use crate::input::{Input, Machine, Number};

pub fn eval(input: &Input) -> Number {
    input.machines.iter()
        .map(|m|match eval_machine(m) {
            Some((a,b)) => a*3 + b,
            None => 0
        })
        .sum()
}

fn eval_machine(m: &Machine) -> Option<(Number,Number)> {
    let a = idiv(
        m.y_p*m.x_b - m.y_b*m.x_p,
        m.x_b*m.y_a - m.y_b*m.x_a)?;
    let b = idiv(
        m.x_p - m.x_a*a,
        m.x_b)?;
    Some((a, b))
}

fn idiv(n: Number, d:Number) -> Option<Number> {
    if n % d == 0 {
        Some(n/d)
    } else {
        None
    }
}
