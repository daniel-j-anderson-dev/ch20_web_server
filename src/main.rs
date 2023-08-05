use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878")?;

    for (connection_id, stream) in listener.incoming().enumerate() {
        println!("Handling Connection #{}", connection_id + 1);
        handle_connection(stream?)?;
    }
        
    return Ok(());
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let http_request: Vec<String> = read_http_request(&mut stream)?;
    
    if http_request[0].contains(&"exit".to_string()) {
        std::process::exit(0)
    }

    let response: &[u8] = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
    stream.write_all(response)?;
    
    return Ok(());
}

fn read_http_request(stream: &mut TcpStream) -> Result<Vec<String>, Box<dyn Error>> {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(stream);

    let mut http_request: Vec<String> = Vec::new();
    
    for line in buf_reader.lines().into_iter() {
        let line: String = line?;
        if line.is_empty() {
            http_request.push(line);
        };
    }
    
    if !http_request.is_empty() {
        return Ok(http_request);
    } else {
        return Err("Empty http request".into());
    }
}