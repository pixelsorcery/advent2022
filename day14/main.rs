use std::io;
use std::collections::HashMap;

fn parse_pairs(pairs: Vec<&str>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for pair in pairs {
        let numbers: Vec<&str> = pair.split(',').collect();
        if numbers.len() != 2 {
            panic!("Invalid input: {}", pair);
        }
        let x = numbers[0].parse::<i32>().unwrap();
        let y = numbers[1].parse::<i32>().unwrap();
        result.push((x, y));
    }
    result
}

fn main() {
    let mut state: HashMap<(i32, i32), char> = HashMap::new();
    let wall_value = '#';
    let start_pos = (500, 0);

    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let coords: Vec<&str> = line.trim().split(" -> ").collect();

        let pairs = parse_pairs(coords);

        for i in 1..pairs.len() {
            if pairs[i-1].0 < pairs[i].0 {
                let xbegin = pairs[i-1].0;
                let xend = pairs[i].0;
                for j in xbegin..xend {
                    state.insert((j, pairs[i as usize].1), wall_value);
                }
            } else {
                let xbegin = pairs[i].0;
                let xend = pairs[i-1].0;
                for j in xbegin..xend {
                    state.insert((j, pairs[i as usize].1), wall_value);
                }
            }

            if pairs[i-1].1 < pairs[i].1 {
                let xbegin = pairs[i-1].1;
                let xend = pairs[i].1;
                for j in xbegin..xend {
                    state.insert((pairs[i as usize].0, j), wall_value);
                }
            } else {
                let xbegin = pairs[i].1;
                let xend = pairs[i-1].1;
                for j in xbegin..xend {
                    state.insert((pairs[i as usize].0, j), wall_value);
                }
            }

            state.insert(pairs[i-1], wall_value);
            state.insert(pairs[i], wall_value);
        }
    }

    // find max
    let mut maxy = 0;
    for (key, _) in &state {
        if key.1 > maxy { maxy = key.1};
    }
    
    let mut cury = 0;

    while cury < maxy {
        let mut pos = start_pos;
        while cury < maxy {
            let mut newpos = (pos.0, pos.1+1);
            if state.contains_key(&newpos) {
                newpos = (pos.0-1, pos.1+1);
                if pos.1 > cury {cury = pos.1}
            } else {
                pos = newpos;
                if pos.1 > cury {cury = pos.1}
                continue;
            }
            if state.contains_key(&newpos) {
                newpos = (pos.0+1, pos.1+1);
                if pos.1 > cury {cury = pos.1}
            } else {
                pos = newpos;
                if pos.1 > cury {cury = pos.1}
                continue;
            }
            if state.contains_key(&newpos) {
                state.insert(pos, 'o');
                if pos.1 > cury {cury = pos.1}
                break;
            } else {
                pos = newpos;
                if pos.1 > cury {cury = pos.1}
                continue;
            }
        }
    }

    let mut res = 0;
    for (_, val) in &state {
        if *val == 'o' { res += 1;}
    }

    println!("pt1: {res}");

    let floor = maxy + 2;
    let mut done = false;
    while !done {
        let mut pos = start_pos;
        loop {
            let mut newpos = (pos.0, pos.1+1);

            if newpos.1 == floor {
                state.insert(pos, 'o');
                break;
            }

            if state.contains_key(&newpos) {
                newpos = (pos.0-1, pos.1+1);
                if pos.1 > cury {cury = pos.1}
            } else {
                pos = newpos;
                if pos.1 > cury {cury = pos.1}
                continue;
            }
            if state.contains_key(&newpos) {
                newpos = (pos.0+1, pos.1+1);
                if pos.1 > cury {cury = pos.1}
            } else {
                pos = newpos;
                if pos.1 > cury {cury = pos.1}
                continue;
            }
            if state.contains_key(&newpos) {
                state.insert(pos, 'o');
                if pos == start_pos {
                    done = true;
                    break;
                }
                if pos.1 > cury {cury = pos.1}
                break;
            } else {
                pos = newpos;
                if pos.1 > cury {cury = pos.1}
                continue;
            }
        }
    }

    res = 0;
    for (_, val) in &state {
        if *val == 'o' { res += 1;}
    }

    println!("pt2: {res}");
}