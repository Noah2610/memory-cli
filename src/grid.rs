use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::card::Card;
use crate::{Pos, CARDS_PER_ROW, CARD_PAIRS};

pub struct Grid {
    pub cards: HashMap<Pos, Card>,
    pub size:  (u16, u16),
}

impl Grid {
    pub fn new() -> Self {
        Self::generate_grid()
    }

    fn generate_grid() -> Self {
        let mut rng = rand::thread_rng();

        let mut shuffled_card_ids =
            (0 .. CARD_PAIRS)
                .into_iter()
                .fold(Vec::new(), |mut pairs, i| {
                    pairs.push(i);
                    pairs.push(i);
                    pairs
                });
        shuffled_card_ids.shuffle(&mut rng);

        let mut cards = HashMap::new();
        let mut x: u16 = 0;
        let mut y: u16 = 0;
        let mut size: (u16, u16) = (0, 0);
        for _ in 0 .. CARD_PAIRS * 2 {
            if x >= CARDS_PER_ROW {
                x = 0;
                y += 1;
            }

            if x >= size.0 {
                size.0 = x + 1;
            }
            if y >= size.1 {
                size.1 = y + 1;
            }

            match shuffled_card_ids.pop() {
                Some(id) => {
                    let pos = (x, y);
                    let card = Card(id);
                    cards.insert(pos, card);
                }
                None => {
                    panic!("Should have enough shuffled card ids for all cards")
                }
            }

            x += 1;
        }

        Grid { cards, size }
    }
}
