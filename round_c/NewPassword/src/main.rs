use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut i = 1;
    for line in stdin.lock().lines().skip(1) {
        if i%2 == 0 {
            solve(&line.unwrap(), i/2);
        }
        i += 1;
    }
}

fn solve(line: &str, case: i64) {
    let mut upper_ok: bool = false;
    let mut lower_ok: bool = false;
    let mut digit_ok: bool = false;
    let mut special_ok: bool = false;

    for each in line.chars() {
        if each.is_ascii_uppercase() {
            upper_ok = true;
        } else if each.is_ascii_lowercase() {
            lower_ok = true;
        } else if each.is_ascii_digit() {
            digit_ok = true;
        } else if is_spec(each) {
            special_ok = true;
        }
    }
    
    let mut new_string: String = line.to_string();

    if !upper_ok {
        new_string.push_str("A");
    } 
    if !lower_ok {
        new_string.push_str("a");
    } 
    if !digit_ok {
        new_string.push_str("1");
    } 
    if !special_ok {
        new_string.push_str("@");
    }

    while new_string.len() <7 {
        new_string.push_str("1");
    }
    println!("Case #{}: {}", case, new_string);
}

fn is_spec(sign: char) -> bool {
    match sign {
        '&'|'#'|'*'|'@' => true,
        _ => false
    }
}

