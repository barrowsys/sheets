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
#![allow(dead_code)]

use crate::consts::*;
use crate::error::*;
use crate::func::*;
use crate::Sheet;

use convert_base::Convert;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use std::str::FromStr;
use regex::Regex;
use rust_decimal::prelude::*;

fn to_col(n: u16) -> String {
    let mut conv: Convert = Convert::new(65535, 27);
    let v: Vec<u32> = conv.convert(&[n]);
    let v: String = v.into_iter().map(|u| std::char::from_u32(64+u).unwrap()).collect();
    v
}
fn from_col(s: &str) -> Result<u16> {
    let mut conv: Convert = Convert::new(27, 65535);
    let t = s.chars();
    let t: Vec<u32> = t.map(|c| (c as u32).saturating_sub(64)).collect();
    if t.iter().all(|n| *n > 0 && *n < 27) {
        let v: Vec<u32> = conv.convert(&t);
        Ok(v[0] as u16)
    } else {
        Err(ErrorKind::coord_ref(&format!("{:?}", t)))
    }
}

#[derive(Debug)]
pub enum CoordRef {
    Absolute(u16),
    Relative(i16, u16),
}
impl CoordRef {
    pub fn parse(_s: &str, relative_to: u16) -> Result<Self> {
        fn handle_col(s: &str) -> Result<u16> {
            if let Ok(u) = s.parse() {
                Ok(u)
            } else {
                from_col(s)
            }
        }
        let err = ErrorKind::coord_ref(_s);
        let column: CoordRef = if let Some(s) = _s.strip_prefix("$") {
            let s = handle_col(s)?;
            CoordRef::Absolute(s)
        } else {
            let s = handle_col(_s)?;
            let a = relative_to;
            use std::cmp::Ordering;
            let s: i16 = match a.cmp(&s) {
                Ordering::Less => a.checked_add(s).ok_or(err)? as i16,
                Ordering::Equal => 0,
                Ordering::Greater => (s as i16).checked_sub(a as i16).ok_or(err)?,
            };
            CoordRef::Relative(s, relative_to)
        };
        Ok(column)
    }
    pub fn value(&self) -> Result<u16> {
        match self {
            Self::Absolute(v) => Ok(*v),
            //TODO: this should be prettier
            Self::Relative(r, a) => {
                if let Ok(r) = u16::try_from(*r) {
                    if let Some(ret) = a.checked_add(r) {
                        return Ok(ret);
                    }
                } else if let Ok(r) = u16::try_from(-r) {
                    if let Some(ret) = a.checked_sub(r) {
                        return Ok(ret);
                    }
                }
                Err(ErrorKind::RelativeNumber(*r, *a))
            },
        }
    }
}
lazy_static! {
    static ref CELL_REF: Regex = Regex::new(
        r"^(\$?[a-zA-Z])(\$?\d+)$"
    ).unwrap();
}
#[derive(Debug)]
pub struct CellRef(CoordRef, CoordRef);
impl CellRef {
    pub fn parse(s: &str, relative_to: (u16, u16)) -> Result<Self> {
        if let Some(captures) = CELL_REF.captures(&s) {
            let column = captures.get(1).unwrap().as_str();
            let column = CoordRef::parse(column, relative_to.0)?;
            let row = captures.get(2).unwrap().as_str();
            let row = CoordRef::parse(row, relative_to.1)?;
            let r = Self(column, row);
            Ok(r)
        } else {
            Err(ErrorKind::cell_ref(s))
        }
    }
    pub fn value(&self) -> Result<(u16, u16)> {
        Ok((self.0.value()?, self.1.value()?))
    }
}
impl Function for CellRef {
    fn calc(&self, r_track: u8, sheet: &Sheet) -> Result<Decimal> {
        let r = self.value()?;
        let r = sheet.cell_ref(r.0, r.1).unwrap();
        if let Some(d) = r.data.calc(r_track+1, sheet)? {
            Ok(d)
        } else {
            Ok(Decimal::from_u32(0).unwrap())
        }
    }
}
pub struct CellRange {
    from: (u16, u16),
    to: (u16, u16),
    current: (u16, u16),
}
impl CellRange {
    pub fn new(from: CellRef, to: CellRef) -> Result<Self> {
        let mut current = from.value()?;
        current.0 -= 1;
        Ok(Self {
            from: from.value()?,
            to: to.value()?,
            current,
        })
    }
}
impl Iterator for CellRange {
    type Item = CellRef;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.0 += 1;
        if self.current.0 > self.to.0 {
            self.current.0 = self.from.0;
            if self.current.1 > self.to.1 {
                return None;
            } else {
                self.current.1 += 1;
            }
        }
        Some(CellRef(CoordRef::Absolute(self.current.0), CoordRef::Absolute(self.current.1)))
    }
}

#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum CellData {
    Text(String),
    Number(Decimal),
    Function(
        String,
        #[serde(skip)]
        Option<Box<dyn Function>>
    ),
}
impl CellData {
    pub fn from_raw(input: &str, relative_to: (u16, u16)) -> Self {
        if let Some(input2) = input.strip_prefix("=") {
            Self::Function(String::from(input), parse_function(input2, relative_to))
        } else {
            let d2 = Decimal::from_str(input);
            if let Ok(d) = d2 {
                Self::Number(d)
            } else {
                Self::Text(String::from(input))
            }
        }
    }
    pub fn as_raw(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),
            Self::Number(d) => format!("{}", d),
            Self::Function(s, _) => s.clone(),
        }
    }
    pub fn as_num(&self) -> Option<Decimal> {
        match self {
            Self::Text(_) => None,
            Self::Number(d) => Some(d.clone()),
            Self::Function(_, _) => None,
        }
    }
    pub fn as_display(&self, sheet: &Sheet) -> String {
        match self {
            Self::Text(s) => String::from(s),
            Self::Number(d) => format!("{}", d),
            Self::Function(_, _) => {
                let c = self.calc(0, sheet);
                match c {
                    Ok(c) => {
                        if let Some(d) = c {
                            return format!("{}", d);
                        } else {
                            return String::from("NONE");
                        }
                    }
                    Err(e) => format!("{}", e),
                }
            }
        }
    }
    pub fn calc(&self, r_track: u8, sheet: &Sheet) -> Result<Option<Decimal>> {
        match self {
            Self::Number(d) => Ok(Some(d.clone())),
            Self::Function(_, Some(f)) => Ok(Some(f.calc(r_track, sheet)?)),
            Self::Text(s) => {
                Ok(Some(Decimal::from_u32(5).unwrap())) //1230 + (r_track as u32)
            }
            _ => Ok(None)
        }
    }
}
impl Default for CellData {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
impl PartialEq for CellData {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Text(s) => {
                if let Self::Text(s2) = other {
                    return s.eq(s2);
                }
            }
            Self::Number(n) => {
                if let Self::Number(n2) = other {
                    return n.eq(n2);
                }
            }
            Self::Function(s, _) => {
                if let Self::Function(s2, _) = other {
                    return s.eq(s2);
                }
            }
        }
        false
    }
}
impl Clone for CellData {
    fn clone(&self) -> Self {
        match self {
            Self::Text(s) => Self::Text(s.clone()),
            Self::Number(n) => Self::Number(n.clone()),
            Self::Function(s, _) => Self::Function(s.clone(), None),
        }
    }
}

#[repr(u8)]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CellAlignment {
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
pub struct Cell {
    pub data: CellData,
    pub raw: Option<String>,
    pub display: Option<String>,
    pub num: Option<Decimal>,
    pub alignment: CellAlignment,
}
impl Cell {
    pub fn set_data(&mut self, input: &str, relative_to: (u16, u16)) {
        self.data = CellData::from_raw(input, relative_to);
    }
    pub fn display(&self, sheet: &Sheet) -> String {
        let text = self.data.as_display(sheet);
        let ending = if text.len() > celltextsize { "+" } else { "|" };
        let rtvl: String = text.chars().take(celltextsize).collect();
        match self.alignment {
            CellAlignment::Left => format!("{:<w$}{}", rtvl, ending, w = celltextsize),
            CellAlignment::Right => format!("{:>w$}{}", rtvl, ending, w = celltextsize),
            CellAlignment::Center => format!("{:^w$}{}", rtvl, ending, w = celltextsize)
        }
    }
}

