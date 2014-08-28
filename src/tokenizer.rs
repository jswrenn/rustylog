use std::iter::Peekable;

#[deriving(Eq, PartialEq, Show, Clone)]
pub enum Token {
    GroupL,
    GroupR,
    Separator,
    Implies,
    Variable(String),
    Atom(String),
}

pub struct Tokenizer<T> {
    iter:  Peekable<char, T>
}

impl<T: Iterator<char>> Tokenizer<T> {
    pub fn new(input: T) -> Tokenizer<T> {
        Tokenizer {
            iter: input.peekable(),
        }
    }
}

impl<T: Iterator<char>> Iterator<Token> for Tokenizer<T> {
    fn next(&mut self) -> Option<Token> {
        match self.iter.peek() {
            Some(&'(')           => {self.iter.next(); Some(GroupL)},
            Some(&')')           => {self.iter.next(); Some(GroupR)},
            Some(&',')           => {self.iter.next(); Some(Separator)},
            Some(&'←')           => {self.iter.next(); Some(Implies)},
            Some(&'A'..'Z')      => {
                let chars:String = self.iter.by_ref().take_while(
                    |&c| match c {
                            'A'..'Z' => true,
                            _        => false,
                    }).collect();
                return Some(Atom(chars));
            },
            Some(&'a'..'z')     => {
                let chars:String = self.iter.by_ref().take_while(
                    |&c| match c {
                            'a'..'z' => true,
                            _        => false,
                    }).collect();
                return Some(Atom(chars));
            },
            Some(_)             => {self.iter.next(); return self.next()},
            None                => {None},
        }
    }
}
