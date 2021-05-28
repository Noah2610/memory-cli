extern crate rand;
extern crate termion;

mod card;
mod grid;
mod render;

use std::collections::HashSet;
use std::io;

use grid::Grid;

pub const CARD_PAIRS: u16 = 6;
pub const CARDS_PER_ROW: u16 = 4;

pub const CELL_SIZE: (u16, u16) = (2, 2);
pub const CELL_PADDING: (u16, u16) = (2, 1);

pub const HIDDEN_CHR: char = '█';

pub type Pos = (u16, u16);

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let grid = Grid::new();

    let mut is_running = true;
    let mut revealed: HashSet<Pos> = HashSet::new();
    let mut selected: Vec<Pos> = Vec::with_capacity(2);
    let mut guesses: usize = 0;

    render::render(&mut stdout, &grid, &revealed)?;

    while is_running {
        if let Some(target) = get_input(&stdin) {
            let is_on_grid = grid.cards.contains_key(&target);
            let is_revealed = revealed.contains(&target);
            if is_on_grid && !is_revealed {
                revealed.insert(target);
                selected.push(target);
            }
        }

        render::render(&mut stdout, &grid, &revealed)?;

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
                guesses += 1;
            } else {
                panic!("Selected cards aren't in grid");
            }
        }

        is_running = grid.cards.len() != revealed.len();
    }

    render::render_gameover(&mut stdout, guesses, &grid.size)
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
