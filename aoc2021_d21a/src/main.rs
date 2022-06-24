// AoC 2021 day 21 part 1
//
// Pretty straightforward problem.  I enjoyed creating an iterator for the 100-sided die

struct RerollableDie {
    sides: u16,
    roll_count: u16,
    curval: u16,
}

impl RerollableDie {
    fn new(sides: u16) -> Self {
        Self {
            sides,
            roll_count: 0,
            curval: 0,
        }
    }

    fn roll3(&mut self) -> u16 {
        self.take(3).sum()
    }
}

impl Iterator for RerollableDie {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        self.curval += 1;
        let retval = self.curval;
        self.curval %= self.sides;
        self.roll_count += 1;
        Some(retval)
    }
}

struct Player {
    score: u16,
    position: u16,
}

impl Player {
    fn new(initpos: u16) -> Self {
        Self {
            score: 0,
            position: initpos,
        }
    }

    fn turn(&mut self, die: &mut RerollableDie) -> u16 {
        let v = die.roll3();
        self.position += v;
        self.position %= 10;
        if self.position == 0 {
            self.position = 10;
        }
        self.score += self.position;
        self.score
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut d = RerollableDie::new(100);
    let mut p1 = Player::new(6);
    let mut p2 = Player::new(1);
    let losers_score;

    loop {
        if p1.turn(&mut d) >= 1000 {
            losers_score = p2.score;
            break;
        }

        if p2.turn(&mut d) >= 1000 {
            losers_score = p1.score;
            break;
        }
    }
    println!("aoc21a: {}", losers_score as u32 * d.roll_count as u32);

    Ok(())
}
