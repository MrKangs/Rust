use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    fs,
    thread,
    time::Duration
};
use multi_thread::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // This will create unlimitd threads which is not healthy for the long run
        // thread::spawn(||{
        //     handle_connection(stream);
        // });
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{}",request_line);

    let (status_line, filename) = match &request_line[..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
    stream.write_all(response.as_bytes()).unwrap();

}
