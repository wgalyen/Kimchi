use client::Client;
use deadpool::unmanaged::Pool;
use tokio::sync::mpsc;

use crate::uri;
use crate::{client, types};

pub struct ClientPool {
    tx: mpsc::Sender<types::Response>,
    rx: mpsc::Receiver<uri::Uri>,
    pool: deadpool::unmanaged::Pool<client::Client>,
}

impl ClientPool {
    pub fn new(
        tx: mpsc::Sender<types::Response>,
        rx: mpsc::Receiver<uri::Uri>,
        clients: Vec<Client>,
    ) -> Self {
        let pool = Pool::from(clients);
        ClientPool { tx, rx, pool }
    }

    pub async fn listen(&mut self) {
        while let Some(req) = self.rx.recv().await {
            let client = self.pool.get().await;
            let tx = self.tx.clone();
            tokio::spawn(async move {
                let resp = client.check(req).await;
                tx.send(resp).await.unwrap();
            });
        }
    }
}
