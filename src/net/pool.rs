use std::sync::Arc;

use tokio::sync::Semaphore;

/// Manages a pool of connections. When the pool is empty, a new connection is created
pub struct Pool(Arc<PoolInner>);

struct PoolInner {
    addr: String,
    semaphore: Semaphore,
}

impl Pool {
    pub fn new(addr: String, max_connections: usize) -> Pool {
        let inner = PoolInner {
            addr,
            semaphore: Semaphore::new(max_connections),
        };
        Self(Arc::new(inner))
    }
}