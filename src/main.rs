use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for (connection_id, stream) in listener.incoming().enumerate() {
        let _stream = stream?;
        println!("Connection #{}", connection_id + 1);
    }
    
    return Ok(());
}