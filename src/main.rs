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

fn handle_connection(mut stream: TcpStream) -> Vec<String> {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&mut stream);

    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap()) // explore more robust error handling
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);
    return http_request;
}