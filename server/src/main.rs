use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    println!("Hello, world!");
}

pub struct Server;
impl Server {
    pub fn new() -> Self {
        Self
    }

    pub fn add(&mut self, conn: Connection) {
        conn.0.send(Message::Host).unwrap();
    }
}

pub struct Connection(Sender<Message>);
impl Connection {
    pub fn new() -> (Self, Receiver<Message>) {
        let (tx, rx) = channel();
        (Self(tx), rx)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Host,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Given the server is running
    /// When Alice joins
    /// Then Alice is the Host
    // /// And Alice is able to change the numbers of players to 3
    fn host_joins() {
        let mut server = Server::new();
        let (conn, reply) = Connection::new();

        server.add(conn);
        let response = reply.recv().unwrap();

        assert_eq!(response, Message::Host);
    }
}
