/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
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

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::vec_deque::VecDeque;
use std::fs;
use std::io::{stdout, Write, prelude::*};
use std::thread::sleep;
use std::time::Duration;
use std::convert::TryInto;
use ndarray::prelude::*;
use serde::{Serialize, Deserialize};

use crossterm::{
    execute, queue,
    cursor::{self, MoveToNextLine},
    event::{self, poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Color, Attribute, Print, ResetColor, SetBackgroundColor, SetForegroundColor, SetAttribute},
    terminal,
    ExecutableCommand, Result,
};

// const fullsize: usize = 20;
const fullwidth: usize = 20;
const fullheight: usize = 64;
const cellswide: u16 = fullwidth as u16 + 1;
const cellshigh: u16 = fullheight as u16 + 1;
const cellwidth: u16 = 10; //Note: this isnt actually fully handled
const celltextsize: usize = cellwidth as usize - 1;

#[repr(u8)]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum CellAlignment {
    Left,
    Right,
    Center,
}
impl Default for CellAlignment {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
struct Cell {
    text: String,
    alignment: CellAlignment,
}
impl Cell {
    fn get_display(&self) -> String {
        let ending = if self.text.len() > celltextsize { "+" } else { "|" };
        let rtvl: String = self.text.chars().take(celltextsize).collect();
        match self.alignment {
            CellAlignment::Left => format!("{:<w$}{}", rtvl, ending, w = celltextsize),
            CellAlignment::Right => format!("{:>w$}{}", rtvl, ending, w = celltextsize),
            // {
            //     let rtvl: String = self.text.chars().rev().take(celltextsize).collect::<String>().chars().rev().collect();
            //     format!("{:>w$}{}", rtvl, ending, w = celltextsize)
            // },
            CellAlignment::Center => format!("{:^w$}{}", rtvl, ending, w = celltextsize)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Sheet {
    array: Array2<Cell>,
    // array: Array2D<Cell>,
}
impl Sheet {
    fn cell_ref(&self, x: u16, y: u16) -> Option<&Cell> {
        self.array.get((y as usize - 1, x as usize - 1))
        // &self.array[x as usize - 1][y as usize - 1]
    }
    fn cell_ref_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.array.get_mut((y as usize - 1, x as usize - 1))
        // &mut self.array[x as usize - 1][y as usize - 1]
    }
    fn set_cell(&mut self, x: u16, y: u16, text: &str) {
        let t: &mut String = &mut self.cell_ref_mut(x, y).unwrap().text;
        t.clear();
        t.push_str(text);
    }
    fn get_display(&self, x: u16, y: u16) -> String {
        self.cell_ref(x, y).unwrap().get_display()
    }
    fn edit_cell<W: Write>(&mut self, stdout: &mut W, curpos: (u16, u16), scrollpos: (u16, u16)) -> Result<()> {
        let (curx, cury) = curpos;
        let (cx, cy) = curpos;
        let (scrollx, scrolly) = scrollpos;
        let curx = curx.checked_sub(scrollx);
        let cury = cury.checked_sub(scrolly);
        if let Some(curx) = curx {
            if let Some(cury) = cury {
                if curx >= 1 && cury >= 1 {
                    let mut buf = self.cell_ref(cx, cy).unwrap().text.clone();
                    let mut txtpos = buf.len();
                    let mut extra = buf.len().saturating_sub(8);
                    execute!(stdout,
                        cursor::SavePosition,
                        cursor::Show,
                    )?;
                    loop {
                        execute!(stdout,
                            SetAttribute(Attribute::OverLined),
                            SetBackgroundColor(Color::White),
                            SetForegroundColor(Color::Black),
                            cursor::MoveTo(x_pos(curx), cury),
                            Print(format!("{:^w$}", "", w = celltextsize + extra)),
                            SetBackgroundColor(Color::Grey),
                            SetForegroundColor(Color::Black),
                            cursor::MoveTo(x_pos(curx), cury),
                            Print(format!("{:^w$}|", "", w = celltextsize)),
                            cursor::MoveTo(x_pos(curx), cury),
                            Print(&buf),
                            cursor::MoveTo(x_pos(curx), cury),
                            cursor::MoveRight(txtpos as u16),
                        )?;
                        extra = buf.len().saturating_sub(8);
                        if let Event::Key(KeyEvent { code, ..}) = event::read()? {
                            match code {
                                KeyCode::Enter => {
                                    self.set_cell(cx, cy, &buf);
                                    break;
                                },
                                KeyCode::Esc => {
                                    break;
                                }
                                KeyCode::Left => {
                                    txtpos = std::cmp::max(txtpos.saturating_sub(1), 0);
                                }
                                KeyCode::Right => {
                                    txtpos = std::cmp::min(txtpos.saturating_add(1), buf.len());
                                }
                                KeyCode::Char(c) => {
                                    buf.insert(txtpos, c);
                                    txtpos += 1;
                                }
                                KeyCode::Backspace => {
                                    if let Some(idx) = txtpos.checked_sub(1) {
                                        buf.remove(idx);
                                        txtpos = idx;
                                    }
                                }
                                KeyCode::Delete => {
                                    buf.remove(txtpos);
                                    txtpos = std::cmp::min(txtpos, buf.len());
                                }
                                KeyCode::Up => {}
                                KeyCode::Down => {}
                                KeyCode::Home => {}
                                KeyCode::End => {}
                                KeyCode::PageUp => {}
                                KeyCode::PageDown => {}
                                KeyCode::Tab => {}
                                KeyCode::BackTab => {}
                                KeyCode::Insert => {}
                                KeyCode::F(_) => {}
                                KeyCode::Null => break
                            }
                        }
                    }
                    execute!(stdout,
                        cursor::Hide,
                        cursor::RestorePosition,
                        ResetColor,
                    )?;
                }
            }
        }
        Ok(())
    }
    fn display<W: Write>(&self, stdout: &mut W, curpos: (u16, u16), scrollpos: (u16, u16)) -> Result<()> {
        let (curx, cury) = curpos;
        let (scrollx, scrolly) = scrollpos;
        let curx = curx.checked_sub(scrollx);
        let cury = cury.checked_sub(scrolly);
        let (chars_wide, chars_high) = terminal::size()?;
        let chars_high = chars_high.saturating_sub(1);
        let chars_wide = chars_wide.saturating_sub(4);
        let cells_wide = std::cmp::min(cellswide, (chars_wide+cellwidth)/cellwidth);
        queue!(stdout,
            SetBackgroundColor(Color::Grey),
            SetForegroundColor(Color::Black),
            SetAttribute(Attribute::OverLined),
            terminal::DisableLineWrap,
        )?;
        for pos in 1..cells_wide {
            if pos + scrollx < cellswide {
                queue!(stdout,
                    cursor::MoveTo(x_pos(pos), 0),
                    Print(format!(" {:^w$}|", std::char::from_u32(64+(pos + scrollx) as u32).unwrap(), w=celltextsize-1)),
                )?;
            } else {
                queue!(stdout,
                    cursor::MoveTo(x_pos(pos), 0),
                    terminal::Clear(terminal::ClearType::FromCursorDown),
                )?;
            }
        }
        for row in 1..chars_high {
            if row + scrolly < cellshigh {
                queue!(stdout,
                    cursor::MoveTo(0, row),
                    Print(format!("{:>3} ", row + scrolly)),
                )?;
            } else {
                queue!(stdout,
                    cursor::MoveTo(0, row),
                    terminal::Clear(terminal::ClearType::CurrentLine),
                )?;
            }
        }
        queue!(stdout,
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black),
            SetAttribute(Attribute::OverLined),
        )?;
        for col in 1..cells_wide {
            for row in 1..chars_high {
                if curx == Some(col) && cury == Some(row) {
                    queue!(stdout, SetBackgroundColor(Color::Grey))?;
                }
                if col + scrollx < cellswide && row + scrolly < cellshigh {
                    queue!(stdout,
                        cursor::MoveTo(x_pos(col), row),
                        Print(self.get_display(col + scrollx, row + scrolly)),
                    )?;
                }
                if curx == Some(col) && cury == Some(row) {
                    queue!(stdout, SetBackgroundColor(Color::White))?;
                }
            }
        }
        execute!(stdout, ResetColor, SetAttribute(Attribute::Reset))?;
        Ok(())
    }
}
impl Default for Sheet {
    fn default() -> Self {
        Sheet {
            array: Array2::from_elem((cellshigh as usize, cellswide as usize), Cell::default()),
        }
    }
}

fn x_pos(x: u16) -> u16 {
    x * cellwidth - (cellwidth - 4)
}


struct Context {
    sheet: Sheet,
    scrollpos: (u16, u16),
    curpos: (u16, u16),
    filename: String,
    status_line: String,
    running: bool,
    saved: bool,
    command_history: Vec<String>,
}
impl Context {
    fn enter_command<W: Write>(&mut self, stdout: &mut W) -> Result<Option<String>> {
        let mut buf = String::from(":");
        let mut buf2 = String::new();
        let mut hist_pos = self.command_history.len();
        let mut txtpos = buf.len();
        let mut extra = buf.len().saturating_sub(8);
        let mut suggest_count = 0;
        let (chars_wide, chars_high) = terminal::size()?;
        let row_num = chars_high.saturating_sub(1);
        let chars_wide = chars_wide.saturating_sub(0);
        execute!(stdout,
            cursor::SavePosition,
            cursor::Show,
        )?;
        loop {
            execute!(stdout,
                SetBackgroundColor(Color::Grey),
                SetForegroundColor(Color::Black),
                cursor::MoveTo(0, chars_high),
                terminal::Clear(terminal::ClearType::CurrentLine),
                cursor::MoveTo(0, chars_high),
                Print(&buf),
                cursor::MoveTo(0, chars_high),
                cursor::MoveRight(txtpos as u16),
            )?;
            if let Event::Key(KeyEvent { code, ..}) = event::read()? {
                if suggest_count > 0 {
                    let cpos = cursor::position()?;
                    for i in 0..suggest_count {
                        execute!(stdout,
                            cursor::MoveUp(i+1),
                            terminal::Clear(terminal::ClearType::CurrentLine),
                            cursor::MoveDown(i+1),
                        )?;
                    }
                    suggest_count = 0;
                }
                match code {
                    KeyCode::Enter => {
                        execute!(stdout,
                            cursor::Hide,
                            cursor::RestorePosition,
                            ResetColor,
                        )?;
                        return Ok(Some(buf));
                    },
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Left => {
                        txtpos = std::cmp::max(txtpos.saturating_sub(1), 0);
                    }
                    KeyCode::Right => {
                        txtpos = std::cmp::min(txtpos.saturating_add(1), buf.len());
                    }
                    KeyCode::Char(c) => {
                        buf.insert(txtpos, c);
                        txtpos += 1;
                    }
                    KeyCode::Backspace => {
                        if let Some(idx) = txtpos.checked_sub(1) {
                            if idx == 0 {
                                break;
                            }
                            buf.remove(idx);
                            txtpos = idx;
                        }
                    }
                    KeyCode::Delete => {
                        buf.remove(txtpos);
                        txtpos = std::cmp::min(txtpos, buf.len());
                    }
                    KeyCode::Up => {
                        if hist_pos == self.command_history.len() {
                            if hist_pos > 0 {
                                hist_pos = hist_pos.saturating_sub(1);
                                buf2.push_str(&buf);
                                buf.clear();
                                buf.push_str(&self.command_history.get(hist_pos).unwrap());
                                txtpos = buf.len();
                            }
                        } else if hist_pos == 0 {
                        } else {
                            hist_pos = hist_pos.saturating_sub(1);
                            buf.clear();
                            buf.push_str(&self.command_history.get(hist_pos).unwrap());
                            txtpos = buf.len();
                        }
                    }
                    KeyCode::Down => {
                        if hist_pos != self.command_history.len() {
                            hist_pos = hist_pos.saturating_add(1);
                            if hist_pos == self.command_history.len() {
                                buf.clear();
                                buf.push_str(&buf2);
                                buf2.clear();
                                txtpos = buf.len();
                            } else {
                                buf.clear();
                                buf.push_str(&self.command_history.get(hist_pos).unwrap());
                                txtpos = buf.len();
                            }
                        }
                    }
                    KeyCode::Home => {}
                    KeyCode::End => {}
                    KeyCode::PageUp => {}
                    KeyCode::PageDown => {}
                    KeyCode::Tab => {
                        let mut file = buf.split(" ");
                        let start = match file.next() {
                            Some(s) => s,
                            None => continue
                        };
                        let file: Vec<&str> = file.collect();
                        if file.len() == 0 {
                            continue;
                        }
                        let file = file.join(" ");
                        let len = file.len() as u16;
                        let files = fs::read_dir(".")?;
                        let files: Vec<String> = files.filter_map(|f| {
                            if let Ok(f) = f {
                                if let Ok(f) = f.file_name().into_string() {
                                    if f.starts_with(&file) {
                                        return Some(f);
                                    }
                                }
                            }
                            None
                        }).collect();
                        if files.len() == 1 {
                            buf = format!("{} {}", start, files[0]);
                            txtpos = buf.len();
                        } else if files.len() != 0 {
                            let cpos = cursor::position()?;
                            for f in files {
                                suggest_count += 1;
                                queue!(stdout,
                                    cursor::MoveUp(1),
                                    terminal::Clear(terminal::ClearType::CurrentLine),
                                    Print(&f),
                                    cursor::MoveLeft(f.len() as u16),
                                )?;
                            }
                            execute!(stdout,
                                cursor::MoveTo(cpos.0, cpos.1),
                            )?;
                        }
                    }
                    KeyCode::BackTab => {}
                    KeyCode::Insert => {}
                    KeyCode::F(_) => {}
                    KeyCode::Null => break
                }
            }
        }
        execute!(stdout,
            cursor::Hide,
            cursor::RestorePosition,
            ResetColor,
        )?;
        Ok(None)
    }
    fn display<W: Write>(&mut self, mut stdout: &mut W) -> Result<()> {
        let status_row = terminal::size()?.1.saturating_sub(1);
        queue!(stdout,
            cursor::SavePosition,
            terminal::Clear(terminal::ClearType::All),
        )?;
        self.sheet.display(&mut stdout, self.curpos, self.scrollpos)?;
        execute!(stdout,
            SetBackgroundColor(Color::Grey),
            SetForegroundColor(Color::Black),
            cursor::MoveTo(0, status_row),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveTo(0, status_row),
            Print(&self.status_line),
            SetAttribute(Attribute::Reset),
            cursor::RestorePosition,
            ResetColor,
        )
    }
    fn set_current_cell(&mut self, text: &str) {
        self.sheet.set_cell(self.curpos.0, self.curpos.1, text);
        self.saved = false;
    }
    fn edit_current_cell<W: Write>(&mut self, mut stdout: &mut W) -> Result<()> {
        self.sheet.edit_cell(&mut stdout, self.curpos, self.scrollpos)?;
        self.saved = false;
        Ok(())
    }
    fn open_sheet(&mut self, filename: Option<&str>) -> std::result::Result<(), bincode::Error> {
        if let Some(filename) = filename {
            self.filename.clear();
            self.filename.push_str(filename);
        }
        let mut file = fs::File::open(&self.filename)?;
        self.sheet = bincode::deserialize_from(file)?;
        self.saved = true;
        Ok(())
    }
    fn write_sheet(&mut self, filename: Option<&str>) -> std::result::Result<(), bincode::Error> {
        if let Some(filename) = filename {
            self.filename.clear();
            self.filename.push_str(filename);
        }
        let mut file = fs::File::create(&self.filename)?;
        bincode::serialize_into(file, &self.sheet)?;
        self.saved = true;
        Ok(())
    }
    fn move_up(&mut self) {
        self.curpos.1 = std::cmp::max(self.curpos.1.saturating_sub(1), 1);
    }
    fn move_down(&mut self) {
        self.curpos.1 = std::cmp::min(self.curpos.1.saturating_add(1), cellshigh-2);
    }
    fn move_left(&mut self) {
        self.curpos.0 = std::cmp::max(self.curpos.0.saturating_sub(1), 1);
    }
    fn move_right(&mut self) {
        self.curpos.0 = std::cmp::min(self.curpos.0.saturating_add(1), cellswide-2);
    }
    fn scroll_up(&mut self) {
        self.scrollpos.1 = self.scrollpos.1.saturating_sub(1);
    }
    fn scroll_down(&mut self) {
        self.scrollpos.1 = std::cmp::min(self.scrollpos.1.saturating_add(1), cellshigh-2);
    }
    fn scroll_left(&mut self) {
        self.scrollpos.0 = self.scrollpos.0.saturating_sub(1);
    }
    fn scroll_right(&mut self) {
        self.scrollpos.0 = std::cmp::min(self.scrollpos.0.saturating_add(1), cellswide-2);
    }
}
impl Default for Context {
    fn default() -> Self {
        Context {
            sheet: Sheet::default(),
            scrollpos: (0u16, 0u16),
            curpos: (1u16, 1u16),
            filename: String::new(),
            status_line: String::new(),
            running: true,
            saved: true,
            command_history: Vec::new(),
        }
    }
}

fn handle_command(context: &mut Context, command_str: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            "^:(?P<c>\\w+)(?P<f>!)?( (?P<a>.+))?$"
            // "^:(?P<c>e(dit)?|w(rite)?|q(uit)?|wq)(?P<f>!)?( (?P<a>.+))?$"
        ).unwrap();
    }
    context.command_history.push(String::from(command_str));
    if let Some(captures) = RE.captures(&command_str) {
        let command = captures.name("c").unwrap().as_str();
        let force = captures.name("f").is_some();
        let argument = captures.name("a").map(|c| c.as_str());
        Ok(match command {
            "q" | "quit" => {
                if context.saved || force {
                    context.running = false;
                    String::new()
                } else {
                    String::from("File edited since last save! Write and close with :wq or force with :q!")
                }
            }
            "wq" => {
                if context.write_sheet(argument).is_ok() {
                    context.running = false;
                    String::new()
                } else {
                    format!("ERROR writing file {}", &context.filename)
                }
            }
            "e" | "edit" => {
                if context.saved || force {
                    if context.open_sheet(argument).is_ok() {
                        format!("Opened file {}", &context.filename)
                    } else {
                        format!("ERROR opening file {}", &context.filename)
                    }
                } else {
                    String::from("File edited since last save! Write current file or force with :e!")
                }
            },
            "w" | "write" => {
                if context.write_sheet(argument).is_ok() {
                    format!("Wrote file {}", &context.filename)
                } else {
                    format!("ERROR writing file {}", &context.filename)
                }
            },
            _ => {
                format!("Unknown command \"{}\"", &command)
                // format!("Command: \"{}\" Argument: \"{}\" matched regex but not command selection!!", command, argument.unwrap_or("null"))
            },
        })
    } else {
        Ok(format!("Unknown command \"{}\"", &command_str))
    }
    // Ok(String::new())
}

fn run<W: Write>(mut stdout: &mut W) -> Result<()> {
    println!("yeet");
    let mut context = Context::default();
    println!("yeet2");
    for i in 1..4 {
        context.sheet.cell_ref_mut(i, 1).unwrap().alignment = CellAlignment::Right;
        context.sheet.cell_ref_mut(i, 2).unwrap().alignment = CellAlignment::Center;
        context.sheet.cell_ref_mut(i, 3).unwrap().alignment = CellAlignment::Left;
    }
    execute!(stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        cursor::MoveTo(0, cellshigh),
    )?;
    context.display(&mut stdout)?;
    while context.running {
        terminal::enable_raw_mode()?;
        if poll(Duration::from_millis(500))? {
            let event = read()?;
            stdout.execute(cursor::MoveToNextLine(1))?;
            match event {
                Event::Key(KeyEvent { modifiers: KeyModifiers::NONE, code }) => {
                    match code {
                        KeyCode::Up => context.move_up(),
                        KeyCode::Down => context.move_down(),
                        KeyCode::Left => context.move_left(),
                        KeyCode::Right => context.move_right(),
                        KeyCode::Delete => context.set_current_cell(""),
                        KeyCode::Enter => context.edit_current_cell(&mut stdout)?,
                        KeyCode::Char(':') => {
                            if let Some(command_str) = context.enter_command(&mut stdout)? {
                                let output = handle_command(&mut context, &command_str)?;
                                if output.len() > 0 {
                                    context.status_line = output;
                                }
                            }
                        },
                        _ => {},
                    }
                },
                Event::Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => {
                    match code {
                        KeyCode::Up => context.scroll_up(),
                        KeyCode::Down => context.scroll_down(),
                        KeyCode::Left => context.scroll_left(),
                        KeyCode::Right => context.scroll_right(),
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        context.display(&mut stdout)?;
        execute!(stdout,
            cursor::SavePosition,
        )?;
        execute!(stdout,
            cursor::RestorePosition,
            SetAttribute(Attribute::Reset),
            ResetColor,
        )?;
    }
    Ok(())
}

fn main() -> Result<()> {
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
    println!("Cell size: {} bytes", std::mem::size_of::<Cell>());
    println!("Sheet static size: {} bytes", std::mem::size_of::<Sheet>());
    let sheet = Sheet::default();
    println!("Sheet heap size: {} bytes", std::mem::size_of_val(&sheet));
    result
}

