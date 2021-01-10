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

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::convert::TryInto;
use array2d::Array2D;

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
#[derive(Clone)]
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

#[derive(Default, Clone)]
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

struct Sheet {
    array: Array2D<Cell>,
}
impl Sheet {
    fn cell_ref(&self, x: u16, y: u16) -> Option<&Cell> {
        self.array.get(y as usize - 1, x as usize - 1)
        // &self.array[x as usize - 1][y as usize - 1]
    }
    fn cell_ref_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.array.get_mut(y as usize - 1, x as usize - 1)
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
        let chars_wide = chars_wide-4;
        let cells_wide = std::cmp::min(cellswide, (chars_wide+cellwidth)/cellwidth);
        queue!(stdout,
            cursor::SavePosition,
            terminal::Clear(terminal::ClearType::All),
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
        queue!(stdout,
            SetBackgroundColor(Color::Black),
        )?;
        for row in 0..chars_high {
            if row + scrolly < cellshigh {
                queue!(stdout,
                    cursor::MoveTo(cells_wide*cellwidth-4, row),
                    terminal::Clear(terminal::ClearType::UntilNewLine),
                )?;
            }
        }
        execute!(stdout, ResetColor, SetAttribute(Attribute::Reset), cursor::RestorePosition)?;
        Ok(())
    }
}
impl Default for Sheet {
    fn default() -> Self {
        Sheet {
            array: Array2D::filled_with(Cell::default(), cellshigh as usize, cellswide as usize),
        }
    }
}

fn x_pos(x: u16) -> u16 {
    x * cellwidth - (cellwidth - 4)
}

fn run<W: Write>(mut stdout: &mut W) -> Result<()> {
    let mut sheet = Sheet::default();
    // for i in 1..cellswide {
    //     for j in 1..cellshigh {
    //         sheet.set_cell(i, j, &format!("{},{}", i, j));
    //         println!("{} + {} = {}", i, j, i+j);
    //         if i == 5 {
    //             sheet.cell_ref_mut(i, j).unwrap().alignment = CellAlignment::Right;
    //         } else if i == 6 {
    //             sheet.cell_ref_mut(i, j).unwrap().alignment = CellAlignment::Left;
    //         }
    //     }
    // }
    for i in 1..4 {
        sheet.cell_ref_mut(i, 1).unwrap().alignment = CellAlignment::Right;
        sheet.cell_ref_mut(i, 2).unwrap().alignment = CellAlignment::Center;
        sheet.cell_ref_mut(i, 3).unwrap().alignment = CellAlignment::Left;
    }
    let mut scrollpos = (0u16, 0u16);
    let mut curpos = (1u16, 1u16);
    execute!(stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        cursor::MoveTo(0, cellshigh),
    )?;
    sheet.display(&mut stdout, curpos, scrollpos)?;
    loop {
        terminal::enable_raw_mode()?;
        if poll(Duration::from_millis(500))? {
            let event = read()?;
            print!("Event: {:?}", event);
            stdout.execute(cursor::MoveToNextLine(1))?;
            if event == Event::Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('c') }) {
                break;
            }
            match event {
                Event::Key(KeyEvent { modifiers: KeyModifiers::NONE, code }) => {
                    match code {
                        KeyCode::Up => curpos.1 = std::cmp::max(curpos.1.saturating_sub(1), 1),
                        KeyCode::Down => curpos.1 = std::cmp::min(curpos.1.saturating_add(1), cellshigh-2),
                        KeyCode::Left => curpos.0 = std::cmp::max(curpos.0.saturating_sub(1), 1),
                        KeyCode::Right => curpos.0 = std::cmp::min(curpos.0.saturating_add(1), cellswide-2),
                        KeyCode::Delete => sheet.set_cell(curpos.0, curpos.1, ""),
                        KeyCode::Enter => sheet.edit_cell(&mut stdout, curpos, scrollpos)?,
                        _ => {},
                    }
                },
                Event::Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => {
                    match code {
                        KeyCode::Up => scrollpos.1 = scrollpos.1.saturating_sub(1),
                        KeyCode::Down => scrollpos.1 = std::cmp::min(scrollpos.1.saturating_add(1), cellshigh-2),
                        KeyCode::Left => scrollpos.0 = scrollpos.0.saturating_sub(1),
                        KeyCode::Right => scrollpos.0 = std::cmp::min(scrollpos.0.saturating_add(1), cellswide-2),
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        sheet.display(&mut stdout, curpos, scrollpos)?;
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
    result
}

