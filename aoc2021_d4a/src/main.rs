use std::collections::HashSet;

#[derive(Debug)]
struct BingoCard {
    card: Vec<Vec<(u32, bool)>>,
}

impl BingoCard {
    fn new() -> Self {
        Self { card: vec![] }
    }

    // function to mark the board with the passed number.  If the mark results in a bingo then true is returned
    fn mark_and_check(&mut self, num: u32) -> bool {
        let mut ccopy = self.card.clone();
        for (y, v) in self.card.iter_mut().enumerate() {
            for (x, (n, m)) in v.iter_mut().enumerate() {
                if *n == num {
                    *m = true; // mark
                    ccopy[y][x].1 = true; // mark clone as well

                    // if the marked row or column is full return true otherwise return false
                    return (0..5).map(|xx| ccopy[y][xx].1).all(|b| b)
                        || (0..5).map(|yy| ccopy[yy][x].1).all(|b| b);
                }
            }
        }

        false // num not found on card
    }

    fn calc_sum_of_unmarked(&self) -> u32 {
        let mut sum = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.card[y][x].1 {
                    sum += self.card[y][x].0;
                }
            }
        }
        sum
    }
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut first_line = true;
    let mut caller: Vec<u32> = vec![];
    let mut cards: Vec<BingoCard> = vec![];
    let mut card_idx = 0;

    while reader.read_line(&mut inputstr)? != 0 {
        if first_line {
            caller = inputstr
                .trim()
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            first_line = false;
        } else {
            let input = inputstr.trim();
            if input.is_empty() {
                cards.push(BingoCard::new());
            } else {
                cards[card_idx].card.push(
                    input
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| (s.parse::<u32>().unwrap(), false))
                        .collect(),
                );
                if cards[card_idx].card.len() == 5 {
                    card_idx += 1;
                }
            }
        }

        inputstr.clear();
    }

    let mut winner_indices: HashSet<usize> = HashSet::new();

    'lp: for n in &caller {
        for (idx, bc) in cards.iter_mut().enumerate() {
            if winner_indices.contains(&idx) {
                continue;
            } else if bc.mark_and_check(*n) {
                if winner_indices.is_empty() {
                    println!("aoc4a: {}", bc.calc_sum_of_unmarked() * n);
                } else if winner_indices.len() == card_idx - 1 {
                    println!("aoc4b: {}", bc.calc_sum_of_unmarked() * n);
                    break 'lp;
                }
                winner_indices.insert(idx);
            }
        }
    }

    Ok(())
}
