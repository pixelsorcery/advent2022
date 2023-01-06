use std::io;

fn in_order(a: &Vec<char>, b: &Vec<char>) -> bool {

    let mut leftitr = a.iter().peekable();
    let mut rightitr = b.iter().peekable();

    let mut parens: i32 = 0;
    while leftitr.peek().is_some() && rightitr.peek().is_some() {
        let mut left = leftitr.next().unwrap();
        let mut right = rightitr.next().unwrap();

        //check for paranthesis, increment if left is a list
        while *left == '[' && right.is_digit(10) {
            parens += 1;
            left = leftitr.next().unwrap();
        }

        while *right == '[' && left.is_digit(10) {
            parens -= 1;
            right = rightitr.next().unwrap();
        }
        if left.is_digit(10) && right.is_digit(10) {
            let mut left = left.to_digit(10).unwrap();
            let mut right = right.to_digit(10).unwrap();
            if left == 1 && leftitr.peek().unwrap().is_digit(10) {
                left = 10;
                leftitr.next();
            }
            if right == 1 && rightitr.peek().unwrap().is_digit(10) {
                right = 10;
                rightitr.next();
            }

            if left < right {return true;}
            if left > right {return false;}

            if parens != 0 {
                // close out the lists
                while parens > 0 {
                    if **leftitr.peek().unwrap() == ']' {
                        leftitr.next();
                        parens -= 1;
                    } else {
                        return false;
                    }
                }
                
                while parens < 0 {
                    if **rightitr.peek().unwrap() == ']' {
                        rightitr.next();
                        parens += 1;
                    } else {
                        return true;
                    }
                }
            }
        }

        if *left == *right {continue;}

        // if left runs out first return true
        if *left == ']' {return true;}
        if *right == ']' {return false;}
    }

    return false;
}

fn main() {
    let mut left: Vec<(i32, i32, i32)> = Vec::new();
    let mut right: Vec<(i32, i32, i32)> = Vec::new();
    let mut listidx = 1;
    let mut result = 0;
    let div2 = vec!('[','[','2',']',']');
    let div6 = vec!('[','[','6',']',']');
    let mut div2idx = 1;
    let mut div6idx = 1;

    loop {
        left.clear();
        right.clear();
        let mut line1 = String::new();
        let bytes = io::stdin()
            .read_line(&mut line1)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let chars1: Vec<char> = line1.trim().chars().collect();
        let mut line2 = String::new();
        let bytes = io::stdin()
            .read_line(&mut line2)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let chars2: Vec<char> = line2.trim().chars().collect();

        if in_order(&chars1, &chars2) {
            result += listidx;
        }

        if in_order(&chars1, &div2) {div2idx += 1;}
        if in_order(&chars1, &div6) {div6idx += 1;}
        if in_order(&chars2, &div2) {div2idx += 1;}
        if in_order(&chars2, &div6) {div6idx += 1;}      
        listidx += 1;

        let bytes = io::stdin()
        .read_line(&mut line1)
        .expect("Failed to read line");
        if bytes == 0 {
            break;
        }
    }

    println!("pt1: {result}");
    println!("pt2: {}", div2idx * (div6idx+1));
}
