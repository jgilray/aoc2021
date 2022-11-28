// AOC 2021 day 24

use std::collections::HashMap;

// custom error type
#[derive(Debug)]
struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Inp(usize),
    AddReg(usize, usize),
    AddDirect(usize, isize),
    MulReg(usize, usize),
    MulDirect(usize, isize),
    DivReg(usize, usize),
    DivDirect(usize, isize),
    ModReg(usize, usize),
    ModDirect(usize, isize),
    EqlReg(usize, usize),
    EqlDirect(usize, isize),
}

fn parse_instruction(s: &str) -> Result<Instr, String> {
    let v = s.split(' ').collect::<Vec<_>>();
    if v.len() < 2 || v.len() > 3 {
        return Err("Bad ALU instruction".to_string());
    }

    // check v[1] register
    let reg1: usize = match v[1] {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => return Err("Bad Register specification: first argument".to_string()),
    };

    if v.len() > 2 {
        let mut is_direct = false;
        let second: isize = match v[2] {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => {
                let retval;
                if let Ok(r2) = v[2].parse::<isize>() {
                    is_direct = true;
                    retval = r2;
                } else {
                    return Err("Bad Register specification: second argument".to_string());
                }
                retval
            }
        };

        if is_direct {
            match v[0] {
                "add" => Ok(Instr::AddDirect(reg1, second)),
                "mul" => Ok(Instr::MulDirect(reg1, second)),
                "div" => Ok(Instr::DivDirect(reg1, second)),
                "mod" => Ok(Instr::ModDirect(reg1, second)),
                "eql" => Ok(Instr::EqlDirect(reg1, second)),
                _ => Err("bad 2 argument direct opcode".to_string()),
            }
        } else {
            match v[0] {
                "add" => Ok(Instr::AddReg(reg1, second as usize)),
                "mul" => Ok(Instr::MulReg(reg1, second as usize)),
                "div" => Ok(Instr::DivReg(reg1, second as usize)),
                "mod" => Ok(Instr::ModReg(reg1, second as usize)),
                "eql" => Ok(Instr::EqlReg(reg1, second as usize)),
                _ => Err("bad 2 argument register opcode".to_string()),
            }
        }
    } else if v[0] == "inp" {
        return Ok(Instr::Inp(reg1));
    } else {
        return Err("bad 1 argument opcode".to_string());
    }
}

// runs the passed program, returning the value of Register z when the program terminates
fn run_program(vp: &[Instr], zval: isize, input: isize) -> Option<isize> {
    // the registers w - z
    let mut regs: Vec<isize> = vec![0, 0, 0, zval];

    // simulate the program
    for instr in vp {
        match *instr {
            Instr::Inp(ridx) => {
                regs[ridx] = input;
            }
            Instr::AddReg(ridx, r2idx) => regs[ridx] += regs[r2idx],
            Instr::AddDirect(ridx, val) => regs[ridx] += val,
            Instr::MulReg(ridx, r2idx) => regs[ridx] *= regs[r2idx],
            Instr::MulDirect(ridx, val) => regs[ridx] *= val,
            Instr::DivReg(ridx, r2idx) => regs[ridx] /= regs[r2idx],
            Instr::DivDirect(ridx, val) => regs[ridx] /= val,
            Instr::ModReg(ridx, r2idx) => regs[ridx] %= regs[r2idx],
            Instr::ModDirect(ridx, val) => regs[ridx] %= val,
            Instr::EqlReg(ridx, r2idx) => {
                if regs[ridx] == regs[r2idx] {
                    regs[ridx] = 1;
                } else {
                    regs[ridx] = 0;
                }
            }
            Instr::EqlDirect(ridx, val) => {
                if regs[ridx] == val {
                    regs[ridx] = 1;
                } else {
                    regs[ridx] = 0;
                }
            }
        }
    }

    Some(regs[3]) // register z
}

struct Solver {
    digit_progs: Vec<Vec<Instr>>,
    cache: HashMap<(usize, isize), Option<isize>>,
    solve_max: bool,
}

impl Solver {
    fn num_digits(&self) -> usize {
        self.digit_progs.len()
    }

    fn recursive_search(&mut self, ndigit: usize, prevz: isize) -> Option<isize> {
        if ndigit >= self.num_digits() {
            if prevz == 0 {
                return Some(0);
            }

            return None;
        }

        // memoization
        if let Some(&cached) = self.cache.get(&(ndigit, prevz)) {
            return cached;
        }

        for i in 1..=9 {
            let ii = if self.solve_max { 10 - i } else { i };

            let nextz = run_program(&self.digit_progs[ndigit], prevz, ii)?;

            if let Some(best_suffix) = self.recursive_search(ndigit + 1, nextz) {
                let exp = self.num_digits() - ndigit - 1;
                let new_suffix = 10_isize.pow(exp as u32) * ii + best_suffix;

                self.cache.insert((ndigit, prevz), Some(new_suffix));
                return Some(new_suffix);
            }
        }

        self.cache.insert((ndigit, prevz), None);
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../alu_instructions.dat");

    // idiom: An iterator over Result<T, E> can be collect()-ed directly into a Result<Vec<T>, E>
    let ivec = input
        .lines()
        .map(|line| parse_instruction(line))
        .collect::<Result<Vec<_>, String>>()?;

    // break the program into 14 separate chunks, one for each input digit
    let mut digit_progs = Vec::new();
    let mut ivec_iter = ivec.iter();
    let num_digits: usize = 14;
    for _ in 0..num_digits {
        let mut dprog: Vec<Instr> = Vec::new();
        for _ in 0..(ivec.len() / num_digits) {
            dprog.push(*ivec_iter.next().unwrap());
        }
        digit_progs.push(dprog);
    }

    let mut solver = Solver {
        digit_progs: digit_progs.clone(),
        cache: HashMap::new(),
        solve_max: true,
    };

    if let Some(res) = solver.recursive_search(0, 0) {
        println!("aoc24a: {}", res);
    } else {
        return Err(Box::new(Error::new("failed to find max solution")));
    }

    let mut solver = Solver {
        digit_progs,
        cache: HashMap::new(),
        solve_max: false,
    };

    if let Some(res) = solver.recursive_search(0, 0) {
        println!("aoc24b: {}", res);
    } else {
        return Err(Box::new(Error::new("failed to find min solution")));
    }

    Ok(())
}
