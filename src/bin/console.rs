use crossterm::{
    event::{self, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};
use std::io::stdout;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    let mut counter = 0;

    loop {
        term.draw(|frame| {
            frame.render_widget(
                Paragraph::new(format!("Counting for days: {} (Use 'q' to quit)", counter))
                    .white()
                    .on_blue(),
                frame.size(),
            );
        })?;

        if event::poll(std::time::Duration::from_millis(128))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => break,
                        event::KeyCode::Char('j') => counter -= 1,
                        event::KeyCode::Char('k') => counter += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
