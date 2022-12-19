use std::io;
use std::collections::HashMap;

fn move_tail(headpos: (i32, i32), tailpos: (i32, i32)) -> (i32, i32){
    let surround = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

    let xdif = (headpos.0 - tailpos.0).abs();
    let ydif = (headpos.1 - tailpos.1).abs();

    let mut newpos = tailpos;
    let mut diff = 10;
    if xdif > 1 || ydif > 1 {
        // find a tile around tailpos where it's at least 1 away
        for dir in surround {
            let newxpos = tailpos.0 + dir.0;
            let newypos = tailpos.1 + dir.1;
            let newxdif = (headpos.0 - (tailpos.0 + dir.0)).abs();
            let newydif = (headpos.1 - (tailpos.1 + dir.1)).abs();
            let curdiff = newxdif + newydif;
            if curdiff <= 2 && curdiff < diff {
                diff = curdiff;
                newpos = (newxpos, newypos);
            }
        }

        return newpos;
    }

    return tailpos;
}

fn main() {
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut moves: Vec<(String, u32)> = Vec::new();
    let mut tail_positions = HashMap::<(i32, i32), u32>::new();
    let mut headpos:(i32, i32) = (0, 0);
    let mut tailpos:(i32, i32) = (0, 0);

    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let mut line_itr = line.split(' ');
        let dir = line_itr.next().unwrap();
        let amt = line_itr.next().unwrap().trim().parse::<u32>().unwrap();

        moves.push((dir.to_string(), amt));
    }

    for (dir, a) in &moves {
        let mut amt = a.clone();
        let mut add_amt = dirs[0];
        match dir.as_str() {
            "R" => add_amt = dirs[0],
            "L" => add_amt = dirs[1],
            "U" => add_amt = dirs[2],
            "D" => add_amt = dirs[3],
            _ => assert!(false, "Invalid direction")
        }

        while amt > 0 {
            headpos.0 += add_amt.0;
            headpos.1 += add_amt.1;
            tailpos = move_tail(headpos, tailpos);
            tail_positions.entry(tailpos).and_modify(|e| {*e += 1}).or_insert(1);
            amt -= 1;
        }
    }

    let total_moves = tail_positions.len();
    println!("pt1: {total_moves}");

    let mut nodes = vec!((0,0); 10);
    tail_positions = HashMap::<(i32, i32), u32>::new();

    for (dir, a) in &moves {
        let mut amt = a.clone();
        let mut add_amt = dirs[0];
        match dir.as_str() {
            "R" => add_amt = dirs[0],
            "L" => add_amt = dirs[1],
            "U" => add_amt = dirs[2],
            "D" => add_amt = dirs[3],
            _ => assert!(false, "Invalid direction")
        }

        while amt > 0 {
            nodes[0].0 += add_amt.0;
            nodes[0].1 += add_amt.1;
            for i in 1..10 {
                nodes[i] = move_tail(nodes[i-1], nodes[i]);
            }
            tailpos = nodes[9];
            tail_positions.entry(tailpos).and_modify(|e| {*e += 1}).or_insert(1);
            amt -= 1;

            /* 
            for y in (0..7).rev() {
                for x in 0..7 {
                    let mut printed = false;
                    for n in 0..10 {
                        if nodes[n].0 == x && nodes[n].1 == y {
                            print!("{n}");
                            printed = true;
                            break;
                        }
                    }
                    if printed == false {
                        print!(".");
                    }
                }
                print!("\n");
            }
            println!("");
*/
            //io::stdout().flush().unwrap();
        }
    }

    let total_moves = tail_positions.len();
    println!("pt2: {total_moves}");

}