#![feature(impl_trait_in_assoc_type)]

use my_redis::aof::Command;
use my_redis::aof::recover_from_aof;

use std::net::SocketAddr;
use my_redis::{S, LogLayer};

use std::collections::HashMap;
pub mod read_file;

use std::path::PathBuf;
lazy_static::lazy_static! {
    // #[derive(Debug)]
    pub static ref CWD: PathBuf = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
}

#[volo::main]
async fn main() {
    let mut path = (*CWD).clone();
    path.push("../../src/log/log.aof");
    println!("{:?}",path);
    let commands =  match recover_from_aof(path.to_str().unwrap().to_string()).await {
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

    println!("{:?}", db);

    let config = read_file::read_file(
        String::from("/home/ljy/code/Mini-Redis/my-redis/src/config.txt")
    );

    let args: Vec<String> = std::env::args().collect();
    let string_name = args[1].to_string();
    
    for i in &config {
        if string_name == i.name {
            let str = format!("{}:{}", i.host, i.port);
            let addr: SocketAddr = str.parse().unwrap();
            let addr = volo::net::Address::from(addr);
            let mut slave_vec = Vec::<String>::new();
            if i._type == "master" {
                for j in &config {
                    // j.show();
                    if j.master_host == i.host && j.master_port == i.port {
                        slave_vec.push(format!("{}:{}", j.host, j.port));
                    }
                }
            }
            volo_gen::my_redis::ItemServiceServer::new(
                    S {
                        s_box: std::sync::RwLock::new(std::cell::RefCell::new(
                            my_redis::WrappedS {
                                db: db.clone(),
                                channel: HashMap::new(),
                            }
                        )),
                        master_type: i._type == "master",
                        slave_vec
                    }
                )
                .layer_front(LogLayer)
                .run(addr)
                .await
                .unwrap();
        }
    }

    

}

