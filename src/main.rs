/*
 * --------------------
 * THIS FILE IS LICENSED UNDER THE FOLLOWING TERMS
 *
 * this code may not be used for any purpose. be gay, do crime
 *
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */
 
use simplelog::*;
use std::io::stdout;
use std::fs::File;

use sheets::Result;
use sheets::run;
mod error;


mod func {
    use rust_decimal::prelude::*;
    trait Function {
        fn calculate(&self) -> std::result::Result<Decimal, String> {
            Err(format!("NO IMPL"))
        }
    }
}

use crossterm::{
    execute,
    style::{Attribute, SetAttribute, ResetColor},
    cursor,
    terminal,
    ExecutableCommand,
};

fn main() -> Result<()> {
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("log.txt").unwrap());
    let mut stdout = stdout();
    let result = run(&mut stdout);
    execute!(stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        ResetColor,
        SetAttribute(Attribute::Reset),
    )?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    // println!("Cell size: {} bytes", std::mem::size_of::<Cell>());
    // println!("Sheet static size: {} bytes", std::mem::size_of::<Sheet>());
    // let sheet = Sheet::default();
    // println!("Sheet heap size: {} bytes", std::mem::size_of_val(&sheet));
    result
}

