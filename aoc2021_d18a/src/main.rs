// AoC 2021 day 18

#[derive(Debug, Clone)]
enum SFNum {
    Number(u64),
    Pair(Box<SFNum>, Box<SFNum>),
}

// "explode" the pair pointed at by the passed path
// to find prev number: up until went right, go left then right until get a number
// to find next number: up until went left, go right then left until get a number
fn explode(sfn: &SFNum, path: usize) -> Result<SFNum, String> {
    let mut retval = sfn.clone();
    let mut r = &mut retval;
    let level = 2_usize.pow((path as f64).log2().floor() as u32);
    let mut p = path - level; // remove path marker

    // find the pair at the passed path
    let mut lev = level / 2;
    while lev > 0 {
        match r {
            SFNum::Pair(left, right) => {
                if p >= lev {
                    r = &mut **right;
                    p -= lev;
                } else {
                    r = &mut **left;
                }
            }
            _ => return Err("path mismatches SFNum".to_string()),
        }
        lev /= 2;
    }

    // change the pair at the path to 0 after collecting its values
    let rhs;
    let lhs;
    match &r {
        SFNum::Pair(left, right) => {
            match (&**left, &**right) {
                (SFNum::Number(ll), SFNum::Number(rr)) => {
                    (lhs, rhs) = (*ll, *rr); // collect values
                    *r = SFNum::Number(0); // change pair to single zero
                }
                _ => return Err("path mismatches SFNum, expected SFNum::Numbers".to_string()),
            }
        }
        _ => return Err("path mismatches SFNum, expected SFNum::Pair".to_string()),
    }

    // find and change previous number - if it exists
    if path != level {
        r = &mut retval;
        let mut pp = 2 * (path - level - 1); // extra left at the end (in case pp = 0)
        lev = level;

        while lev > 0 {
            match r {
                SFNum::Pair(left, right) => {
                    if pp >= lev {
                        r = &mut **right;
                        match &r {
                            SFNum::Number(n) => {
                                *r = SFNum::Number(n + lhs); // change number to sum
                                break;
                            }
                            _ => pp -= lev,
                        }
                    } else {
                        r = &mut **left;
                        if let SFNum::Number(n) = &r {
                            *r = SFNum::Number(n + lhs); // change number to sum
                            break;
                        }
                    }
                }
                _ => return Err("path mismatches SFNum".to_string()),
            }
            lev /= 2;
        }
    }

    // find and change next number - if it exists
    if path != level * 2 - 1 {
        r = &mut retval;
        let mut pp = path - level + 1; // extra left at the end
        lev = level / 2;

        loop {
            if lev == 0 {
                lev = 1024; // keep going left until we find a SFNum::Number
            }
            match r {
                SFNum::Pair(left, right) => {
                    if pp >= lev {
                        r = &mut **right;
                        if let SFNum::Number(n) = &r {
                            *r = SFNum::Number(n + rhs); // change number to sum
                            break;
                        }
                        pp -= lev;
                    } else {
                        r = &mut **left;
                        if let SFNum::Number(n) = &r {
                            *r = SFNum::Number(n + rhs); // change number to sum
                            break;
                        }
                    }
                }
                _ => return Err("path mismatches SFNum".to_string()),
            }
            lev /= 2;
        }
    }

    Ok(retval)
}

// search the passed SFNum for SFNum::Pair(SFNum::Number, SFNum::Number)s and return
// a binary path to the leftmost one at a depth greater than 4 or None
// using a DFS so that the first found is the leftmost
// note that the path has a 1 marker on the left, so a path of l-l-l-r is 10001 or 17
fn find_leftmost_pair(sfn: &SFNum, depth: u32, path: usize) -> Option<usize> {
    match sfn {
        SFNum::Pair(left, right) => {
            let l = &**left;
            let r = &**right;

            match (l, r) {
                (SFNum::Number(_), SFNum::Number(_)) => {
                    if depth >= 4 {
                        Some(2_usize.pow(depth) + path)
                    } else {
                        None
                    }
                }

                (SFNum::Pair(_, _), SFNum::Pair(_, _)) => {
                    let pathl = find_leftmost_pair(l, depth + 1, path * 2);
                    if pathl.is_some() {
                        pathl
                    } else {
                        let pathr = find_leftmost_pair(r, depth + 1, path * 2 + 1);
                        if pathr.is_some() {
                            pathr
                        } else {
                            None
                        }
                    }
                }

                (SFNum::Pair(_, _), _) => find_leftmost_pair(l, depth + 1, path * 2),

                (_, SFNum::Pair(_, _)) => find_leftmost_pair(r, depth + 1, path * 2 + 1),
            }
        }

        SFNum::Number(_) => None,
    }
}

// search the passed SFNum for the leftmost SFNum::Number(n) where n >= 10, replace the number
// with a SFNum::Pair and return true.  If such a number is not found, return false
fn split(sfn: &mut SFNum) -> bool {
    match sfn {
        SFNum::Pair(left, right) => {
            if split(&mut **left) {
                true
            } else {
                split(&mut **right)
            }
        }

        SFNum::Number(n) => {
            if *n >= 10 {
                *sfn = SFNum::Pair(
                    Box::new(SFNum::Number(*n / 2)),
                    Box::new(SFNum::Number((*n + 1) / 2)),
                );
                true
            } else {
                false
            }
        }
    }
}

// recursively calculate the magnitude of a SFNum
fn calc_magnitude(sfn: &SFNum) -> u64 {
    match sfn {
        SFNum::Pair(left, right) => 3 * calc_magnitude(&**left) + 2 * calc_magnitude(&**right),

        SFNum::Number(n) => *n,
    }
}

fn reduce(sfn: &SFNum) -> Result<SFNum, String> {
    let mut done: bool = false;
    let mut nsfn = sfn.clone();

    while !done {
        let path = find_leftmost_pair(&nsfn, 0, 0);
        if path.is_some() {
            nsfn = explode(&nsfn, path.unwrap())?;
        } else {
            done = !split(&mut nsfn);
        }
    }

    Ok(nsfn)
}

fn parse_rest(citer: &mut std::str::Chars) -> Result<SFNum, String> {
    let mut comma_expected = true;
    let mut got_lhs = false;
    let mut got_rhs = false;
    let mut left_side = SFNum::Number(0);
    let mut right_side = SFNum::Number(0);
    let mut getting_number = false;
    let mut num: u64 = 0;
    let mut c = citer.next();

    while c.is_some() {
        match c.unwrap() {
            '0'..='9' => {
                num *= 10;
                num += c.unwrap() as u64 - '0' as u64;
                getting_number = true;
            }
            '[' => {
                if getting_number {
                    return Err("bad snailfish number: [ unexpected".to_string());
                }
                if comma_expected {
                    left_side = parse_rest(citer)?;
                    got_lhs = true;
                } else {
                    right_side = parse_rest(citer)?;
                    got_rhs = true;
                }
            }
            ',' => {
                if !comma_expected || (!got_lhs && !getting_number) {
                    return Err("bad snailfish number: , unexpected".to_string());
                }
                if getting_number {
                    left_side = SFNum::Number(num);
                    getting_number = false;
                    num = 0;
                }
                comma_expected = false;
            }
            ']' => {
                if comma_expected {
                    return Err("bad snailfish number: , missing?".to_string());
                } else if !got_rhs && !getting_number {
                    return Err("bad snailfish number: ] unexpected".to_string());
                }
                if getting_number {
                    return Ok(SFNum::Pair(
                        Box::new(left_side),
                        Box::new(SFNum::Number(num)),
                    ));
                } else {
                    return Ok(SFNum::Pair(Box::new(left_side), Box::new(right_side)));
                }
            }
            _ => return Err("unexpected char: ".to_string()),
        }
        c = citer.next();
    }

    Err("parse_rest failed, missing ] ?".to_string())
}

fn parse_sfnum(s: &str) -> Result<SFNum, String> {
    let mut getting_number = false;
    let mut num: u64 = 0;
    let mut citer = s.chars();
    let mut c = citer.next();

    while c.is_some() {
        match c.unwrap() {
            '0'..='9' => {
                num *= 10;
                num += c.unwrap() as u64 - '0' as u64;
                getting_number = true;
            }
            '[' => {
                if getting_number {
                    return Err("bad snailfish number, [ follows number: ".to_string() + s);
                }
                let sfn = parse_rest(&mut citer)?;
                if citer.next().is_some() {
                    return Err("extra characters after snailfish number".to_string());
                } else {
                    return Ok(sfn);
                }
            }
            _ => return Err("bad character in snailfish number: ".to_string() + s),
        }
        c = citer.next();
    }

    if getting_number {
        Ok(SFNum::Number(num))
    } else {
        Err("empty snailfish number".to_string())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut sfn = SFNum::Number(0);
    let mut got_first: bool = false;
    let mut vsfn: Vec<SFNum> = vec![];

    while reader.read_line(&mut inputstr)? != 0 {
        let addend = inputstr.trim();
        vsfn.push(parse_sfnum(addend)?); // for part 2
        if !got_first {
            sfn = parse_sfnum(addend)?;
            got_first = true;
        } else {
            // calculate sfn + addend, leaving the result in sfn
            sfn = SFNum::Pair(Box::new(sfn), Box::new(parse_sfnum(addend)?));
        }
        sfn = reduce(&sfn)?;

        inputstr.clear();
    }
    println!("aoc18a: {}", calc_magnitude(&sfn));

    let mut max_magnitude = 0;
    for i in 0..vsfn.len() {
        for j in i..vsfn.len() {
            let mut add = SFNum::Pair(Box::new(vsfn[i].clone()), Box::new(vsfn[j].clone()));
            add = reduce(&add)?;
            let mag = calc_magnitude(&add);
            if mag > max_magnitude {
                max_magnitude = mag;
            }
            add = SFNum::Pair(Box::new(vsfn[j].clone()), Box::new(vsfn[i].clone()));
            add = reduce(&add)?;
            let mag = calc_magnitude(&add);
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }
    println!("aoc18b: {}", max_magnitude);

    Ok(())
}
