// AoC 2021 day 6

// perform one step of the lantern fish simulation
fn one_step(v: &[usize]) -> Vec<usize> {
    let mut next_v: Vec<_> = vec![0; v.len()];
    for i in 0..v.len() {
        if i == 0 {
            next_v[6] += v[i];
            next_v[8] += v[i];
        } else {
            next_v[i - 1] += v[i];
        }
    }
    next_v
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut timers: Vec<usize> = vec![0; 9];

    while reader.read_line(&mut inputstr)? != 0 {
        let iter = inputstr.trim().split(',');
        for s in iter {
            let timer = s.parse::<usize>().unwrap();
            timers[timer] += 1;
        }

        inputstr.clear();
    }

    for _ in 0..80 {
        timers = one_step(&timers);
    }

    let ans: usize = timers.iter().sum();
    println!("aoc6a: {}", ans);

    for _ in 80..256 {
        timers = one_step(&timers);
    }

    let ans: usize = timers.iter().sum();
    println!("aoc6b: {}", ans);

    Ok(())
}
