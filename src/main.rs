use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let start = input_line.trim_end().to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let target = input_line.trim_end().to_string();

    let answer = solve(start.clone(), target.clone());
    println!("start: {} \ntarget: {} \nanswer: {}", start, target, answer);
}

fn solve(start: String, target: String) -> usize{0}

#[cfg(test)]
mod main_tests {
    use crate::solve;

    #[test]
    fn example_1() {
        let start = String::from("1101");
        let target = String::from("0100");
        let answer = solve(start, target);
        assert_eq!(answer, 2);
    }

    fn example_2() {
        let start = String::from("101010");
        let target = String::from("010101");
        let answer = solve(start, target);
        assert_eq!(answer, 26);
    }

    fn example_3() {
        let start = String::from("11001001000");
        let target = String::from("10000110011");
        let answer = solve(start, target);
        assert_eq!(answer, 877);
    }
}