use std::str::Chars;
use std::vec::Vec;
use std::iter::Peekable;

/*
**	Data structure
*/

#[derive(Debug)]
pub enum	Token {
	Separator,
	Word(String)
}

/*
**	Tokenizer trait implementation for string
**	Tokenize()
*/

pub trait	Tokenizer {
	fn		tokenize(&self) -> Vec<Token>;
}

fn		tokenize_word(pk: &mut Peekable<Chars>) -> String
{//je suis pas venue ici pour souffrire OK
	let mut	s:String = String::new();
	let mut	quoted = false;
	let mut escaped = false;

	while let Some(&ch) = pk.peek() {
		if ch == '\n' || ch == ' ' {
			pk.next().unwrap();
		}
		else if ch == '"'{
			quoted = true;
			pk.next().unwrap();			
			break;
		}
		else {
			break;
		}
	}
	while let Some(&ch) = pk.peek() {
		if ch == '\\' {
			escaped = true;
			pk.next().unwrap();
		}
		else if quoted == true && ch == '"' {
			if escaped {
				escaped = false;
				pk.next().unwrap();
				s.push(ch);
			}
			else
			{
				break ;
			}
		}
		else if quoted || (ch != '\n' && ch != ' ' && ch != ';') {
			pk.next().unwrap();
			s.push(ch);
		}
		else {
			break;
		}
	}
	s
}

impl	Tokenizer for String {
	fn		tokenize(&self) -> Vec<Token> {
		let mut	tokens: Vec<Token> = vec![];
		let mut	pk = self.chars().peekable();

		loop {
			match pk.peek() {
				Some(&ch) => match ch {
					';' => {
						tokens.push(Token::Separator);
						pk.next();
					}
					' ' | '\n' => {
						pk.next();
					}
					_ => {
						tokens.push(Token::Word(tokenize_word(&mut pk)));
					}
				},
				None => break
			}
		}
		tokens
	}
}
