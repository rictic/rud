use std::io;
use std::sync::{Mutex, Arc};
use std::io::{TcpListener, TcpStream, Listener, Acceptor};
use std::io::net::tcp::TcpAcceptor;
use std::comm::{Receiver, Sender, channel};

struct Server {
    #[allow(dead_code)]
    game: GameWorld,
    port: u16,
    acceptor: TcpAcceptor,
    players: Arc<Mutex<Vec<PlayerHandle>>>
}

struct PlayerHandle {
    channel: Sender<()>,
}

#[deriving(Clone, Send)]
struct GameWorld;


enum Command {
    Look,
}

impl Server {
    fn new(host: &str, port: u16) -> io::IoResult<Server> {
        let a : TcpAcceptor = try!(TcpListener::bind((host, port)).unwrap().listen());
        let game = GameWorld::new();
        let players = Arc::new(Mutex::new(Vec::new()));
        let server = Server {
            acceptor: a.clone(), game: game.clone(), port: port,
            players: players.clone(),
        };
        spawn(proc() {
            let mut a2 = a.clone();
            for socket in a2.incoming() {
                match socket {
                    Ok(s) => {
                        let game_clone = game.clone();
                        let (snd, rec) = channel::<()>();
                        spawn(proc() {game_clone.handle(s, rec)});
                        players.clone().lock().push(PlayerHandle {channel: snd});
                    },
                    Err(ref e) if e.kind == io::IoErrorKind::EndOfFile => break,
                    Err(e) => panic!(
                        "unexpected error accepting connection: {}", e),
                }
            }
        });
        Ok(server)
    }
    #[cfg(test)]
    fn new_test_server() -> Server {
        use std::rand::random;
        for _ in range(0u, 100) {
            match Server::new("127.0.0.1", random()) {
                Ok(s) => {
                    return s
                },
                Err(_) => (), // try again!
            }
        }
        panic!("Unable to bind to a port for the test server!");
    }
    fn close(&mut self) {
        let _ = self.acceptor.close_accept();
        let players = self.players.lock();
        for player in players.iter() {
            let _ = player.channel.send_opt(());
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.close();
        println!("Dropped server");
    }
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld // lol
    }

    pub fn handle(&self, mut socket : TcpStream, close_rc: Receiver<()>) {
        let line_rc : Receiver<String> = GameWorld::get_lines(socket.clone());
        socket.write(b">> ").unwrap();

        loop {
            select! {
                ln = line_rc.recv_opt() => {
                    let line = match ln {
                        Ok(l) => l,
                        Err(_) => return, // connection closed by user
                    };
                    let response = match self.parse(line) {
                        Some(Command::Look) =>
                            "You are in a dark and spooky cave. You are likely to be eaten by a grue.\n",
                        None => "I have no idea what you just said there chief.\n"
                    };
                    socket.write(response.as_bytes()).unwrap();
                    socket.write(b">> ").unwrap();
                },
                () = close_rc.recv() => {
                    let _ = socket.write(b"\n\nServer is going down hard.\n");
                    let _ = socket.close_read();
                    let _ = socket.close_write();
                    return;
                }
            }
        }
    }

    pub fn get_lines<R: Reader + Send>(reader: R) -> Receiver<String> {
        let (snd, rcv) = channel();
        spawn(proc() {
            for line in io::BufferedReader::new(reader).lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => {
                        continue;
                    }
                };
                snd.send(line);
            }
        });
        rcv
    }

    pub fn parse(&self, s : String) -> Option<Command> {
        println!("{}", s);
        match s.as_slice().trim() {
            "l" | "look" => Some(Command::Look),
            _ => None
        }
    }
}

#[test]
fn test_connect_and_disconnect() {
    let server = Server::new_test_server();
    let mut stream = TcpStream::connect(("127.0.0.1", server.port));
    stream.write(b"look\n").unwrap();
    let mut reader = io::BufferedReader::new(stream);
    let line = reader.read_line().unwrap();
    assert_eq!(">> You are in a dark and spooky cave. You are likely to be eaten by a grue.\n", line);
}

#[cfg(not(test))]
fn main() {
    let mut s = Server::new("127.0.0.1", 8482).unwrap();
    // Now that our accept loop is running, wait for the program to be
    // requested to exit.
    println!("Listening on port {}", s.port);
    println!("Admin shell:");
    print!(">> ");
    for line in io::BufferedReader::new(io::stdio::stdin()).lines() {
        if let Ok(line) = line {
            if line.trim() == "exit" {
                s.close();
                break;
            }
        }
        print!(">> ");
    }
}
