use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, Read};
use std::string::String;
use byteorder::{ByteOrder, LittleEndian};

pub enum	LoginMessages {
	Identification = 1,
	IdentificationSuccess = 2,
	IdentificationFailure = 3
}

pub enum	Messages {
	ExecSysteme = 0,
	ExecSystemeDump = 1,
	FileDump = 2,
	FilePush = 3,
	Exit = 4
}

pub enum Response {
	RequestSuccess = 0,
	RequestFailure = 1,
	FileDumpResult = 2,
}

pub struct MessageStream<'a> {
	reader: BufReader<&'a TcpStream>,
	writer: BufWriter<&'a TcpStream>
}

pub trait MessageStreamTrait<'a> {
	fn read_message(&mut self) -> Option<MessageContainer>;
	fn send_message(&mut self, msg: MessageContainer);
	fn from_tcp_stream(stream: &'a TcpStream) -> MessageStream<'a>;
}

impl<'a> MessageStreamTrait<'a> for MessageStream<'a> {
	fn read_message(&mut self) -> Option<MessageContainer> {
		let mut buffer = [0 as u8; 12];

		match self.reader.read_exact(&mut buffer) {
			Ok(_) => {
				let id = LittleEndian::read_i32(&buffer);
				let len = LittleEndian::read_u64(&buffer[4..12]);

				if len > 0 {
					let mut buf = vec![0 ; len as usize];
					self.reader.read_exact(&mut buf).expect("Can't read packet body !");
					return Option::Some(MessageContainer { id: id, content: String::from_utf8(buf).unwrap() });
				}
				Option::Some(MessageContainer { id: id, content:String::new() })
			},
			Err(_) => {
				Option::None
			}
		}
	}
	fn send_message(&mut self, msg: MessageContainer){
		let mut buffer = [0 as u8; 12];
		LittleEndian::write_i32(&mut buffer, msg.id);
		LittleEndian::write_u64(&mut buffer[4..12], msg.content.len() as u64);
		self.writer.write(&buffer).unwrap();
		self.writer.write(msg.content.as_bytes()).unwrap();
		self.writer.flush().expect("[error]Can't flush MessageStream !");
	}
	fn from_tcp_stream(stream: &'a TcpStream) -> MessageStream<'a> {
		MessageStream {
			reader: BufReader::new(stream),
			writer: BufWriter::new(stream)
		}
	}
}

#[derive(Debug)]
pub struct MessageContainer {
	pub id: i32,
	pub content: String
}
