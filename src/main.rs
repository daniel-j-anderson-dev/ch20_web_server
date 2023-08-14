use std::{
    fs, thread,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
};

pub mod error;
pub mod thread_pool;
use crate::{error::Error, thread_pool::ThreadPool};

fn main() {
    let ip_addr: String = std::env::args().nth(1).unwrap_or("127.0.0.1:7878".to_string());

    let listener: TcpListener = TcpListener::bind(&ip_addr)
    .unwrap_or_else(|error| {
        eprintln!("Couldn't start listening on {ip_addr}: {error}");
        std::process::exit(1);
    });

    let pool: ThreadPool = ThreadPool::new(4)
    .unwrap_or_else(|error| {
        eprintln!("Could't create thread pool: {error}");
        std::process::exit(1);
    });

    println!("\n-------------------Listening on {ip_addr}-------------------\n");

    for (connection_id, possible_stream) in listener.incoming().take(2).enumerate() {
        let stream: TcpStream = match possible_stream {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("TcpStream error: {error}");
                continue;
            }
        };

        println!("\n-----------------------\nCONNECTION {}\n-----------------------", connection_id);

        let job = || {
            handle_connection(stream)
            .unwrap_or_else(|error| {
                eprintln!("Error while handling connection: {error}");
            });
        };

        pool.execute(job)
        .unwrap_or_else(|error| {
            eprintln!("Thread pool error: {error}")
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let request: String = read_request(&mut stream)?;

    let response: String = parse_request(&request)?;

    stream.write_all(response.as_bytes())?;

    return Ok(());
}

fn read_request(stream: &mut TcpStream) -> Result<String, Error> {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(stream);

    let mut http_request: String = String::new();

    for line in buf_reader.lines() {
        let line: String = line?;
        if line.is_empty() {
            break
        };

        http_request.push_str(&line);
        http_request.push('\n');
    }

    return Ok(http_request);
}

fn parse_request(request: &str) -> Result<String, Error> {
    let request_line: &str = request.lines().nth(0).unwrap_or_default();

    let (status, content_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/hello.html"),
        "GET /exit HTTP/1.1" | "GET /exit/ HTTP/1.1" => std::process::exit(0),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html"),
    };

    return build_response(status, content_path);
}

fn build_response(status: &str, content_path: &str) -> Result<String, Error> {
    if content_path.contains(".html") {
        let hmtl: String = fs::read_to_string(content_path).map_err(|error| Error::Io(error))?;

        let response: String = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            hmtl.len(),
            hmtl
        );

        return Ok(response);
    } else {
        println!("Content type requested is not yet supported");
    }
    // TODO: Fix this
    // if content_path.contains(".png") || content_path.contains(".bmp") {
    //     let image: String = base64::encode(&fs::read(content_path).map_err(|error| Error::Io(error))?);
    //     let response: String = format!(
    //         "{}\r\nContent-Type: image/bmp\r\nContent-Length: {}\r\n\r\n{}",
    //         status,
    //         image.len(),
    //         image
    //     );

    //     return Ok(response);
    // }

    return Ok(String::new());
}
