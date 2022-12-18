use std::io;
use std::collections::HashMap;

struct FsNode {
    size: u64,
    childs: Vec<String>,
}

fn dfs_dir(dir: &FsNode, node_map: &HashMap<String, FsNode>) -> u64 {
    let mut total: u64 = 0;
    for child in &dir.childs {
        let node = &node_map[child];
        if node.size == 0 {
            total += dfs_dir(&node, node_map);
        } else {
            assert!(node.size > 0);
            total += node.size;
        }
    }

    return total;
}

fn create_full_path(dir_stack: &Vec<String>, name: &str) -> String {
    let full_path = dir_stack.join("-") + "-" + name;
    return full_path;
}

fn main() {
    let mut total: u64 = 0;
    let mut del_candidate: u64 = u64::MAX;

    let mut node_map: HashMap<String, FsNode> = HashMap::new();
    let mut dir_stack: Vec<String> = Vec::new();

    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let words: Vec<&str> = line.trim().split(' ').collect();

        if words[0] == "$" {
            if words[1] == "cd" {
                if words[2] == ".." {
                    dir_stack.pop();
                } else {
                    let full_path = create_full_path(&dir_stack, words[2]);
                    if node_map.contains_key(&full_path) == false {
                        let n = FsNode { 
                            size: 0,
                            childs: Vec::new(),
                        };
                        node_map.insert(full_path.to_string(), n);
                    }
                    dir_stack.push(full_path.to_string());
                }
            } 
            // else skip ls
            // TODO: is this ok?
        } else if words[0] == "dir" {
            let name = words[1];
            if node_map.contains_key(name) == false {
                let n = FsNode { 
                    size: 0,
                    childs: Vec::new(),
                };
                let full_path = create_full_path(&dir_stack, &name);
                node_map.insert(full_path.to_string(), n);
                node_map.entry(dir_stack.last().unwrap().to_string()).and_modify(|e| {
                    e.childs.push(full_path.to_string());
                });
            } else {
                assert!(false, "dir with same name found");
            }
        } else {
            match words[0].trim().parse::<u64>() {
                Ok(n) => {
                    let name = words[1];
                    let full_path = create_full_path(&dir_stack, name);
                    let node = FsNode {
                        size: n,
                        childs: Vec::new(),
                    };
                    node_map.insert(full_path.to_string(), node);
                    // todo: is this the right way?...
                    node_map.entry(dir_stack.last().unwrap().to_string()).and_modify(|e| {
                        e.childs.push(full_path.to_string());
                    });
                },
                Err(..) => continue
            }
        }
    }

    let root_size = dfs_dir(&node_map["-/"],  &node_map);
    let size_needed = 30000000 - (70000000 - root_size);

    for (_, dir) in &node_map {
        let size = dfs_dir(&dir, &node_map);
        if size <= 100000 {
            total += size;
        }

        if size > size_needed && size < del_candidate {
            del_candidate = size
        }
    }

    println!("pt1: {total}");
    println!("pt2: {del_candidate}");
}