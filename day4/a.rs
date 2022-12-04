use std::io;

fn contains(s1:&u32, e1:&u32, s2:&u32, e2:&u32) -> bool {
    s2 >= s1 && s2 <= e1 && e2 >= s1 && e2 <= e1
}

fn overlap(s1:&u32, e1:&u32, s2:&u32, e2:&u32) -> bool {
    s2 >= s1 && s2 <= e1 || e2 >= s1 && e2 <= e1
}

fn check_ranges(ranges: &Vec<(u32, u32, u32, u32)>) -> u32 {
    let mut contained = 0;
    for range in ranges {
        let (s1, e1, s2, e2) = range;
        if contains(s1, e1, s2, e2) || contains(s2, e2, s1, e1) {
            contained += 1;
        }
    }
    contained
}

fn check_any_overlap(ranges: &Vec<(u32, u32, u32, u32)>) -> u32 {
    let mut contained = 0;
    for range in ranges {
        let (s1, e1, s2, e2) = range;
        if overlap(s1, e1, s2, e2) || overlap(s2, e2, s1, e1) {
            contained += 1;
        }
    }
    contained
}

fn main() {
    let mut ranges: Vec<(u32, u32, u32, u32)> = Vec::new();
    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }
        let mut iter = line.split(',');
        let r1 = iter.next().unwrap();
        let r2 = iter.next().unwrap();
        iter = r1.split('-');
        let s1:u32 = iter.next().unwrap().trim().parse().unwrap();
        let e1:u32 = iter.next().unwrap().trim().parse().unwrap();
        iter = r2.split('-');
        let s2:u32 = iter.next().unwrap().trim().parse().unwrap();
        let e2:u32 = iter.next().unwrap().trim().parse().unwrap();
        ranges.push((s1, e1, s2, e2));
    }

    let p1 = check_ranges(&ranges);
    let p2 = check_any_overlap(&ranges);
    println!("p1: {p1}");
    println!("p2: {p2}");
}