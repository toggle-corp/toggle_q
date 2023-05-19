use std::env;
use std::process;

use toggle_q::{serve, parse_mode, Mode, connect_and_send_message};


const SERVER_HOST: &'static str = "localhost"; // this is to change in future, configurable
const SERVER_PORT: u16 = 3333;

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_mode(args) {
        None => {
            eprintln!("Invalid params. Usage: toggle_q (serve <port>) | (send <msg>).");
            process::exit(1);
        },
        Some(Mode::Server(port)) => {
            serve(port).unwrap();
        },
        Some(Mode::Client(message)) => {
            connect_and_send_message(SERVER_HOST, SERVER_PORT, message).unwrap();
        },
    }
}
