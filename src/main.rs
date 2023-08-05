use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878")?;

    for (connection_id, stream) in listener.incoming().enumerate() {
        println!("Handling Connection #{}", connection_id + 1);
        handle_connection_book_ver(stream?)?;
    }
        
    return Ok(());
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let http_request = read_http_request(&mut stream)?;

    let response: String = build_html_response(r"E:\src\rust\web_server\example.html")?;
        
    stream.write_all(response.as_bytes())?;

    return Ok(());
}

fn handle_connection_book_ver(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let http_request: Vec<String> = read_http_request_book_ver(&mut stream);

    let response: String = build_html_response(r"E:\src\rust\web_server\example.html")?;
    
    stream.write_all(response.as_bytes())?;

    return Ok(());
}

fn build_html_response(html_file_path: &str) -> Result<String, std::io::Error> {
    let status_line: &str = "HTTP/1.1 200 OK";
    let contents: String = fs::read_to_string(html_file_path)?;
    let length: usize = contents.len();
    let response: String = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    return Ok(response);
}

fn read_http_request(stream: &mut TcpStream) -> Result<Vec<String>, std::io::Error> {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(stream);
    let mut http_request: Vec<String> = Vec::new();
    for (line_id, line) in buf_reader.lines().enumerate() {
        println!("unwrapping line #{} with ?", line_id+1);
        let line: String = line?;
        if !line.is_empty() {
            http_request.push(line);
        };
        println!("line #{} collected", line_id+1);
    }
    return Ok(http_request);
}

fn read_http_request_book_ver(stream: &mut TcpStream) -> Vec<String> {
    let buffer_reader = BufReader::new(stream);
    let http_request = buffer_reader
        .lines()
        .map(|result| result.expect("io error"))
        .take_while(|line| !line.is_empty())
        .collect();
    return http_request;
}