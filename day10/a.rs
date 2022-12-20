use std::io;

fn main() {
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut results: Vec<i32> = Vec::new();
    let mut total = 0;
    let mut xreg:i32 = 1;
    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let mut line_itr = line.split(' ');
        let op = line_itr.next().unwrap().trim();

        if op == "addx" {
            let imm = line_itr.next().unwrap().trim().parse::<i32>().unwrap();
            results.push(xreg.clone());
            results.push(xreg.clone());
            xreg = xreg + imm;
        } else {
            results.push(xreg);
        }
    }

    for &cycle in &cycles {
        let xval = results[cycle-1];
        total += cycle as i32 * xval;
    }

    println!("pt1: {total}");

    let mut idx = 0;
    for _ in 0..6 {
        for x in 0..40 {
            let xpos = results[idx];
            if x == xpos || x == xpos - 1 || x == xpos + 1 {
                print!("#");
            } else {
                print!(".");
            }
            idx += 1;
        }
        println!("");
    }
}