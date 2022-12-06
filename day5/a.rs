use std::io;

struct Instr {
    count: u32,
    from: u32,
    to: u32
}

fn eval(inst: &Instr, stacks: &mut Vec<Vec<char>>) {
    let from = inst.from as usize - 1;
    let to = inst.to as usize - 1;
    for _ in 0..inst.count {
        let val = stacks[from].pop().unwrap();
        stacks[to].push(val);
    }
}

fn eval_2(inst: &Instr, stacks: &mut Vec<Vec<char>>) {
    let from = inst.from as usize - 1;
    let to = inst.to as usize - 1;
    let mut temp: Vec<char> = Vec::new();
    for _ in 0..inst.count {
        let val = stacks[from].pop().unwrap();
        temp.push(val);
    }
    temp.reverse();
    stacks[to].append(&mut temp);
}

fn part1(instrs: &Vec<Instr>, stacks: &mut Vec<Vec<char>>) -> String {
    for inst in instrs {
        eval(&inst, stacks);
    }

    let mut word: String = "".to_string();
    for v in stacks {
        word += &v.last().unwrap().to_string();
    }
    return word;
}

fn part2(instrs: &Vec<Instr>, stacks: &mut Vec<Vec<char>>) -> String {
    for inst in instrs {
        eval_2(&inst, stacks);
    }

    let mut word: String = "".to_string();
    for v in stacks {
        word += &v.last().unwrap().to_string();
    }
    return word;
}

fn main() {
    let mut stacks: Vec<Vec<char>> = vec![
    vec!['H','B','V','W','N','M','L','P'], 
    vec!['M','Q','H'], 
    vec!['N','D','B','G','F','Q','M','L'], 
    vec!['Z','T','F','Q','M','W','G'],
    vec!['M','T','H','P'],
    vec!['C','B','M','J','D','H','G','T'],
    vec!['M','N','B','F','V','R'],
    vec!['P','L','H','M','R','G','S'],
    vec!['P','D','B','C','N']];

    // skip the whole stack part
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line == "\n" {
            break;
        }
    }

    let mut instrs: Vec<Instr> = Vec::new();

    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let line_itr = line.split(' ');
        let line_vec: Vec<&str> = line_itr.collect();
        instrs.push(Instr {
            count: line_vec[1].parse::<u32>().unwrap(),
            from: line_vec[3].parse::<u32>().unwrap(),
            to: line_vec[5].trim().parse::<u32>().unwrap()
        });
    }

    let p1 = part1(&instrs, &mut stacks);

    stacks = vec![
    vec!['H','B','V','W','N','M','L','P'], 
    vec!['M','Q','H'], 
    vec!['N','D','B','G','F','Q','M','L'], 
    vec!['Z','T','F','Q','M','W','G'],
    vec!['M','T','H','P'],
    vec!['C','B','M','J','D','H','G','T'],
    vec!['M','N','B','F','V','R'],
    vec!['P','L','H','M','R','G','S'],
    vec!['P','D','B','C','N']];

    let p2 = part2(&instrs, &mut stacks);
    println!("p1: {p1}");
    println!("p2: {p2}");
}