use std::io;
use anyhow::{Result, bail};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let start_line = input_line.trim_end().to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let target_line = input_line.trim_end().to_string();
    let (start, target) = match parse_inputs(start_line, target_line) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {}", e);
            return;
        },
    };

    let answer = solve(start.clone(), target.clone());
    println!("start: {:?} \ntarget: {:?} \nanswer: {}", start, target, answer);
}

fn parse_inputs(start_line: String, target_line: String) -> Result<(Vec<u8>, Vec<u8>)> {
    if target_line.len() != start_line.len() {
        bail!("start and target outputs must be of same size");
    }

    let (mut start, mut target) = (Vec::new(), Vec::new());
    for c in start_line.split("").collect::<Vec<_>>() {
        if c.len() > 0 {
            start.push(parse_input!(c, u8));
        }
    }
    for c in target_line.split("").collect::<Vec<_>>() {
        if c.len() > 0 {
            target.push(parse_input!(c, u8));
        }
    }

    Ok((start, target))
}

fn solve(start: Vec<u8>, target: Vec<u8>) -> usize {
    0
}

fn rule_check(current_state: &Vec<u8>, pos: usize) -> bool {
    if pos == current_state.len() - 1 {
        return true;
    }
    if current_state[pos + 1] == 1 {
        let mut i = pos + 2;
        while i < current_state.len() {
            if current_state[i] == 1 {
                return false;
            }
            i += 1;
        }
        return true;
    }

    false
}

#[cfg(test)]
mod main_tests {
    use crate::{rule_check, solve, parse_inputs};

    #[test]
    fn parse_inputs_example_1() {
        let start_line = "1101".to_string();
        let target_line = "0010".to_string();
        let (start, target) = parse_inputs(start_line, target_line).unwrap();
        assert_eq!(start, vec![1,1,0,1]);
        assert_eq!(target, vec![0,0,1,0]);
    }

    #[test]
    fn rule_check_example_1() {
        let state = vec![1,1,0,1];
        assert_eq!(rule_check(&state, 0), false);
        assert_eq!(rule_check(&state, 1), false);
        assert_eq!(rule_check(&state, 2), true);
        assert_eq!(rule_check(&state, 3), true);
    }

    #[test]
    fn solve_example_1() {
        let start = vec![1,1,0,1];
        let target = vec![0,0,1,0];
        let answer = solve(start, target);
        assert_eq!(answer, 2);
    }

    #[test]
    fn solve_example_2() {
        let start = vec![1,0,1,0,1,0];
        let target = vec![0,1,0,1,0,1];
        let answer = solve(start, target);
        assert_eq!(answer, 26);
    }

    #[test]
    fn solve_example_3() {
        let start = vec![1,1,0,0,1,0,0,1,0,0,0];
        let target = vec![1,0,0,0,0,1,1,0,0,1,1];
        let answer = solve(start, target);
        assert_eq!(answer, 877);
    }
}