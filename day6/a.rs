use std::io;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let strln = line.trim().chars().count();
    let chars: Vec<char> = line.chars().collect();
    let mut idx = 0;
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..strln-3 {
        println!("strln is {strln}");
        set.insert(chars[i]);
        set.insert(chars[i+1]);
        set.insert(chars[i+2]);
        set.insert(chars[i+3]);

        if set.len() == 4 {
            idx = i+4;
            break;
        }
        set.clear();
    }

    println!("pt1: {idx}");

    set.clear();
    for i in 0..strln-13 {
        for j in 0..14 {
            set.insert(chars[i+j]);
        }
        if set.len() == 14 {
            idx = i+14;
            break;
        }
        set.clear();
    }

    println!("pt2: {idx}");
}