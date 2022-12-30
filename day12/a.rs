use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

fn get_idx(pos:&(i32, i32, i32), width: i32) -> i32{
    pos.1 * width + pos.0
}

fn get_steps(map: &Vec<i32>, 
    startpos:&(i32, i32, i32), 
    endpos:&(i32, i32, i32),
    width: i32, 
    height: i32) -> i32 {

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    queue.push_back(*startpos);

    // bfs
    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        let curidx = get_idx(&cur, width) as usize;

        if cur.0 == endpos.0 && cur.1 == endpos.1 {
            return cur.2;
        }

        // check all 4 directions
        for dir in &dirs {
            let newpos = (cur.0 + dir.0, cur.1 + dir.1, cur.2 + 1);
            if newpos.0 >= 0 && 
                newpos.0 < width &&
                newpos.1 >= 0 && 
                newpos.1 < height {
                    let idx = get_idx(&newpos, width) as usize;
                    let delta = map[idx] - map[curidx];
                    if delta <= 1 {
                        let pos = (newpos.0, newpos.1);
                        if !memo.contains_key(&pos) {
                            queue.push_back(newpos.clone());
                            memo.insert(pos, 1);
                        }
                    }
            }
        }
    }
    return 999999;
}

fn main() {
    let mut map: Vec<i32> = Vec::new();
    //let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    //let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    //let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    // read in map
    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let mut vals: Vec<i32> = line.trim().chars().map(|c| {c as i32}).collect();
        width = vals.len() as i32;
        map.append(&mut vals);
        height += 1;
    }

    // find start and end
    let mut startpos:(i32, i32, i32) = (0,0,0);
    let mut endpos:(i32, i32, i32) = (0,0,0);
    for i in 0..map.len() {
        let idx = i as i32;
        if map[i] == ('S' as i32) {
            startpos = (idx % width, idx / width, 0);
            map[i] = 'a' as i32;
            continue;
        }
        if map[i] == ('E' as i32) {
            endpos = (idx % width, idx / width, 0);
            map[i] = 'z' as i32;
            continue;
        }
    }

    let pt1 = get_steps(&map, &startpos, &endpos, width, height);
    println!("pt1: {pt1}");

    let mut lowest = 99999;
    for i in 0..map.len() {
        let idx = i as i32;
        if map[i] == 'a' as i32 {
            let apos = (idx % width, idx / width, 0);
            let res = get_steps(&map, &apos, &endpos, width, height);

            if res < lowest {
                lowest = res;
            }
        }
    }

    println!("pt2: {lowest}");

}