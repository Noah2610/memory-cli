pub struct Card(pub u16);

impl Card {
    pub const fn chr(&self) -> char {
        match self.0 {
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
        }
    }
}
