use std::collections::VecDeque;
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener, TcpStream};
use std::io::Error;
use std::io::{Read, Write};

type QueueProcessor<T> = dyn Fn(T, VecDeque<T>) -> ();

pub type Port = u16;
pub type Message = String;

pub enum Mode {
    Server(Port),
    Client(Message),
}

pub fn parse_mode(args: Vec<String>) -> Option<Mode> {
    if args.len() < 3 {
        None
    }
    else if args[1] == "serve" {
        let port: Result<u16, _> = args[2].parse::<u16>();
        match port {
            Ok(p) => Some(Mode::Server(p)),
            Err(_) => None,
        }
    }
    else if args[1] == "send" {
        Some(Mode::Client(args[2].clone()))
    }
    else { None }
}


pub struct Queue<T> {
    name: String,
    items: VecDeque<T>,
    queue_processor: QueueProcessor<T>,
}


pub fn listen_on(port: u16) -> Result<TcpListener, Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, port);
    let listener = TcpListener::bind(socket)?;
    println!("Listening on {}, access this port to end the program", port);
    Ok(listener)
}

pub fn serve(port: u16) -> Result<(), Error> {
    let listener = listen_on(port)?;
    loop {
        let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
        println!("Connection received! {:?} is sending data.", addr);
        let mut input = String::new();
        let _ = tcp_stream.read_to_string(&mut input)?;
        println!("Received message from {:?}: {}", addr, input);
    }
}


fn connect_to_server(server_host: &str, server_port: u16) -> Option<TcpStream> {
    let mut sock_addr: String = String::from(server_host);
    sock_addr.push_str(":");
    sock_addr.push_str(server_port.to_string().as_str());

    match TcpStream::connect(sock_addr) {
        Ok(mut stream) => {
            Some(stream)
        },
        Err(e) => {
            None
        }
    }
}

pub fn connect_and_send_message(
    server_host: &str,
    server_port: u16,
    message: String
) -> Result<(), String> {
    let maybe_stream = connect_to_server(server_host, server_port);
    match maybe_stream {
        None => Err(String::from("Could not connect to server")),
        Some(mut stream) => {
            stream.write(message.as_bytes()).unwrap();
            Ok(())
        }
    }
    // match stream.read_exact(&mut data) {
    //     Ok(_) => {
    //         if &data == msg {
    //             println!("Reply is ok!");
    //         } else {
    //             let text = from_utf8(&data).unwrap();
    //             println!("Unexpected reply: {}", text);
    //         }
    //     },
    //     Err(e) => {
    //         println!("Failed to receive data: {}", e);
    //     }
    // }
}
