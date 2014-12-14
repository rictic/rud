use std::io;
use std::io::{TcpListener, TcpStream, Listener, Acceptor};


fn handle(mut socket : TcpStream) {
    println!("I'm in a thread handling this connection");
    let mut reader = io::BufferedReader::new(socket.clone());
    socket.write(b">> ").unwrap();
    for _ in reader.lines() {
        socket.write(b"You are a very lucky winner!\n").unwrap();
        socket.write(b">> ").unwrap();
    }
}

fn main() {
    let a = TcpListener::bind("127.0.0.1:8482").listen().unwrap();

    spawn(proc() {
        let mut a2 = a.clone();
        for socket in a2.incoming() {
            match socket {
                Ok(s) => {
                    spawn(proc() {handle(s)})
                },
                Err(ref e) if e.kind == io::IoErrorKind::EndOfFile => break,
                Err(e) => panic!("unexpected error: {}", e),
            }
        }
    });

    // Now that our accept loop is running, wait for the program to be
    // requested to exit.
    println!("Now listening!!");
}
