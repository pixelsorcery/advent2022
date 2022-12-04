use std::io;

fn calc_part_1(line: &String) -> u32 {
    let size = line.chars().count();
    let (a, b) = line.split_at(size/2);
    let common_val = find_common_val(a, b);
    calc_value(common_val)
}

fn calc_part_2(line1: &String, line2: &String, line3: &String) -> u32 {
    let common_val = find_common_val_3(line1, line2, line3);
    calc_value(common_val)
}

fn calc_value(x: char) -> u32 {
    let val = x as u32;
    if val < 91 {
        return val - 65 + 27;
    } else {
        return val - 96;
    }
}

fn find_common_val(a: &str, b: &str) -> char {
    for achar in a.chars() {
        for bchar in b.chars() {
            if achar == bchar {
                return achar
            };
        }
    }

    return ' ';
}

fn find_common_val_3(a: &str, b: &str, c: &str) -> char {
    for achar in a.chars() {
        for bchar in b.chars() {
            for cchar in c.chars() {
                if achar == bchar && bchar == cchar{
                    return achar;
                }
            }
        }
    }

    return ' ';
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    loop {
        let mut line1 = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();
        let bytes = io::stdin()
            .read_line(&mut line1)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }
        io::stdin()
            .read_line(&mut line2)
            .expect("Failed to read line");
        io::stdin()
            .read_line(&mut line3)
            .expect("Failed to read line");
        part1 += calc_part_1(&line1);
        part1 += calc_part_1(&line2);
        part1 += calc_part_1(&line3);

        part2 += calc_part_2(&line1, &line2, &line3);
    }

    println!("Pt1: {part1}");
    println!("Pt2: {part2}");
}