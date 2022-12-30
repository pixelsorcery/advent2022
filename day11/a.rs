use std::io;

#[derive(PartialEq, Eq, Clone)]
enum Operation {
    Add,
    Mul
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    op_param: Vec<String>,
    test: u64,
    true_monkey: u32,
    false_monkey: u32
}

fn read_line() -> (usize, String) {
    let mut line = String::new();
    let bytes = io::stdin()
    .read_line(&mut line)
    .expect("Failed to read line");
    return (bytes, line);
}

fn part1(monkeys: &mut Vec<Monkey>, iterations: u32, is_pt1: bool) {
    let mut counts: Vec<u64> = vec![0; monkeys.len()];

    let mut modulus: u64 = 1;
    for i in 0..monkeys.len() {
        modulus *= monkeys[i].test;
    }

    for _ in 0..iterations{
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                counts[i] += 1;
                let item = monkeys[i].items.pop().unwrap().clone();

                let first: u64;
                let second: u64;
                if monkeys[i].op_param[0] == "old" {
                    first = item;
                } else {
                    first = monkeys[i].op_param[0].parse::<u64>().unwrap();
                }

                if monkeys[i].op_param[1] == "old" {
                    second = item;
                } else {
                    second = monkeys[i].op_param[1].parse::<u64>().unwrap();
                }

                let mut result: u64;
                if monkeys[i].op == Operation::Mul {
                    result = first * second;
                } else {
                    result = first + second;
                }

                if is_pt1 {
                    result /= 3;
                }

                let rem = result % monkeys[i].test;

                let monkey_idx: usize;
                if rem == 0 {
                    monkey_idx = monkeys[i].true_monkey as usize;
                } else {
                    monkey_idx = monkeys[i].false_monkey as usize;
                }

                monkeys[monkey_idx].items.push(result % modulus);
            }
        }
    }

    counts.sort_by(|a, b| b.cmp(a));
    println!("result: {}", counts[0] * counts[1]);
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut line: String;

    loop {
        let (mut bytes, _) = read_line();
        if bytes == 0 {
            break;
        }

        (bytes, line) = read_line();
        if bytes == 0 {
            break;
        }

        line = line.replace(",", "");
        let words: Vec<&str> = line.trim().split_whitespace().collect();

        let mut items:Vec<u64> = Vec::new();

        for i in 0..words.len() {
            match words[i].trim().parse::<u64>() {
                Ok(n) => {
                    items.push(n);
                },
                Err(..) => continue
            }
        }
        
        //println!("{:?}", items);

        (bytes, line) = read_line();
        if bytes == 0 {
            break;
        }
        
        let words: Vec<&str> = line.trim().split_whitespace().collect();

        let mut params:Vec<String> = Vec::new();
        params.push(words[3].to_string());
        params.push(words[5].to_string());
        //println!("{:?}", params);

        let oper: Operation;
        if words[4] == "*" {
            oper = Operation::Mul;
        } else {
            oper = Operation::Add;
        }
        
        (bytes, line) = read_line();
        if bytes == 0 {
            break;
        }

        let words: Vec<&str> = line.trim().split_whitespace().collect();
        let test = words[3].trim().parse::<u64>().unwrap();

        //println!("test is: {}", test);
        (bytes, line) = read_line();
        if bytes == 0 {
            break;
        }
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        let true_test: u32 = words[5].trim().parse::<u32>().unwrap();
        //println!("true_test is: {}", true_test);
        (bytes, line) = read_line();
        if bytes == 0 {
            break;
        }
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        let false_test: u32 = words[5].trim().parse::<u32>().unwrap();
        //println!("false_test is: {}", false_test);

        monkeys.push(Monkey{
            items: items,
            op: oper,
            op_param: params,
            test: test,
            true_monkey: true_test,
            false_monkey: false_test
        });

        // read blank line in between
        (bytes, _) = read_line();
        if bytes == 0 {
            break;
        }
    }


    let mut monkeys2 = monkeys.clone();
    part1(&mut monkeys, 20, true);
    part1(&mut monkeys2, 10000, false);
}