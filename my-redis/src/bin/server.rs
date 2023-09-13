#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
use my_redis::{S, LogLayer};

use std::collections::HashMap;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::my_redis::ItemServiceServer::new(
        S {
            s_box: std::sync::RwLock::new(std::cell::RefCell::new(
                my_redis::WrappedS {
                    db: HashMap::new(),
                    channel: HashMap::new(),
                }
            ))
        }
    )
    .layer_front(LogLayer)
    .run(addr)
    .await
    .unwrap();

}

