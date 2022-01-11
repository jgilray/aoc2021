// AOC 2021 day 3

fn sort_oxy_co2(vals: &[Vec<u8>], v: &mut Vec<usize>, target: u8, idx: usize) {
    if v.len() > 1 {
        let mut to_remove: Vec<usize> = vec![];
        for (ii, i) in v.iter().enumerate() {
            if vals[*i][idx] != target {
                to_remove.push(ii);
            }
        }

        while !to_remove.is_empty() {
            v.remove(to_remove.pop().unwrap());
        }
    }
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut v: Vec<usize> = vec![0; 20];
    let mut vals: Vec<Vec<u8>> = vec![];
    let mut num_lines = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        if num_lines == 0 {
            v.truncate(inputstr.trim_end().len());
        }

        let mut val: Vec<u8> = vec![];
        for (i, c) in inputstr.trim_end().chars().enumerate() {
            match c {
                '1' => v[i] += 1,
                '0' => {}
                _ => panic!("bad character {}", c),
            }
            val.push(c as u8 - b'0');
        }
        vals.push(val);

        num_lines += 1;
        inputstr.clear();
    }

    // part one
    let mut gamma = 0;
    let mut epsilon = 0;

    for count1 in &v {
        gamma <<= 1;
        epsilon <<= 1;
        if count1 >= &(num_lines - count1) {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!("aoc3a: {}", gamma * epsilon);

    // part two
    let mut oxygen_idx: Vec<usize> = (0..num_lines).collect();
    let mut co2_idx: Vec<usize> = (0..num_lines).collect();

    for i in 0..v.len() {
        let mut count1 = 0;
        for j in &oxygen_idx {
            if vals[*j][i] == 1 {
                count1 += 1;
            }
        }

        if count1 >= oxygen_idx.len() - count1 {
            sort_oxy_co2(&vals, &mut oxygen_idx, 1, i);
        } else {
            sort_oxy_co2(&vals, &mut oxygen_idx, 0, i);
        }

        count1 = 0;
        for j in &co2_idx {
            if vals[*j][i] == 1 {
                count1 += 1;
            }
        }

        if count1 >= co2_idx.len() - count1 {
            sort_oxy_co2(&vals, &mut co2_idx, 0, i);
        } else {
            sort_oxy_co2(&vals, &mut co2_idx, 1, i);
        }
    }

    let oxy_val = vals[oxygen_idx[0]]
        .iter()
        .fold(0, |s: usize, x| s * 2 + *x as usize);
    let co2_val = vals[co2_idx[0]]
        .iter()
        .fold(0, |s: usize, x| s * 2 + *x as usize);

    println!("aoc3b: {}", oxy_val * co2_val);

    Ok(())
}
