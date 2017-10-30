use messages::*;

static CLI_HANDLERS: &'static [fn(msg: MessageContainer)] = 
	&[ handle_request_success, handle_request_failure, handle_file_dump_result ];

fn		handle_request_failure(msg: MessageContainer) {
	println!("[client]Server say request failure : \"{}\"", msg.content);
}

fn		handle_request_success(msg: MessageContainer) {
	println!("[client]Server say request done : \"{}\"", msg.content);
}

fn		handle_file_dump_result(msg: MessageContainer) {
	println!("{}", msg.content);
}

pub fn		client_handle_messages(msg: MessageContainer)
{
	if msg.id >= 0 && (msg.id as usize) < CLI_HANDLERS.len() {
		CLI_HANDLERS[msg.id as usize](msg);
	}
	else {
		println!("[client]Undefined server response : {:?}", msg);
	}
}