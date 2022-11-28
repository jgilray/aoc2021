// AOC 2021 day 25

// location on the sea floor
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

#[derive(Debug, Clone, Copy)]
struct SeaCucumber {
    east_moving: bool,
    can_move: bool,
    loc: Point,
    next_loc: Point,
}

impl SeaCucumber {
    fn new(east_moving: bool, loc: Point) -> Self {
        Self {
            east_moving,
            can_move: false,
            loc,
            next_loc: loc,
        }
    }

    // returns false if the Sea cucumber cannot move
    fn mark_if_moveable(&mut self, floor: &mut Vec<Vec<bool>>) -> bool {
        if self.east_moving {
            if self.loc.x == floor[self.loc.y].len() - 1 {
                if !floor[self.loc.y][0] {
                    self.next_loc.x = 0;
                    self.can_move = true;
                } else {
                    self.next_loc.x = self.loc.x;
                    self.can_move = false;
                }
            } else if !floor[self.loc.y][self.loc.x + 1] {
                self.next_loc.x = self.loc.x + 1;
                self.can_move = true;
            } else {
                self.next_loc.x = self.loc.x;
                self.can_move = false;
            }
        } else {
            // south goer
            if self.loc.y == floor.len() - 1 {
                if !floor[0][self.loc.x] {
                    self.next_loc.y = 0;
                    self.can_move = true;
                } else {
                    self.next_loc.y = self.loc.y;
                    self.can_move = false;
                }
            } else if !floor[self.loc.y + 1][self.loc.x] {
                self.next_loc.y = self.loc.y + 1;
                self.can_move = true;
            } else {
                self.next_loc.y = self.loc.y;
                self.can_move = false;
            }
        }

        self.can_move
    }

    // should always call check_moveable before calling this function
    fn move_one_step(&mut self, floor: &mut [Vec<bool>]) {
        if self.can_move {
            floor[self.loc.y][self.loc.x] = false;
            floor[self.next_loc.y][self.next_loc.x] = true;
            self.loc = self.next_loc;
        }
    }
}

// move a tribe of Sea cucumbers
fn move_group(sc_group: &mut [SeaCucumber], floor: &mut Vec<Vec<bool>>) -> bool {
    let mut one_can_move = false;

    for sc in sc_group.iter_mut() {
        if sc.mark_if_moveable(floor) {
            one_can_move = true;
        }
    }

    for sc in sc_group.iter_mut() {
        sc.move_one_step(floor);
    }

    one_can_move
}

// goes though a step in the movement of the Sea cucumbers on the ocean floor, returns true
// if any Sea cucumber moved during the step
fn step(eg: &mut Vec<SeaCucumber>, sg: &mut Vec<SeaCucumber>, floor: &mut Vec<Vec<bool>>) -> bool {
    // move east goers before south goers
    let eg_movement = move_group(eg, floor);
    let sg_movement = move_group(sg, floor);

    eg_movement || sg_movement
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../sea_cucumbers.dat");
    let mut east_goers: Vec<SeaCucumber> = vec![];
    let mut south_goers: Vec<SeaCucumber> = vec![];
    let mut col = 0_usize;

    // sea_floor
    let mut sea_floor = input
        .lines()
        .map(|line| {
            let mut v: Vec<bool> = vec![true; line.len()];
            for (row, ch) in line.chars().enumerate() {
                match ch {
                    '>' => east_goers.push(SeaCucumber::new(true, Point::new(col, row))),
                    'v' => south_goers.push(SeaCucumber::new(false, Point::new(col, row))),
                    _ => v[row] = false,
                }
            }
            col += 1;
            v
        })
        .collect::<Vec<Vec<bool>>>();

    for i in 1.. {
        let moved = step(&mut east_goers, &mut south_goers, &mut sea_floor);
        if !moved {
            println!("aoc25a: {}", i);
            break;
        }
    }

    Ok(())
}
