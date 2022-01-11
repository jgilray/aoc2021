// AOC 2021 day 1

fn count_increases(v: &[usize]) -> usize {
    let mut incr = 0;

    let mut last_d = usize::MAX;
    for d in v {
        if *d > last_d {
            incr += 1;
        }
        last_d = *d;
    }

    incr
}

fn main() {
    let input = include_str!("../../depth.dat");

    let v: Vec<usize> = input.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    println!("aoc1a: {}", count_increases(&v));

    // part two
    let vwindow: Vec<usize> = v.windows(3).map(|s| s.iter().sum()).collect();
    println!("aoc1b: {}", count_increases(&vwindow));
}
