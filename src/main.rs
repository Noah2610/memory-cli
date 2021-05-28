extern crate rand;
extern crate termion;

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;

const CARD_PAIRS: usize = 6;
const CARDS_PER_ROW: usize = 4;
// const ROWS: usize = (CARDS / CARDS_PER_ROW) + (CARDS % CARDS_PER_ROW).min(1);

const SLEEP_MS: u64 = 1000;

struct Card(pub usize);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.0 {
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
    pub cards: HashMap<Pos, Card>,
    pub size:  (usize, usize),
}

type GridSize = (usize, usize);

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
        let mut x = 0;
        let mut y = 0;
        let mut size = (0, 0);
        for _ in 0 .. CARD_PAIRS * 2 {
            if x >= CARDS_PER_ROW {
                x = 0;
                y += 1;
            }

            if x > size.0 {
                size.0 = x
            };
            if y > size.1 {
                size.1 = y
            };

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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut grid = Grid::new();

    let mut is_running = true;
    let mut revealed: HashSet<Pos> = HashSet::new();
    let mut selected: Vec<Pos> = Vec::with_capacity(2);

    while is_running {
        if let Some(target) = get_input(&stdin) {
            let is_on_grid = target.0 >= 0
                && target.0 < grid.size.0
                && target.1 >= 0
                && target.1 < grid.size.1;
            let is_revealed = revealed.contains(&target);
            if is_on_grid && !is_revealed {
                revealed.insert(target);
                selected.push(target);
            }
        }

        render(&mut stdout, &grid, &revealed);

        if selected.len() == 2 {
            if let (Some(card_a), Some(card_b)) =
                (grid.cards.get(&selected[0]), grid.cards.get(&selected[1]))
            {
                let is_pair = card_a.0 == card_b.0;

                if !is_pair {
                    revealed.remove(&selected[0]);
                    revealed.remove(&selected[1]);
                }
                selected.clear();
            } else {
                panic!("Selected cards aren't in grid");
            }
        }
    }
}

fn get_input(stdin: &io::Stdin) -> Option<Pos> {
    let mut input_buf = String::new();
    let _ = stdin.read_line(&mut input_buf);
    let input = input_buf.trim().replace(" ", "");
    let parsed = input
        .split("")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    if parsed.len() == 2 {
        Some((parsed[0], parsed[1]))
    } else {
        None
    }
}

fn render(stdout: &mut io::Stdout, grid: &Grid, revealed: &HashSet<Pos>) {
}
