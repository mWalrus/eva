use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use crate::app::App;

pub fn run(mut app: App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if let Err(_e) = app.poll_event() {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let list_items: Vec<ListItem> = app
        .rows
        .iter()
        .map(|r| {
            let row_spans: Vec<Span> = r
                .cells
                .iter()
                .map(|c| {
                    Span::styled(
                        format!("| ({}{}) {} ", c.col, r.row_num, &c.value),
                        Style::default().fg(c.color),
                    )
                })
                .collect();
            let spans = Spans::from(row_spans);
            ListItem::new(spans)
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().title("Sheet").borders(Borders::ALL))
        .highlight_symbol(" > ")
        .highlight_style(
            Style::default()
                .bg(Color::Indexed(8))
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(list, f.size(), &mut app.state)
}
