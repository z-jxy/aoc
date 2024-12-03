use std::{iter::Peekable, str::Chars};

pub fn solve(chars: &mut Peekable<Chars>, p2: bool) -> usize {
    let mut result = 0;
    let mut enabled = true;
    while let Some(c) = chars.next() {
        match c {
            'd' if p2 => {
                let mut strr = String::from("d");

                for _ in 0..3 {
                    if let Some(ch) = chars.next() {
                        strr.push(ch);
                    }
                }

                if strr == "do()" {
                    enabled = true;
                    continue;
                }

                // don't()
                for _ in 0..3 {
                    if let Some(ch) = chars.next() {
                        strr.push(ch);
                    }
                }

                if strr == "don't()" {
                    enabled = false;
                    continue;
                }
            }
            'm' if enabled => {
                let mut strr = String::from("m");

                for _ in 0..2 {
                    if let Some(ch) = chars.next() {
                        strr.push(ch);
                    }
                }

                if strr != "mul" {
                    continue;
                }

                if let Some(ch) = chars.next() {
                    if ch == '(' {
                        let mut args = Vec::new();
                        let mut arg = String::new();
                        while let Some(ch) = chars.next() {
                            if ch == ',' {
                                args.push(arg.parse::<i32>().unwrap());
                                arg = String::new();
                            } else if ch == ')' {
                                args.push(arg.parse::<i32>().unwrap());
                                break;
                            } else if ch.is_numeric() {
                                arg.push(ch);
                            } else {
                                break;
                            }
                        }
                        if args.len() != 2 {
                            continue;
                        }

                        result += (args[0] * args[1]) as usize;
                    }
                }
            }
            _ => {}
        }
    }
    result
}

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let mut chars = input.chars().peekable();

    let p1 = solve(&mut chars.clone(), false);
    let p2 = solve(&mut chars, true);

    (p1 as usize, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        const EXAMPLE_P1: &str =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let p1 = solve(&mut EXAMPLE_P1.chars().peekable(), false);
        assert_eq!(p1, 161);
    }

    #[test]
    fn p2() {
        const EXAMPLE_P2: &str =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        let p1 = solve(&mut EXAMPLE_P2.chars().peekable(), true);
        assert_eq!(p1, 48);
    }
}
