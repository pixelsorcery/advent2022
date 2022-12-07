use std::io;
use std::collections::HashSet;

fn solve(len: usize, chars: &Vec<char>) -> usize{
    let mut idx = 0;
    let strln = chars.len();
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..strln-(len-1) {
        for j in 0..len {
            set.insert(chars[(i+j)]);
        }
        if set.len() == len {
            idx = i+len;
            break;
        }
        set.clear();
    }

    return idx;
}

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let chars: Vec<char> = line.chars().collect();

    let pt1 = solve(4, &chars);
    println!("pt1: {pt1}");
    let pt2 = solve(14, &chars);
    println!("pt2: {pt2}");
}