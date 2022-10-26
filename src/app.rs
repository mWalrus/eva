use crate::cell::Cell;
use crate::EVENT_TIMEOUT;
use anyhow::{anyhow, Result};
use crossterm::event::{self, Event};
use std::path::PathBuf;
use tui::widgets::ListState;

use office::Excel;

use crate::keymap::{key_match, ExKeyList};
use crate::row::{Row, LETTERS};

pub struct AppState {
    pub y_scroll: ListState,
    pub x_scroll: ListState,
}

impl Default for AppState {
    fn default() -> Self {
        let mut ls = ListState::default();
        ls.select(Some(0));
        Self {
            y_scroll: ls.clone(),
            x_scroll: ls,
        }
    }
}

pub struct App {
    pub rows: Vec<Row>,
    pub row_count: usize,
    pub col_count: usize,
    pub keys: ExKeyList,
    pub state: ListState,
}

impl App {
    pub fn new(path: PathBuf, sheet: String) -> Result<Self> {
        let path = std::env::current_dir()?.join(path);
        println!("{path:#?}");
        let mut workbook = Excel::open(&path).unwrap();

        if let Ok(range) = workbook.worksheet_range(&sheet) {
            let (row_count, col_count) = range.get_size();
            let rows: Vec<Row> = range
                .rows()
                .enumerate()
                .map(|(i, r)| {
                    let cells: Vec<Cell> = r
                        .iter()
                        .enumerate()
                        .map(|(i, c)| Cell::from((c, LETTERS[i])))
                        .collect();
                    Row::new(cells, i + 1)
                })
                .collect();
            let keys = ExKeyList::default();
            let mut state = ListState::default();
            state.select(Some(0));
            return Ok(Self {
                rows,
                row_count,
                col_count,
                keys,
                state,
            });
        }

        Err(anyhow!("Failed to read sheet {sheet} from {path:#?}"))
    }

    fn poll() -> Result<Option<Event>> {
        if event::poll(EVENT_TIMEOUT)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn poll_event(&mut self) -> Result<()> {
        if let Some(Event::Key(key)) = Self::poll()? {
            if key_match(&key, &self.keys.up) {
                self.up();
            } else if key_match(&key, &self.keys.down) {
                self.down();
            } else if key_match(&key, &self.keys.quit) {
                return Err(anyhow!("Exit application"));
            }
        }
        Ok(())
    }

    fn up(&mut self) {
        let selected = self.state.selected().unwrap();
        self.state.select(Some(selected.saturating_sub(1)))
    }

    fn down(&mut self) {
        let selected = self.state.selected().unwrap();
        self.state
            .select(Some((selected + 1).min(self.rows.len() - 1)))
    }

    // fn left(&mut self) {
    //     let selected_cell = self.state.x_scroll.selected().unwrap();
    //     self.state
    //         .x_scroll
    //         .select(Some(selected_cell.saturating_sub(1)));
    // }

    // fn right(&mut self) {
    //     let selected_row = self.state.y_scroll.selected().unwrap();
    //     let row_len = self.rows[selected_row].len();
    //     self.state.x_scroll.select(Some(selected_row.min(row_len)));
    // }
}
