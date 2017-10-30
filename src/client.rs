use std::io::{stdin, BufReader, BufRead};
use std::net::TcpStream;
use consts;
use messages::*;
use lexer::Tokenizer;
use parser::Parser;
use client_handlers::client_handle_messages;
use std::io::{self, Write};

fn		parse_cmd(stream: &mut MessageStream, cmd: String) -> bool{
	let tokens = cmd.tokenize();
	match tokens.parse_request() {
		Some(msg) => {
			stream.send_message(msg);
			true
		},
		None => {
			println!("[client]Invalide request !");
			false
		}
	}
}

fn		client_frame(mut stream: MessageStream) {
	let mut	stdread = BufReader::new(stdin());
	
	loop {
		let mut buf = String::new();
		print!("\x1b[36mtssh>\x1b[0m ");
		io::stdout().flush().unwrap();
		match stdread.read_line(&mut buf) {
			Ok(_) => {
				if buf.eq("exit") {
					return ;
				}
				if buf.len() >= 3 && parse_cmd(&mut stream, buf) == true {
					match stream.read_message() {
						Some(response) => {
							client_handle_messages(response);
						},
						None => {
							println!("[client]Server dosen't responde !");
						}
					}
				}
			},
			Err(_) => {
				println!("[client]Error, can't read stdin !");
			}
		}
	}
}

pub fn	client_connect(username: &str, password: &str)
{
	match TcpStream::connect(consts::CONNECT_ADDR) {
		Ok(socket) => {
			let mut stream = MessageStream::from_tcp_stream(&socket);

			stream.send_message(MessageContainer { id: LoginMessages::Identification as i32, content: String::from(username) + &String::from(password) });
			match stream.read_message() {
				Some(msg) => {
					match msg.id {
						2 => {
							println!("[client]Server accepte connection !");
							client_frame(stream);
						},
						3 => {
							println!("[client]Server refuse connection : \"{}\"", msg.content);
						}
						_ => {
							println!("[client]Server send bad response !");
						}
					}
				},
				None => {
					println!("[client]Server no response !");
				}
			}
		},
		Err(_) => {
			println!("[client]Can't connect to server !");
		}
	}
}