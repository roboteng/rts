use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

fn main() {
    println!("Hello, world!");
}

#[derive(Default)]
pub struct Server {
    connection: Option<Connection>,
}

impl Server {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, conn: Connection) {
        conn.reply_tx.send(Message::JoinedAsHost).unwrap();
        self.connection = Some(conn);
    }

    pub fn tick(&mut self) {
        if let Some(r) = &self.connection {
            match r.reqest_rx.try_recv() {
                Ok(ServerRequest::ChangePlayers(n)) => {
                    r.reply_tx.send(Message::ChangePlayers(n)).unwrap();
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => self.connection = None,
            };
        }
    }
}

#[allow(unused)]
fn two_way_channe() -> (Connection, ClientConnection) {
    let (request_tx, reqest_rx) = channel();
    let (reply_tx, reply_rx) = channel();
    (
        Connection {
            reply_tx,
            reqest_rx,
        },
        ClientConnection {
            reply_rx,
            tx: request_tx,
        },
    )
}

#[allow(unused)]
pub struct ClientConnection {
    tx: Sender<ServerRequest>,
    reply_rx: Receiver<Message>,
}

pub struct Connection {
    reqest_rx: Receiver<ServerRequest>,
    reply_tx: Sender<Message>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    JoinedAsHost,
    ChangePlayers(u8),
}

#[derive(Debug)]
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
        let (conn, reply) = two_way_channe();

        server.add(conn);
        let response = reply.reply_rx.recv().unwrap();
        server.tick();

        assert_eq!(response, Message::JoinedAsHost);
    }

    #[test]
    /// Given the server is running
    /// And Alice is the host
    /// When Alice tries to change the number of players to 3
    /// Then Alice is able to change the numbers of players to 3
    fn host_changes_number_of_players() {
        let mut server = Server::new();
        let (conn, reply) = two_way_channe();
        server.add(conn);
        let _ = reply.reply_rx.recv().unwrap();

        reply.tx.send(ServerRequest::ChangePlayers(3)).unwrap();
        server.tick();

        let response = reply.reply_rx.try_recv().unwrap();
        assert_eq!(response, Message::ChangePlayers(3));
    }
}
