use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut i = 1;
    for line in stdin.lock().lines().skip(1) {
        solve(&line.unwrap(), i);
        i += 1;
    }
}

fn solve(line: &str, case: i64) {
    let mut pos = false;

    let split_line: Vec<&str> = line.split(" ").collect();
    let n = split_line[0].parse::<i64>().unwrap();
    let mut x = split_line[1].parse::<i64>().unwrap();
    let mut y = split_line[2].parse::<i64>().unwrap();

    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }
    if x%y == 0 {
        x = x/y;
        y = 1;
    }
    if y%x == 0 {
        x = 1;
        y = y/x;
    }

    let mut alans: Vec<i64> = vec![];
    if sum%(x+y)==0 {
        let scale = sum/(x+y);
        alans = find_vec(n, x*scale);
    }

    if alans.len() != 0 {
        pos = true;
    }

    if pos {
        println!("Case #{}: POSSIBLE", case);
        println!("{}",alans.len());
        let mut numbs: String = alans[0].to_string();
        for each in alans.iter().skip(1) {
            numbs.push_str(" ");
            numbs.push_str(&each.to_string());
        }
        println!("{}",numbs);
    } else {
        println!("Case #{}: IMPOSSIBLE", case);
    }
}

fn find_vec(n: i64, sum: i64) -> Vec<i64> {
    if n == sum {
        return vec![n];
    } else if n == 0 {
        return vec![];
    } 
    
    if sum > n {
        let mut with = vec![n];
        let mut sub_vec = find_vec(n-1, sum-n);
        if sub_vec.len() > 0 {
            with.append(&mut sub_vec);
            return with;
        }
    }
    let mut without = find_vec(n-1, sum);
    if without.len() > 0 {
        return without;
    } 
    vec![]
}

