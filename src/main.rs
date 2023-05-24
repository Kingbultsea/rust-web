mod encoding_example;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use rust_web::ThreadPool;

fn main() {
    encoding_example::main::main();

    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        // lines默认utf8
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // 打开文件
    let file = File::open("./index.html").expect("Failed to open file");

    let metadata = file.metadata().unwrap();

    let size = metadata.len();

    let mut reader = BufReader::new(file);

    let buffer_size = 1024; // 每次写入 1024 字节

    // 读取文件并按指定大小写入 TcpStream
    let mut buffer = vec![0; buffer_size];

    stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {size}\r\n\r\n").as_bytes()).unwrap();

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // 读取完毕
            Ok(n) => {
                stream.write_all(&buffer[..n]).unwrap(); // 写入到 TcpStream
            }
            Err(err) => {
                eprintln!("Error reading file: {:?}", err);
                break;
            }
        }
    }

    // stream.write_all("\r\n\r\n".as_bytes()).unwrap();

    // println!("Request: {:#?}", http_request);
}
