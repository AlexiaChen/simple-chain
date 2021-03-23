use std::string::String;
use tokio::net::{TcpListener, TcpStream};
pub struct Server {
    pub version: String,
}

impl Server {
    pub fn new() -> Self {
        return Server {
            version: String::from("0.0.1"),
        };
    }

    pub async fn start_server(
        &self,
        node_id: &'static str,
        miner_addr: &'static str,
    ) {
        let listen_end_point = format!("0.0.0.0:{}", node_id);
        let listener = TcpListener::bind(listen_end_point).await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            self.process(socket).await;
        }
    }

    async fn process(&self, socket: TcpStream) {}
}

pub fn start_server(node_id: &'static str, miner_addr: &'static str) {
    let server: Server = Server::new();

    tokio::spawn(async move {
        server.start_server(node_id, miner_addr).await;
    });
}
