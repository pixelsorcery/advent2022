use std::io;
use std::collections::HashSet;

fn merge_ranges(ranges: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|r| r.0);

    let mut current_range = ranges[0];
    for &(start, end) in ranges.iter().skip(1) {
        if start > current_range.1 {
            result.push(current_range);
            current_range = (start, end);
        } else {
            current_range.1 = current_range.1.max(end);
        }
    }
    result.push(current_range);

    result
}

// find how many tiles it overlaps for the given height
fn calc_overlap_range(s:(i32, i32), range: i32, ypos: i32) -> i32 {
    let dy = (s.1 - ypos).abs();
    range - dy
}

fn overlaps(s: (i32, i32), r: i32, t: i32) -> bool {
    s.1 <= t && s.1 + r >= t || s.1 >= t && s.1 - r <= t
}

fn find_range(s: (i32, i32), b: (i32, i32)) -> i32 {
    let dy = (b.0 - s.0).abs();
    let dx = (b.1 - s.1).abs();

    dx+dy
}

fn main() {
    let mut sensors:Vec<(i32, i32)> = Vec::new();
    let mut beacons:Vec<(i32, i32)> = Vec::new();
    //let target_row = 2000000;
    let target_row = 10;
    let multiplier = 4000000;

    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let parts: Vec<&str> = line.trim().split(":").collect();
        let res1 = parts[0].split(", ").map(|s| s.split("=").nth(1).unwrap()).collect::<Vec<&str>>();
        let res2 = parts[1].split(", ").map(|s| s.split("=").nth(1).unwrap()).collect::<Vec<&str>>();

        let s: (i32, i32) = (res1[0].parse().unwrap(), res1[1].parse().unwrap());
        let b: (i32, i32) = (res2[0].parse().unwrap(), res2[1].parse().unwrap());

        sensors.push(s);
        beacons.push(b);
    }

    let mut ranges:Vec<(i32, i32)> = Vec::new();

    for i in 0..sensors.len() {
        let range = find_range(sensors[i], beacons[i]);
        let o = overlaps(sensors[i], range, target_row);
        if o {
            let startx = sensors[i].0;

            let overlapamt = calc_overlap_range(sensors[i], range, target_row);

            ranges.push((startx - overlapamt, startx + overlapamt));
        }
    }

    let result = merge_ranges(&ranges);
    let range = result[0].1 - result[0].0 + 1;

    let mut beaconsinrow: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..beacons.len() {
        if beacons[i].1 == target_row {
            beaconsinrow.insert(beacons[i]);
        }
    }


    println!("pt1: {}",  range - beaconsinrow.len() as i32);


    let mut finalcoordinate:(i32, i32) = (0, 0);

    for k in 0..=multiplier {

        let mut ranges:Vec<(i32, i32)> = Vec::new();

        if k % 1000000 == 0 {
            println!("{k}");
        }

        for i in 0..sensors.len() {
            let range = find_range(sensors[i], beacons[i]);
            let o = overlaps(sensors[i], range, k);
            if o {
                let startx = sensors[i].0;

                let overlapamt = calc_overlap_range(sensors[i], range, k);

                ranges.push((startx - overlapamt, startx + overlapamt));
            }
        }

        let merged_ranges = merge_ranges(&ranges);
        if merged_ranges.len() == 2 {
            println!("vals are {} {} {} {}", merged_ranges[0].0, merged_ranges[0].1,
            merged_ranges[1].0, merged_ranges[1].1);
            finalcoordinate = (merged_ranges[0].1 + 1, k);
            break;
        }
    }

    let part2: i64 = finalcoordinate.0 as i64 * multiplier as i64 + finalcoordinate.1 as i64;
    println!("pt2: {}", part2);
}
