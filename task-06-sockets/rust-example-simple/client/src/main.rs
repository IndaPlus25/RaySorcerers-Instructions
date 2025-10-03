use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

// Magic to get input from user
fn get_u8_input(s: &str) -> u8 {
    print!("{s}");
    io::stdout().flush().expect("flush failed!");

    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let result = input_line.trim().parse().expect("Input not an u8");

    println!();

    result
}

fn main() -> std::io::Result<()> {
    loop {
        if let Ok(mut socket) = TcpStream::connect("127.0.0.1:1337") {
            println!("Connected to the server!");

            // Send a and b to server
            let a = get_u8_input("First number >>> ");
            let b = get_u8_input("Second number >>> ");
            socket.write(&[a, b])?;
            socket.flush().expect("Couldn't flush the message to the socket");
            println!("Send to server: {a}, {b}");

            // Receive the result
            let mut buf = [0; 1];
            socket.read(&mut buf).expect("Couldn't read from the socket");
            let c = &buf[0];
            println!("Received from server: {c}\n");
        } else {
            println!("Couldn't connect to server...");
        };
    }
}