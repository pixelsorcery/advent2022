use std::io;

fn main() {
    let mut max = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    let mut sum = 0;
    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }
        if "\n" == line {
            if sum > max {
                max3 = max2;
                max2 = max;
                max = sum;
                sum = 0;
                println!("{} {} {}", max, max2, max3);
                continue;
            }
            if sum > max2 {
                max3 = max2;
                max2 = sum;
                sum = 0;
                println!("{} {} {}", max, max2, max3);
                continue;
            }
            if sum > max3 {
                max3 = sum;
                sum = 0;
                println!("{} {} {}", max, max2, max3);
                continue;
            }
            sum = 0;
            continue;
        }
        let line: u32 = line.trim().parse().expect("Please type a number!");
        sum += line;
    }

    println!("Part1: {max}");
    println!("Part2: {}", max + max2 + max3);
}