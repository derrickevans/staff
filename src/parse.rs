use std::str::{Lines, SplitAsciiWhitespace};

pub enum Item<'a> {
    Command {
        name: &'a str,
        args: SplitAsciiWhitespace<'a>,
    },
}

pub struct Parser<'a> {
    lines: Lines<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.lines.next() {
            if let Some(tail) = line.strip_prefix('\\') {
                let mut args = tail.split_ascii_whitespace();
                let name = args.next().unwrap();
                Some(Item::Command { name, args })
            } else {
                todo!()
            }
        } else {
            None
        }
    }
}
