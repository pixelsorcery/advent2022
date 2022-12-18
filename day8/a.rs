use std::io;

fn get_idx(x: u32, y: u32, stride: u32) -> u32 {
    y * stride + x
}

fn calc_scenic_score(x: u32, y: u32, stride: u32, vals: &Vec<u32>) -> u32 {
    let incrs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut scores = [0, 0, 0, 0];
    let idx = get_idx(x as u32, y as u32, stride) as usize;
    let tree_height = vals[idx];
    for i in 0..4 {
        let mut xval = x as i32;
        let mut yval = y as i32;
        let istride = stride as i32;
        xval += incrs[i].0;
        yval += incrs[i].1;
        while xval >= 0 && yval >= 0 && xval < istride && yval < istride {
            scores[i] += 1;
            let idx = get_idx(xval as u32, yval as u32, stride) as usize;
            if tree_height <= vals[idx] {
                break;
            } 
            xval += incrs[i].0;
            yval += incrs[i].1;
        }
    }
    let mut score = 1;
    for s in scores {
        score *= s;
    }
    return score;
}

fn main() {
    let mut all_lines = "".to_string();
    let mut stride = 0;
    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }
        stride = line.trim().len() as u32;
        all_lines += &line.trim();
    }

    let vals: Vec<u32> = all_lines.chars().map(|c| {(c as u32) - ('0' as u32)}).collect();
    let mut vis: Vec<u32> = vec![0; vals.len()];
    // x dir
    let mut total_vis = 4;
    for y in 1..stride-1 {

        let idx = get_idx(0, y as u32, stride) as usize;
        let mut max = vals[idx];
        vis[idx] = 1;
        for x in 1..stride-1 {
            let idx = get_idx(x as u32, y as u32, stride) as usize;
            if vals[idx] > max {
                vis[idx] = 1;
                max = vals[idx];
            }
        }
    }

    for y in 1..stride-1 {

        let idx = get_idx(stride-1 as u32, y as u32, stride) as usize;
        let mut max = vals[idx];
        vis[idx] = 1;

        for x in (1..stride-1).rev(){
            let idx = get_idx(x as u32, y as u32, stride) as usize;
            if vals[idx] > max {
                vis[idx] = 1;
                max = vals[idx];
            }
        }
    }

    for x in 1..stride-1 {

        let idx = get_idx(x as u32, 0, stride) as usize;
        let mut max = vals[idx];
        vis[idx] = 1;

        for y in 1..stride-1 {
            let idx = get_idx(x as u32, y as u32, stride) as usize;
            if vals[idx] > max {
                vis[idx] = 1;
                max = vals[idx];
            }
        }
    }

    for x in 1..stride-1 {

        let idx = get_idx(x as u32, stride-1 as u32, stride) as usize;
        let mut max = vals[idx];
        vis[idx] = 1;

        for y in (1..stride-1).rev() {
            let idx = get_idx(x as u32, y as u32, stride) as usize;
            if vals[idx] > max {
                vis[idx] = 1;
                max = vals[idx];
            }
        }
    }

    for val in &vis {
        total_vis += val;
    }

    println!("pt1: {total_vis}");

    let mut scenic_score = 0;
    for y in 0..stride {
        for x in 0..stride {
            let score = calc_scenic_score(x as u32, y as u32, stride as u32, &vals);
            if score > scenic_score {
                scenic_score = score;
            }
        }
    }

    println!("pt2: {scenic_score}");
}