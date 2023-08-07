use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
    thread,
    error::Error,
};

use web_server::ThreadPool;

fn main() -> Result<(), Box<dyn Error>> { 
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878")?;
    let pool: ThreadPool = ThreadPool::new(4)?;

    for (connection_id, possible_stream)
    in listener.incoming().enumerate() {
        let stream: TcpStream = possible_stream?;
        println!("CONNECTION {}", connection_id + 1);
        pool.execute(|| {
            handle_connection(stream)
        })?; // handle this better later
    }
        
    return Ok(());
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let request: String = read_request(&mut stream)?;
    let response: String = parse_request(&request)?;
    
    stream.write_all(response.as_bytes())?;

    return Ok(());
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
    let request_line = request.lines().next().unwrap_or_default();
    
    let (status, content_path) = match &request_line[..] {
        "GET / HTTP/1.1"      => ("HTTP/1.1 200 OK", r"html\hello.html"),
        "GET /exit HTTP/1.1" |
        "GET /exit/ HTTP/1.1" => std::process::exit(0),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", r"html\hello.html")
        },
        "GET /assets/lake.png HTTP/1.1" => ("HTTP/1.1 200 OK", r"assets\lake.png"),
        _ => ("HTTP/1.1 404 NOT FOUND", r"html\404.html"),
    };
    return build_response(status, content_path);
}

fn build_response(status: &str, content_path: &str) -> Result<String, io::Error> {
    if content_path.contains(".html") {
        let hmtl: String = fs::read_to_string(content_path)?;
        let response: String = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status, hmtl.len(), hmtl);
        return Ok(response);
    }
    if content_path.contains(".png") || content_path.contains(".bmp") {
        let image: String = base64::encode(&fs::read(content_path)?);
        let response: String = format!("{}\r\nContent-Type: image/bmp\r\nContent-Length: {}\r\n\r\n{}", status, image.len(), image);
        return Ok(response);
    }
    return Ok(String::new());
}