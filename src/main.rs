use std::io::Timer;
use std::time::Duration;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn main() {
    let mut timer = Timer::new().unwrap();    
    start_server();
    // TODO: The following line depends on time, which is bad.
    timer.sleep(Duration::milliseconds(1000));
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
    for _ in range(0i, 10i) {
        stream.write_line("echo");
    }
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

