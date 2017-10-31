use messages::Messages;
use lexer::Token;

pub enum CmdNode {
	Message(Messages),
	Argument(String)
}

pub trait Parser {
	fn parse_request(self) -> Option<Vec<CmdNode>>;
}

impl Parser for Vec<Token> {
	fn parse_request(self) -> Option<Vec<CmdNode>> {
		let mut		cmd: Vec<CmdNode> = vec![];
		for tk in self {
			if let Token::Word(cmd_str) = tk {
				cmd.push(match cmd_str.as_str() {
					"sys" => CmdNode::Message(Messages::ExecSysteme),
					"sysdump" => CmdNode::Message(Messages::ExecSystemeDump),
					"fdump" => CmdNode::Message(Messages::FileDump),
					"fpush" => CmdNode::Message(Messages::FilePush),
					"close" => CmdNode::Message(Messages::Exit),
					_ => CmdNode::Argument(String::from(cmd_str.as_str()))
				});
			}
		}
		if cmd.len() > 0 {
			return Option::Some(cmd)
		}
		Option::None
	}
}