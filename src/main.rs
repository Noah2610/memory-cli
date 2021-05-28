extern crate rand;
extern crate termion;

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, Write};

const CARD_PAIRS: u16 = 6;
const CARDS_PER_ROW: u16 = 4;
// const ROWS: u16 = (CARDS / CARDS_PER_ROW) + (CARDS % CARDS_PER_ROW).min(1);

const CELL_SIZE: (u16, u16) = (2, 2);
const CELL_PADDING: (u16, u16) = (2, 1);

const SLEEP_MS: u64 = 1000;

struct Card(pub u16);

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

type Pos = (u16, u16);

struct Grid {
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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut grid = Grid::new();

    let mut is_running = true;
    let mut revealed: HashSet<Pos> = HashSet::new();
    let mut selected: Vec<Pos> = Vec::with_capacity(2);

    render(&mut stdout, &grid, &revealed);

    while is_running {
        if let Some(target) = get_input(&stdin) {
            let is_on_grid = target.0 < grid.size.0 && target.1 < grid.size.1;
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
        .filter_map(|s| s.parse::<u16>().ok().and_then(|n| n.checked_sub(1)))
        .collect::<Vec<u16>>();
    if parsed.len() == 2 {
        Some((parsed[0], parsed[1]))
    } else {
        None
    }
}

fn render(stdout: &mut io::Stdout, grid: &Grid, revealed: &HashSet<Pos>) {
    use termion::clear;
    use termion::cursor;

    render_coords(stdout, &grid.size);
    render_cards(stdout, &grid.cards, &revealed);

    let _ = stdout.flush();
}

fn render_coords(stdout: &mut io::Stdout, size: &Pos) {
    use termion::clear;
    use termion::cursor;

    write!(
        stdout,
        "{}{}",
        clear::All,
        cursor::Goto(CELL_SIZE.0 + CELL_PADDING.0, 1),
    );

    for x in 1 ..= size.0 {
        write!(
            stdout,
            "{: <w$}{}",
            x.to_string(),
            cursor::Right(CELL_PADDING.0),
            w = CELL_SIZE.0 as usize
        );
    }

    for y in 1 ..= size.1 {
        dbg!(y);
        write!(
            stdout,
            "{}{: ^w$}",
            cursor::Goto(1, y * (CELL_SIZE.1 + CELL_PADDING.1)),
            y.to_string(),
            w = CELL_SIZE.0 as usize
        );
    }
}

fn render_cards(
    stdout: &mut io::Stdout,
    cards: &HashMap<Pos, Card>,
    revealed: &HashSet<Pos>,
) {
    use termion::clear;
    use termion::cursor;

    for (pos, card) in cards {
        if revealed.contains(pos) {
            let x = (pos.0 + 1) * (CELL_SIZE.0 + CELL_PADDING.0);
            let y = (pos.1 + 1) * (CELL_SIZE.1 + CELL_PADDING.1);
            for card_row in 0 .. CELL_SIZE.0 {
                write!(
                    stdout,
                    "{}{}",
                    cursor::Goto(x, y + card_row),
                    &(card).to_string().repeat(CELL_SIZE.0 as usize)
                );
            }
        }
    }
}
