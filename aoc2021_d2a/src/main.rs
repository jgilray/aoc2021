// AOC 2021 day 2

use regex::Regex;

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let re = Regex::new(r"^(\D+) (\d+)$").unwrap();
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    let mut depth2 = 0;
    let mut forward2 = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        if re.is_match(inputstr.trim()) {
            let caps = re.captures(inputstr.trim()).unwrap();
            let dir = caps.get(1).map_or("", |m| m.as_str());
            let amt = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());

            // part one
            match dir {
                "forward" => forward += amt,
                "down" => depth += amt,
                "up" => depth -= amt,
                _ => panic!("bad direction {}", dir),
            }

            // part two
            match dir {
                "forward" => {
                    forward2 += amt;
                    depth2 += aim * amt;
                }
                "down" => aim += amt,
                "up" => aim -= amt,
                _ => panic!("bad direction {}", dir),
            }
        } else {
            panic!("bad input data at line: {}", inputstr);
        }

        inputstr.clear();
    }
    println!("aoc2a: {}  aoc2b: {}", depth * forward, depth2 * forward2);

    Ok(())
}
