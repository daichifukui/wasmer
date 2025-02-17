use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use super::{hyper_proxy::HyperProxyConnector, socket_manager::SocketManager};

#[derive(Debug, Clone)]
pub struct DProxyInstance {
    #[allow(unused)]
    pub(super) last_used: Arc<Mutex<Instant>>,
    #[allow(unused)]
    pub(super) socket_manager: Arc<SocketManager>,
    pub(super) client: hyper::Client<HyperProxyConnector>,
}
