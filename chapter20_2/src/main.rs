use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use chapter20_2::ThreadPool;

fn main() {
    // Listening to the TCP Connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Creating a Finite Number of Threads
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {

        // Spawning a Thread for Each Request
        let stream = stream.unwrap();

        // thread::spawn(|| {
        //     handle_connection(stream);
        // });

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Simulating a Slow Request in the Current Server Implementation
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "C:\\Users\\dev\\Workspace\\rust-playground\\chapter20_2\\hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "C:\\Users\\dev\\Workspace\\rust-playground\\chapter20_2\\hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "C:\\Users\\dev\\Workspace\\rust-playground\\chapter20_2\\404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}