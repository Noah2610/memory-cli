extern crate rand;
extern crate termion;

use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;

const CARD_PAIRS: usize = 6;
const CARDS_PER_ROW: usize = 4;
// const ROWS: usize = (CARDS / CARDS_PER_ROW) + (CARDS % CARDS_PER_ROW).min(1);

struct Card {
    pub id: usize,
}

impl From<usize> for Card {
    fn from(id: usize) -> Self {
        Card { id }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.id {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'X',
            4 => 'Y',
            5 => '#',
            6 => '@',
            7 => 'O',
            8 => 'E',
            _ => '-',
        };
        write!(f, "{}", s)
    }
}

type Pos = (usize, usize);

struct Grid {
    cards: HashMap<Pos, Card>,
}

impl Grid {
    pub fn new() -> Self {
        let cards = Self::generate_cards();
        Self { cards }
    }

    fn generate_cards() -> HashMap<Pos, Card> {
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
        let mut x = 0;
        let mut y = 0;
        for _ in 0 .. CARD_PAIRS * 2 {
            if x >= CARDS_PER_ROW {
                x = 0;
                y += 1;
            }

            match shuffled_card_ids.pop() {
                Some(id) => {
                    let pos = (x, y);
                    let card = Card::from(id);
                    cards.insert(pos, card);
                }
                None => {
                    panic!("Should have enough shuffled card ids for all cards")
                }
            }

            x += 1;
        }

        cards
    }
}

fn main() {
    let mut grid = Grid::new();

    let mut selection: Option<Card> = None;
}
