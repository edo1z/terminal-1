#![allow(unused_assignments)]

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
};
use display_info::DisplayInfo;
use std::env;
use std::io::stdout;

fn main() -> Result<()> {
    let term_program = env::var("TERM_PROGRAM").unwrap();
    let os_name = get_os();
    println!("\nYour OS is {}.", os_name);
    print!("Your terminal is ");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Green),
        Print(term_program),
        ResetColor
    )?;

    let all_display = DisplayInfo::all().unwrap();
    for display in all_display {
        println!("\n----- Display {} -----", display.id);
        println!("width: {}", display.width);
        println!("height: {}", display.height);
        println!("scale_factor: {}", display.scale_factor);
        println!("is_primary: {}", display.is_primary);
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn get_os() -> String {
    String::from("mac")
}

#[cfg(target_os = "linux")]
fn get_os() -> String {
    String::from("linux")
}
