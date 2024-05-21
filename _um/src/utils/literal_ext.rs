use std::fmt::Display;

use proc_macro2::Literal;

pub trait LiteralExt: Display {
    fn is_str(&self) -> bool;

    fn as_string(&self) -> Option<String> {
        if self.is_str() {
            let s = self.to_string();
            Some(s[1..self.to_string().len() - 1].to_string())
        } else {
            None
        }
    }
}

impl LiteralExt for Literal {
    fn is_str(&self) -> bool {
        let s = self.to_string();
        s.starts_with('"') && s.ends_with('"')
    }
}
