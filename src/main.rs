use	std::env;

mod server;
mod client;
mod consts;
mod messages;
mod lexer;
mod parser;
mod server_handlers;
mod client_handlers;

/*#[macro_use]
extern crate serde_derive;*/
extern crate byteorder;

extern crate serde;
extern crate serde_json;


fn	usage() {
	println!("usage: tinyssh [client | server] username password \"request\"");
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() <= 1 {
		usage();
		return ;
	}
	match args[1].as_ref() {
		"client" => {
			if args.len() < 4 {
				usage();
			}
			else {
				client::client_connect(&args[2], &args[3]);
			}
		}
		"server" => {
			server::server_listen_connect();
		}
		_ => {
			usage();
		}
	}
}
