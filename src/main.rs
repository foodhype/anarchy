use std::io::Timer;
use std::time::Duration;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn main() {  
    spawn(proc() {
        let listener = TcpListener::bind("127.0.0.1", 8001);

        // bind the listener to the specified address
        let mut acceptor = listener.listen();

        // accept connections and process them, spawning a new tasks for each one
        for stream in acceptor.incoming() {
            match stream {
                Err(e) => {
                    println!("{}", e);
                }
                Ok(stream) => spawn(proc() {
                })
            }
        }

        // close the socket server
        drop(acceptor);
    });

    spawn(proc() {
        let mut timer = Timer::new().unwrap();
        timer.sleep(Duration::milliseconds(400));
        let mut socket = TcpStream::connect("127.0.0.1", 8001);
        loop {
            let response = socket.read_to_end();
            println!("{}", response);
            timer.sleep(Duration::milliseconds(300));
        }
    });

}
