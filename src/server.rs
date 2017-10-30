use std::net::{TcpListener, TcpStream};
use consts;
use messages::*;
use server_handlers::*;

pub enum	ClientStats {
	Disconnected = 0,
	Identification = 1,
	Connected = 2
}

pub struct	Client<'a> {
	pub stream: MessageStream<'a>,
	pub stats: ClientStats
}

fn		server_check_identification(client: &mut Client, msg: MessageContainer) -> bool {
	if msg.content.eq("saluttoto") {
		client.stream.send_message(MessageContainer { id: LoginMessages::IdentificationSuccess as i32, content:String::new() });
		return true
	}
	client.stream.send_message(MessageContainer { id: LoginMessages::IdentificationFailure as i32, content: String::from("Bad credentials !") });
	println!("{}", msg.content);
	false
}

fn		server_handle_connect(socket: TcpStream)
{
	let mut client = Client { stream: MessageStream::from_tcp_stream(&socket), stats: ClientStats::Identification };
	loop {
		match client.stream.read_message() {
			Some(msg) => {
				if msg.id == LoginMessages::Identification as i32 {
					if server_check_identification(&mut client, msg) {
						//println!("[server]Client connected !");
						client.stats = ClientStats::Connected;
						server_frame(client);
						break 
					}
					else {
						//println!("[server]Identification failure !");
						client.stats = ClientStats::Disconnected;
						break
					}
				}
				else {
					//println!("[server]Bad identification request received, closeing connection !");
					client.stats = ClientStats::Disconnected;
					break
				}
			},
			None => {
				//println!("[server]Can't read message from client, exit...");
				client.stats = ClientStats::Disconnected;
				break
			}
		}
	}
}

fn		server_wait_connect(listener: TcpListener)
{
	for inco in listener.incoming()
	{
		match inco {
			Ok(_) => {
				//println!("[info]Client accepted !");
				server_handle_connect(inco.unwrap());
			}
			Err(_) => {
				//println!("[error]Can't accept client !");
			}
		}
	}
}

pub fn	server_listen_connect()
{
	let listener = TcpListener::bind(consts::LISTEN_ADDR);
	match listener {
		Ok(_) => {
			//println!("[info]Server waiting for connections ...");
			server_wait_connect(listener.unwrap());
		}
		Err(_) => {
			//println!("[error]Can't listen for connections !");
		}
	}
}