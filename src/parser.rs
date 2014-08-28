use std::slice::Items;
use tokenizer::*;
use semantics;

macro_rules! lookahead (
	($iter:ident 1) => ({
		let mut _temp = $iter.clone();
		(_temp.next())
    });
    ($iter:ident 2) => ({
		let mut _temp = $iter.clone();
		(_temp.next(),_temp.next())
    });
    ($iter:ident 3) => ({
		let mut _temp = $iter.clone();
		(_temp.next(),_temp.next(),_temp.next())
    });
)


fn parse(tokens: Vec<Token>) {
	parse_term(tokens.iter());
}



fn parse_term(mut iter: Items<Token>) -> Option<semantics::Term> {
	let lookahead = lookahead!(iter 2);
	match lookahead {
		(Some(&Atom(ref s)),       Some(&GroupL)) => parse_structure(iter),
		(Some(&Atom(ref s)),       _            ) => parse_atom(iter),
		(Some(&Variable(ref s)),   _            ) => parse_variable(iter),
		(_,					   _)			  => None,
	}
}

fn parse_atom(mut iter: Items<Token>) -> Option<semantics::Term> {
	match iter.next() {
		Some(&Atom(ref f)) => {
			return Some(semantics::Atom(f.to_string()));
		},
		_ => return None,
	}
}

fn parse_variable(mut iter: Items<Token>) -> Option<semantics::Term> {
	match iter.next() {
		Some(&Variable(ref f)) => {
			return Some(semantics::Variable(f.to_string()));
		},
		_ => return None,
	}
}

fn parse_structure(mut iter: Items<Token>) -> Option<semantics::Term> {
	match iter.next() {
		Some(&Atom(ref f)) => {
			let mut terms = Vec::new();
			loop {
				let lookahead = lookahead!(iter 1);
				match (iter.next()) {
					Some(&Atom(_))| Some(&Variable(_))	=> {
						match parse_term(iter) {
							Some(t) => terms.push(t),
							None	=> return None,
						}
					},
					Some(&Separator)				=> {iter.next();},
					Some(&GroupR)					=> {iter.next(); break;},
					_								=> {return None;},
				}
			}
			return Some(semantics::Structure(f.to_string(), terms));
		},
		_ => None,
	}
}


