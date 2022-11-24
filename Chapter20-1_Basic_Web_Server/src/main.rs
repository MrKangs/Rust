use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    fs
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    
    }
}

fn handle_connection(mut stream: TcpStream) {

    // You cannot have two buffer_reader (BufReader) to call the stream

    //----- Fetching the Get request -----

    // let buffer_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buffer_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    
    // println!("{:?}", http_request);

    // ---- Read and send the response back to the website -----

    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    // stream.write_all(response.as_bytes()).unwrap();

    // ----- Reading the file source -----

    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("hello.html").unwrap();
    // let length = contents.len();

    // let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);

    // stream.write_all(response.as_bytes()).unwrap();

    // ----- Reading the file source with conditions based on GET method -----

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
    stream.write_all(response.as_bytes()).unwrap();

    // ----- Unfactored code from the previous block -----

    // if request_line == "GET / HTTP/1.1"{
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();

    //     let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);

    //     stream.write_all(response.as_bytes()).unwrap();
    // } else{
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);

    //     stream.write_all(response.as_bytes()).unwrap();
    // }




    
}
