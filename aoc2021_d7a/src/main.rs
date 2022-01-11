// AoC 2021 day 7

// calculate fuel cost to realign all crabs to pos in part 1
fn realignment_cost1(v: &[u32], pos: u32) -> u32 {
    v.iter()
        .map(|p| if *p < pos { pos - p } else { p - pos })
        .sum()
}

// calculate fuel cost to realign all crabs to pos in part 2
fn realignment_cost2(v: &[u32], pos: u32) -> u32 {
    v.iter()
        .map(|p| if *p < pos { pos - p } else { p - pos })
        .map(|n| n * (n + 1) / 2)
        .sum()
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut hor_pos: Vec<u32> = vec![];
    let mut largest = 0;
    let mut smallest = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        let iter = inputstr.trim().split(',');

        for s in iter {
            let pos = s.parse::<u32>().expect("bad horizontal position");
            hor_pos.push(pos);
            if pos < smallest {
                smallest = pos;
            }
            if pos > largest {
                largest = pos;
            }
        }

        inputstr.clear();
    }

    // part one
    let mut least_fuel = u32::MAX;
    for p in smallest..largest {
        let fuel = realignment_cost1(&hor_pos, p);
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    println!("aoc7a: {}", least_fuel);

    // part two
    least_fuel = u32::MAX;
    for p in smallest..largest {
        let fuel = realignment_cost2(&hor_pos, p);
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    println!("aoc7b: {}", least_fuel);

    Ok(())
}
