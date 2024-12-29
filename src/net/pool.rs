use super::{Connection, Error};

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use async_channel::{Receiver, Sender, TryRecvError};

/// Manages a pool of connections. When the pool is empty, a new connection is created
pub struct Pool(Arc<PoolInner>);

impl Pool {
    #[must_use] pub fn new(addr: String) -> Self {
        let (sender, receiver) = async_channel::unbounded();
        let inner = PoolInner {
            addr: addr.into(),
            receiver,
            sender,
        };
        Self(Arc::new(inner))
    }

    pub async fn acquire(&self) -> Result<Handle, Error> {
        self.0.acquire().await
    }
}

pub struct Handle(Option<HandleInner>);

impl Deref for Handle {
    type Target = Connection;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for Handle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

struct HandleInner {
    conn: Connection,
    sender: Sender<Connection>,
}

impl Deref for HandleInner {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for HandleInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let Some(HandleInner { conn, sender }) = self.0.take() else {
            return;
        };
        sender.try_send(conn).unwrap();
    }
}

struct PoolInner {
    addr: Arc<str>,
    sender: Sender<Connection>,
    receiver: Receiver<Connection>,
}

impl PoolInner {
    async fn acquire(self: &Arc<Self>) -> Result<Handle, Error> {
        let conn = match self.receiver.try_recv() {
            Ok(conn) => Ok(conn),
            Err(TryRecvError::Empty) => Connection::new(&self.addr).await,
            Err(err) => return Err(err.into()),
        }?;

        let inner = HandleInner {
            conn,
            sender: self.sender.clone(),
        };

        Ok(Handle(Some(inner)))
    }
}
