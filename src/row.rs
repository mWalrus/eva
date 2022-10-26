use super::cell::Cell;

pub static LETTERS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Row {
    pub cells: Vec<Cell>,
    pub row_num: usize,
}

impl Row {
    pub fn new(cells: Vec<Cell>, row_num: usize) -> Self {
        Self { cells, row_num }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        self.cells.iter().for_each(|c| len += c.value.len());
        len
    }
}
