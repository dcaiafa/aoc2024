use crate::input::Input;
use crate::vm;

pub fn eval(s: &str) -> String {
    let input = Input::parse(s);
    let res = vm::eval(&input.prog, input.a as i64);
    res.iter().map(|v|v.to_string()).collect::<Vec<String>>().join(",")
}
