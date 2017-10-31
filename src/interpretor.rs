use parser::*;
use messages::*;

pub trait Interpretor {
	fn intepret(self, stream: &mut MessageStream);
}

impl Interpretor for Vec<CmdNode> {
	fn intepret(self, stream: &mut MessageStream) {
		let mut cnt = String::new();
		let mut id: i32 = -1;
		for item in self.into_iter() {
			if id == -1 {
				match item {
					CmdNode::Message(msg) => {
						id = msg as i32;
					},
					_ => {
						println!("Invalide cmd !");
						return ;
					}
				}
			}
			else {
				match item {
					CmdNode::Argument(arg) => {
						if cnt.len() == 0 {
							cnt += &arg;
						}
						else {
							cnt += " ";
							cnt += &arg;
						}
					}
					_ => break
				}
			}
		}
		stream.send_message(MessageContainer { id: id, content: cnt.clone() })
	}
}