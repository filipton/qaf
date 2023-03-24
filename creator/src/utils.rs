use std::io::stdout;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct AlternateScreenCleanup;

impl AlternateScreenCleanup {
    pub fn new() -> anyhow::Result<Self> {
        execute!(stdout(), EnterAlternateScreen)?;
        return Ok(AlternateScreenCleanup {});
    }
}

impl Drop for AlternateScreenCleanup {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}
