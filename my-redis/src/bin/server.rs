#![feature(impl_trait_in_assoc_type)]

use my_redis::aof::Command;
use my_redis::aof::recover_from_aof;

use std::net::SocketAddr;
use my_redis::{S, LogLayer};

use std::collections::HashMap;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    let commands =  match recover_from_aof().await {
        Ok(commands) => {
            commands
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Vec::new()
        }
    };
        
    let mut db = HashMap::new();
    for command in commands {
        match command {
            Command::Set { key, value } => {
                db.insert(key, value);
            }
            Command::Del { key } => {
                db.remove(&key);
            }
        }
    }

    volo_gen::my_redis::ItemServiceServer::new(
        S {
            s_box: std::sync::RwLock::new(std::cell::RefCell::new(
                my_redis::WrappedS {
                    db: db,
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

