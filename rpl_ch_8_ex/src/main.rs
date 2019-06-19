
use std::collections::HashMap;
use std::io;
fn main() {

    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mean = calc_mean(&numbers);
    println!("The mean of {:?} is {}", numbers, mean);

    let median = calc_median(&numbers);
    println!("The median of {:?} is {}", numbers, median);

    let mode = calc_mode(&numbers);
    println!("The mode of {:?} is {:?}", numbers, mode);
}

fn calc_mean(v: &Vec<i32>) -> f64 {
    let mut total = 0;
    for n in v {
        total += n;
    }
    let total = total as f64;
    total / v.len() as f64
}

fn calc_median(v: &Vec<i32>) -> f64 {
    let mut v2 = v.clone();
    v2.sort();
    let v_size = v2.len();
    if v_size % 2 == 0 {
        return ((v2[v_size / 2] + v2[(v_size / 2) - 1]) / 2) as f64;
    }
    v2[v2.len() / 2] as f64
}

fn calc_mode(v: &Vec<i32>) -> Vec<i32> {
    let mut counts = HashMap::new();

    for &value in v {
        let count = counts.entry(value).or_insert(0);
        *count += 1;
    }

    let max_value = counts.values().max().unwrap();
    let mut mode_vec: Vec<i32> = Vec::new();
    for (key, val) in counts.iter() {
        if val == max_value {
            mode_vec.push(*key);
        }

    }
    mode_vec
}