use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878")?;
    let mut http_requests: Vec<String> = Vec::new();

    for (connection_id, stream) in listener.incoming().enumerate() {
        let stream: TcpStream = stream?;
        println!("Handling Connection #{}", connection_id + 1);
        let mut http_request: Vec<String> = handle_connection(stream);
        
        println!("{:#?}", http_request);
        
        if http_request[0].contains(&"exit".to_string()) {
            std::process::exit(0)
        };
        
        http_requests.append(&mut http_request);
    }

    println!("{} http requests", http_requests.len());
    
    return Ok(());
}

fn handle_connection(mut stream: TcpStream) -> Result<Vec<String>, std::io::Error> {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&mut stream);

    let mut http_request: Vec<String> = Vec::new();
    
    for line in buf_reader.lines().into_iter() {
        let line: String = line?;
        if line.is_empty() {
            http_request.push(line);
        };
    }
    
    return Ok(http_request);
}