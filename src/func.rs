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

use crate::error::*;
use crate::cell::*;
use crate::Sheet;

use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::prelude::*;
use std::thread::sleep_ms;

lazy_static! {
    static ref FUNCTION: Regex = Regex::new(
        "(?:\
            (?P<f>\
                [a-zA-Z0-9:\\$]+\
            )\
            (\
                \\(\
                    (?P<a>\
                        (?:\
                            [^()]+\
                        |\
                            (\\(((?:[^()]+|\
                                (\\(((?:[^()]+\
                                )*)\\))?\
                            )*)\\))?\
                        )*\
                    )\
                \\)\
            )?\
        )"
                    // (?:,\
                    //     (?P<a2>\
                    //         (?:\
                    //             [^()]+\
                    //         |\
                    //             (\\(((?:[^()]+|\
                    //                 (\\(((?:[^()]+\
                    //                 )*)\\))?\
                    //             )*)\\))?\
                    //         )*\
                    //     )
                    // )?\
        // r"(?:(?P<f>[a-zA-Z0-9:]+)(\((?P<a>(?>[^()]+|(?2))*)\))| ?[+-] ?)"
        // r"(?:(?P<f>[a-zA-Z0-9:]+)(\((?P<a>(?>[^()]+|(?2))*)\))| ?[+-] ?)"
    ).unwrap();
}

pub fn parse_function(raw: &str, relative_to: (u16, u16)) -> Option<Box<dyn Function>> {
    let mut captures_iter = FUNCTION.captures_iter(raw);
    let match1 = captures_iter.next()?;
    let func = match1.name("f")?.as_str();
    // let arg = match1.name("a");
    if let Some(arg) = match1.name("a") {
        let args: Vec<&str> = arg.as_str().split(",").collect();
        if func == "SUM" {
            let mut parts = Vec::new();
            for a in args.iter() {
                let refs: Vec<&str> = a.split(":").collect();
                if refs.len() == 2 {
                    if let Ok(s) = CellRef::parse(refs[0], relative_to) {
                        if let Ok(e) = CellRef::parse(refs[1], relative_to) {
                            if let Ok(cr) = CellRange::new(s, e) {
                                parts.extend(cr);
                            }
                        }
                    }
                } else {
                    let f = CellRef::parse(a, relative_to);
                    match f {
                        Ok(f) => parts.push(f),
                        Err(_) => {return None;},
                    }
                }
            }
            return Some(Box::new(Sum { parts }));
        }
    } else {
        let h = CellRef::parse(func, relative_to);
        if let Ok(r) = h {
            return Some(Box::new(r));
        }
    }
    None
}

pub trait Function {
    fn calc(&self, r_track: u8, sheet: &Sheet) -> Result<Decimal> {
        Err(ErrorKind::new("NO IMPL"))
    }
}

struct Sum {
    parts: Vec<CellRef>,
}
impl Function for Sum {
    fn calc(&self, r_track: u8, sheet: &Sheet) -> Result<Decimal> {
        let mut sum = Decimal::zero();
        for p in self.parts.iter() {
            sum += p.calc(r_track+1, sheet)?;
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_relative_reference() {
        let data = "A1:C3";
        let mut captures_iter = FUNCTION.captures_iter(data);
        let match1 = captures_iter.next().unwrap();
        let func = match1.name("f").unwrap().as_str();
        assert_eq!(func, "A1:C3");
    }
    #[test]
    fn test_absolute_reference() {
        let data = "$A$1:$C$3";
        let mut captures_iter = FUNCTION.captures_iter(data);
        let match1 = captures_iter.next().unwrap();
        let func = match1.name("f").unwrap().as_str();
        assert_eq!(func, "$A$1:$C$3");
    }
    #[test]
    fn test_sum() {
        let data = "SUM($A$1:$C$3)";
        let mut captures_iter = FUNCTION.captures_iter(data);
        let match1 = captures_iter.next().unwrap();
        let func = match1.name("f").unwrap().as_str();
        assert_eq!(func, "SUM");
        let arg = match1.name("a").unwrap().as_str();
        assert_eq!(arg, "$A$1:$C$3");
    }
    #[test]
    fn test_max() {
        let data = "MAX(SUM(A1:A3), SUM(B1:B3))";
        let mut captures_iter = FUNCTION.captures_iter(data);
        let match1 = captures_iter.next().unwrap();
        let func = match1.name("f").unwrap().as_str();
        assert_eq!(func, "MAX");
        let arg = match1.name("a").unwrap().as_str();
        assert_eq!(arg, "SUM(A1:A3), SUM(B1:B3)");
    }
}
