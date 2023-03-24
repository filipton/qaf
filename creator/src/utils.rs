use std::io::stdout;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct AlternateScreenCleanup;

impl AlternateScreenCleanup {
    pub fn new() -> anyhow::Result<Self> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        return Ok(AlternateScreenCleanup {});
    }
}

impl Drop for AlternateScreenCleanup {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
