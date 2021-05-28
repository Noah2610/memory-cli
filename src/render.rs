use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use crate::card::Card;
use crate::grid::Grid;
use crate::{Pos, CARD_PAIRS, CELL_PADDING, CELL_SIZE, HIDDEN_CHR};

pub fn render(
    stdout: &mut io::Stdout,
    grid: &Grid,
    revealed: &HashSet<Pos>,
) -> io::Result<()> {
    render_coords(stdout, &grid.size)?;
    render_cards(stdout, &grid.cards, &revealed)?;
    render_prompt(stdout, &grid.size)?;

    stdout.flush()
}

fn render_coords(stdout: &mut io::Stdout, size: &Pos) -> io::Result<()> {
    use termion::clear;
    use termion::cursor;

    write!(
        stdout,
        "{}{}",
        clear::All,
        cursor::Goto(CELL_SIZE.0 + CELL_PADDING.0, 1),
    )?;

    for x in 1 ..= size.0 {
        write!(
            stdout,
            "{: <w$}{}",
            x.to_string(),
            cursor::Right(CELL_PADDING.0),
            w = CELL_SIZE.0 as usize
        )?;
    }

    for y in 1 ..= size.1 {
        write!(
            stdout,
            "{}{: ^w$}",
            cursor::Goto(1, y * (CELL_SIZE.1 + CELL_PADDING.1)),
            y.to_string(),
            w = CELL_SIZE.0 as usize
        )?;
    }

    Ok(())
}

fn render_cards(
    stdout: &mut io::Stdout,
    cards: &HashMap<Pos, Card>,
    revealed: &HashSet<Pos>,
) -> io::Result<()> {
    use termion::cursor;

    for (pos, card) in cards {
        let x = (pos.0 + 1) * (CELL_SIZE.0 + CELL_PADDING.0);
        let y = (pos.1 + 1) * (CELL_SIZE.1 + CELL_PADDING.1);
        let is_revealed = revealed.contains(pos);
        let s = if is_revealed { card.chr() } else { HIDDEN_CHR }
            .to_string()
            .repeat(CELL_SIZE.0 as usize);
        for card_row in 0 .. CELL_SIZE.0 {
            write!(stdout, "{}{}", cursor::Goto(x, y + card_row), &s)?;
        }
    }

    Ok(())
}

fn render_prompt(stdout: &mut io::Stdout, size: &Pos) -> io::Result<()> {
    use termion::cursor;

    write!(
        stdout,
        "{}Input XY> ",
        cursor::Goto(1, (size.1 + 1) * (CELL_SIZE.1 + CELL_PADDING.1))
    )
}

pub fn render_gameover(
    stdout: &mut io::Stdout,
    guesses: usize,
    size: &Pos,
) -> io::Result<()> {
    use termion::cursor;

    writeln!(
        stdout,
        "{}You Win!\nIt took you {} guesses to find all {} pairs,\ngiving you \
         an accuracy of {}%.",
        cursor::Goto(1, (size.1 + 1) * (CELL_SIZE.1 + CELL_PADDING.1) + 2),
        guesses,
        CARD_PAIRS,
        (CARD_PAIRS as f32 / guesses as f32) * 100.0
    )
}
