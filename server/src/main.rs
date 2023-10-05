use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    println!("Hello, world!");
}

pub struct Server {
    connection: Option<Connection>,
}

impl Server {
    pub fn new() -> Self {
        Self { connection: None }
    }

    pub fn add(&mut self, conn: Connection) {
        conn.reply_tx.send(Message::JoinedAsHost).unwrap();
        self.connection = Some(conn);
    }

    fn tick(&mut self) {
        self.connection
            .as_mut()
            .map(|s| s.reply_tx.send(Message::ChangePlayers(3)).unwrap());
    }
}

pub struct ClientConnection {
    tx: Sender<ServerRequest>,
    reply_rx: Receiver<Message>,
}

pub struct Connection {
    reqest_rx: Receiver<ServerRequest>,
    reply_tx: Sender<Message>,
}
impl Connection {
    pub fn new() -> (Self, ClientConnection) {
        let (request_tx, reqest_rx) = channel();
        let (reply_tx, reply_rx) = channel();

        (
            Self {
                reply_tx,
                reqest_rx,
            },
            ClientConnection {
                reply_rx,
                tx: request_tx,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    JoinedAsHost,
    ChangePlayers(u8),
}

pub enum ServerRequest {
    ChangePlayers(u8),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Given the server is running
    /// When Alice joins
    /// Then Alice is the Host
    fn host_joins() {
        let mut server = Server::new();
        let (conn, reply) = Connection::new();

        server.add(conn);
        let response = reply.reply_rx.recv().unwrap();

        assert_eq!(response, Message::JoinedAsHost);
    }

    #[test]
    /// Given the server is running
    /// And Alice is the host
    /// When Alice tries to change the number of players to 3
    /// Then Alice is able to change the numbers of players to 3
    fn host_changes_number_of_players() {
        let mut server = Server::new();
        let (conn, reply) = Connection::new();
        server.add(conn);
        let _ = reply.reply_rx.recv().unwrap();

        reply.tx.send(ServerRequest::ChangePlayers(3)).unwrap();
        server.tick();

        let response = reply.reply_rx.recv().unwrap();
        assert_eq!(response, Message::ChangePlayers(3));
    }
}
