use messages::MessageContainer;
use messages::Messages;
use lexer::Token;

pub trait Parser {
	fn parse_request(self) -> Option<MessageContainer>;
}

impl Parser for Vec<Token> {
	fn parse_request(self) -> Option<MessageContainer> {
		let mut req_id: i32 = -1;
		for tk in self {
			if req_id == -1 {
				if let Token::Word(cmd_str) = tk {
					req_id = match cmd_str.as_str() {
						"sys" => Messages::ExecSysteme as i32,
						"sysdump" => Messages::ExecSystemeDump as i32,
						"fdump" => Messages::FileDump as i32,
						"fpush" => Messages::FilePush as i32,
						"close" => Messages::Exit as i32,
						_ => -1
					}
				}
				if req_id == Messages::Exit as i32 {
					return Option::Some(MessageContainer { id: req_id, content: String::new() })					
				}
			}
			else if let Token::Word(cmd_str) = tk{
				return Option::Some(MessageContainer { id: req_id, content:  cmd_str})
			}
		}
		Option::None
	}
}