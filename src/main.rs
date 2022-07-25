use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
};
use std::env;
use std::io::stdout;

fn main() -> Result<()> {
    let term_program = env::var("TERM_PROGRAM").unwrap();
    print!("\nYour terminal is ");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Green),
        Print(term_program),
        ResetColor
    )?;
    println!("\n");
    Ok(())
}
