// AoC 2021 day 10

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut err_score: usize = 0;
    let mut comp_scores: Vec<usize> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let mut stack: Vec<char> = vec![];
        let mut error_found = false;
        for c in inputstr.trim().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if !stack.is_empty() && *stack.last().unwrap() == '(' {
                        stack.pop();
                    } else {
                        err_score += 3;
                        error_found = true;
                        break;
                    }
                }
                ']' => {
                    if !stack.is_empty() && *stack.last().unwrap() == '[' {
                        stack.pop();
                    } else {
                        err_score += 57;
                        error_found = true;
                        break;
                    }
                }
                '}' => {
                    if !stack.is_empty() && *stack.last().unwrap() == '{' {
                        stack.pop();
                    } else {
                        err_score += 1197;
                        error_found = true;
                        break;
                    }
                }
                '>' => {
                    if !stack.is_empty() && *stack.last().unwrap() == '<' {
                        stack.pop();
                    } else {
                        err_score += 25137;
                        error_found = true;
                        break;
                    }
                }
                _ => panic!("bad char found {}", c),
            }
        }

        // part two
        if !error_found {
            let mut completion_score = 0;
            while !stack.is_empty() {
                let s = match stack.pop().unwrap() {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => {
                        panic!("bad char")
                    }
                };
                completion_score *= 5;
                completion_score += s;
            }
            comp_scores.push(completion_score);
        }

        inputstr.clear();
    }

    println!("aoc10a: {}", err_score);

    comp_scores.sort_unstable();
    let mid_idx = comp_scores.len() / 2;
    println!("aoc10b: {}", comp_scores[mid_idx]);

    Ok(())
}
