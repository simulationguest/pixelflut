use super::{Connection, Error};

use std::sync::Arc;

use async_channel::{Receiver, Sender};

/// Manages a pool of connections. When the pool is empty, a new connection is created
pub struct Pool(Arc<PoolInner>);

impl Pool {
    pub fn new(addr: String) -> Pool {
        let (sender, receiver) = async_channel::unbounded();
        let inner = PoolInner {
            addr: addr.into(),
            receiver,
            sender,
        };
        Self(Arc::new(inner))
    }

    pub async fn acquire(&self) -> Result<Connection, Error> {
        self.0.acquire().await
    }
}
struct PoolInner {
    addr: Arc<str>,
    sender: Sender<Connection>,
    receiver: Receiver<Connection>,
}

impl PoolInner {
    async fn acquire(self: &Arc<Self>) -> Result<Connection, Error> {
        match self.receiver.recv().await {
            Ok(conn) => Ok(conn),
            Err(_) => Connection::new(&self.addr).await,
        }
    }
}
