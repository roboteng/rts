use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    println!("Hello, world!");
}

#[derive(Default)]
pub struct Server {
    connection: Vec<Connection>,
}

impl Server {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, conn: Connection) {
        let message = if self.connection.is_empty() {
            Message::JoinedAsHost
        } else {
            Message::JoinedAsGuest
        };
        conn.reply_tx.send(message).unwrap();
        self.connection.push(conn);
    }

    pub fn tick(&mut self) {
        let messages = self.collect_messages();
        self.process_messages(messages);
    }

    fn collect_messages(&mut self) -> Vec<(usize, ServerRequest)> {
        let mut messages = Vec::new();
        for (index, r) in self.connection.iter().enumerate() {
            while let Ok(message) = r.reqest_rx.try_recv() {
                messages.push((index, message));
            }
        }
        messages
    }

    fn process_messages(&mut self, messages: Vec<(usize, ServerRequest)>) {
        for (index, message) in messages.into_iter() {
            let r = &mut self.connection[index];
            match message {
                ServerRequest::ChangePlayers(n) => {
                    r.reply_tx.send(Message::ChangePlayers(n)).unwrap();
                }
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
    JoinedAsGuest,
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
        server.tick();
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
        let (conn, reply) = two_way_channe();
        server.add(conn);
        let _ = reply.reply_rx.recv().unwrap();

        reply.tx.send(ServerRequest::ChangePlayers(3)).unwrap();
        server.tick();

        let response = reply.reply_rx.try_recv().unwrap();
        assert_eq!(response, Message::ChangePlayers(3));
    }

    #[test]
    /// Given the server is running
    /// And Alice is the Host
    /// When Bob joins
    /// Then Bob is a Guest
    fn guest_joins() {
        let mut server = Server::new();
        let (alice, _reply) = two_way_channe();

        server.add(alice);
        server.tick();

        let (bob, bob_reply) = two_way_channe();
        server.add(bob);
        server.tick();

        let res = bob_reply.reply_rx.try_recv();

        assert_eq!(res, Ok(Message::JoinedAsGuest))
    }
}
