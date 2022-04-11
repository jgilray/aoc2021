// AOC 2021 day 17

const TARGET_XMIN: isize = 241;
const TARGET_XMAX: isize = 273;
const TARGET_YMIN: isize = -97;
const TARGET_YMAX: isize = -63;

// simulates the probe's path, returning max height reached (if it passes through target) or None
fn calc_path(x_init: isize, y_init: isize) -> Option<isize> {
    let mut x = 0_isize;
    let mut y = 0_isize;
    let mut y_highest = y;
    let mut xv = x_init;
    let mut yv = y_init;

    while x <= TARGET_XMAX && y >= TARGET_YMIN {
        x += xv;
        y += yv;
        if y > y_highest {
            y_highest = y;
        }

        if (TARGET_XMIN..=TARGET_XMAX).contains(&x) && (TARGET_YMIN..=TARGET_YMAX).contains(&y) {
            return Some(y_highest);
        }

        xv = match xv {
            isize::MIN..=-1 => xv + 1,
            1..=isize::MAX => xv - 1,
            _ => 0,
        };

        yv -= 1;
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut highest = 0_isize;
    let mut count = 0_usize;

    // try all possible trajectories
    for xi in 0..=TARGET_XMAX {
        for yi in TARGET_YMIN..=100 {
            if let Some(h) = calc_path(xi, yi) {
                count += 1;
                if h > highest {
                    highest = h;
                }
            }
        }
    }

    println!("aoc17a: {}, aoc17b: {}", highest, count);

    Ok(())
}
