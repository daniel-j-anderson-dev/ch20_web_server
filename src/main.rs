use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
    thread,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> { 
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878")?;

    for (connection_id, stream) in listener.incoming().enumerate() {
        let (request, response) = handle_connection(stream?)?;
        println!("CONNECTION {}\nREQUEST\n{}\nRESPONSE\n{}\n", connection_id + 1, request, response);
    }
        
    return Ok(());
}

fn handle_connection(mut stream: TcpStream) -> Result<(String, String), io::Error> {
    let request: String = read_request(&mut stream)?;
    let response: String = parse_request(&request)?;
    
    stream.write_all(response.as_bytes())?;

    return Ok((request, response));
}

fn read_request(stream: &mut TcpStream) -> Result<String, io::Error> {
    let buf_reader: BufReader<&mut TcpStream>  = BufReader::new(stream);
    let mut http_request: String = String::new();
    for line in buf_reader.lines() {
        let line: String = line?;
        if line.is_empty() { break };
        http_request.push_str(&line);
        http_request.push('\n');
    }
    return Ok(http_request);
}

fn parse_request(request: &str) -> Result<String, io::Error> {
    let request_line = request.lines().next().unwrap();
    
    let (status, content_path) = match &request_line[..] {
        "GET / HTTP/1.1"      => ("HTTP/1.1 200 OK", r"html\hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", r"html\hello.html")
        },
        "GET /exit HTTP/1.1"  => std::process::exit(0),
        _                     => ("HTTP/1.1 404 NOT FOUND", r"html\404.html"),
    };
    return build_response(status, content_path);
}

fn build_response(status: &str, content_path: &str) -> Result<String, io::Error> {
    let content: String = fs::read_to_string(content_path)?;
    let response: String = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status, content.len(), content);
    return Ok(response);
}