// AoC 2021 day 21 part 2
//
// recursive state search
// runs in 0.5s without memoization, which actually slows it down about 60%

#[derive(Clone)]
struct GameState {
    p1_loc: u8,
    p2_loc: u8,
    p1_score: u8,
    p2_score: u8,
    p1_turn: bool,
}

impl GameState {
    fn new(init_p1_loc: u8, init_p2_loc: u8) -> Self {
        Self {
            p1_loc: init_p1_loc,
            p2_loc: init_p2_loc,
            p1_score: 0,
            p2_score: 0,
            p1_turn: true,
        }
    }
}

// recursive function that returns (number of p1 winners, number of p2 winners)
fn count(gs: &mut GameState) -> (u64, u64) {
    let mut p1_winners = 0;
    let mut p2_winners = 0;

    if gs.p1_turn {
        if gs.p2_score >= 21 {
            return (0, 1);
        }
    } else if gs.p1_score >= 21 {
        return (1, 0);
    }

    for roll in 3..=9 {
        let ways = match roll {
            3 => 1,
            4 => 3,
            5 => 6,
            6 => 7,
            7 => 6,
            8 => 3,
            _ => 1,
        };

        let mut next_gs = gs.clone();
        if gs.p1_turn {
            let mut nloc = gs.p1_loc + roll;
            nloc %= 10;
            if nloc == 0 {
                nloc = 10;
            }
            next_gs.p1_loc = nloc;
            next_gs.p1_score += nloc;
            next_gs.p1_turn = false;
        } else {
            let mut nloc = gs.p2_loc + roll;
            nloc %= 10;
            if nloc == 0 {
                nloc = 10;
            }
            next_gs.p2_loc = nloc;
            next_gs.p2_score += nloc;
            next_gs.p1_turn = true;
        }

        let (p1_sub_wins, p2_sub_wins) = count(&mut next_gs);
        p1_winners += p1_sub_wins * ways;
        p2_winners += p2_sub_wins * ways;
    }

    (p1_winners, p2_winners)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = GameState::new(6, 1);
    let (p1_final_score, p2_final_score) = count(&mut state);
    let winners_score = std::cmp::max(p1_final_score, p2_final_score);

    println!("aoc21b: {}", winners_score);

    Ok(())
}
