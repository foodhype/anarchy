use std::io::Timer;
use std::time::Duration;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn main() {
    let mut timer = Timer::new().unwrap();    
    start_server();
    timer.sleep(Duration::milliseconds(500));
    start_client();
}


fn start_server() {
    spawn(proc() {
        let listener = TcpListener::bind("127.0.0.1", 8000);
        let mut acceptor = listener.listen();
        for stream in acceptor.incoming() {
            match stream {
                Ok(stream) => spawn(proc() {
                    handle_client(stream)
                }),
                Err(error) => {
                    println!("{}", error);
                }
            }
        }

        drop(acceptor);
    });
}


fn handle_client(mut stream:TcpStream) {
    stream.write(b"echo");
}


fn start_client() {
    spawn(proc() {
        let mut socket = TcpStream::connect("127.0.0.1", 8000);
        let response = socket.read_to_end();
        match response {
            Ok(value) => {
                println!("{}", String::from_utf8(value));
            },
            Err(error) => {
                println!("{}", error);
            }
        };
    });
}

