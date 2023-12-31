#![feature(impl_trait_in_assoc_type)]

// use my_redis::PROXY_BOX;
use my_redis::aof::Command;
use my_redis::aof::recover_from_aof;
use my_redis::read_file;
use volo_gen::my_redis::Item;
use volo_gen::my_redis::ItemType;

use std::net::SocketAddr;
use my_redis::{S, LogLayer};

use std::path::PathBuf;
lazy_static::lazy_static! {
    // #[derive(Debug)]
    pub static ref CWD: PathBuf = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
}

use std::collections::HashMap;

#[volo::main]
async fn main() {

    let (config, pattern) = read_file::read_file(
        String::from("/home/ljy/code/Mini-Redis/my-redis/src/config.txt")
    );

    let mut proxy_box:Vec<(volo_gen::my_redis::ItemServiceClient, volo_gen::my_redis::ItemServiceClient)>
     = Vec::new();

    let args: Vec<String> = std::env::args().collect();
    let string_name = args[1].to_string();

    let index = 
    if pattern == "cluster" { string_name.as_bytes()[string_name.len()-1 as usize] as char }
    else { '0' };

    // unsafe {
        if pattern == "cluster" && proxy_box.is_empty() && string_name == "proxy" {
            for i in &config {
                for j in &config {
                    if i._type == "master" && j._type == "slave" && j.master_host == i.host && j.master_port == i.port {
                        let str = format!("{}:{}", i.host, i.port);
                        let addr: SocketAddr = str.parse().unwrap();
                        let client1 = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis-3213")
                        .layer_outer(LogLayer)
                        .address(addr)
                        .build();
                    println!("addr1:{}", addr);
                        let str = format!("{}:{}", j.host, j.port);
                        let addr: SocketAddr = str.parse().unwrap();
                        let client2 = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis-3213")
                        .layer_outer(LogLayer)
                        .address(addr)
                        .build();
                    
                    println!("addr2:{}", addr);
                        proxy_box.push((client1.clone(), client2.clone()));

                    }
                }
            }
        }
    // }
    
    let mut path = (*CWD).clone();
    path.push(format!("../../src/log/log_{}.aof", index));
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

    let mut flag: bool = false;
    let mut tmp_vec: Vec<String> = Vec::<String>::new();

    for i in &config {
        if string_name == i.name {
            let str = format!("{}:{}", i.host, i.port);
            let addr: SocketAddr = str.parse().unwrap();
            let addr = volo::net::Address::from(addr);
            let mut slave_vec = Vec::<String>::new();
            if i._type == "master" {
                flag = true;
                for j in &config {
                    // j.show();
                    if j.master_host == i.host && j.master_port == i.port {
                        slave_vec.push(format!("{}:{}", j.host, j.port));
                    }
                }
            }
            tmp_vec = slave_vec.clone();
            volo_gen::my_redis::ItemServiceServer::new(
                    S {
                        s_box: std::sync::RwLock::new(std::cell::RefCell::new(
                            my_redis::WrappedS {
                                db: db.clone(),
                                channel: HashMap::new(),
                            }
                        )),
                        master_type: i._type == "master",
                        slave_vec,
                        proxy_type: i._type == "proxy",
                        proxy_box: if i._type == "proxy" { proxy_box.clone() } else { Vec::new() },
                        server_index: index
                    }
                )
                .layer_front(LogLayer)
                .run(addr)
                .await
                .unwrap();

            break;
        }
    }
    
    if pattern == "cluster" && string_name == "proxy" {
        println!("proxy exit");
        for (i, j) in proxy_box {
            let _ = i.redis_command(Item {
                key: None,
                value: None,
                expire_time: Some(-1),
                request_type: ItemType::Set,
            }, false).await;
            // println!("kill 1 ok");
            let _ = j.redis_command(Item {
                key: None,
                value: None,
                expire_time: Some(-1),
                request_type: ItemType::Set,
            }, true).await;
        }
    }
    else if flag {
        for i in tmp_vec {
            let addr: SocketAddr = i.parse().unwrap();
            let client = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis-3213")
            .layer_outer(LogLayer)
            .address(addr)
            .build();
            let _ = client.redis_command(Item {
                key: None,
                value: None,
                expire_time: Some(-1),
                request_type: ItemType::Set,
            }, true).await;
        }
    }

}
