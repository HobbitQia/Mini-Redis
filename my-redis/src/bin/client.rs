// use lazy_static::lazy_static;
use std::net::SocketAddr;
use my_redis::LogLayer;
use volo::FastStr;
use std::sync::Arc;
use volo_gen::my_redis::{Item, ItemType};
use volo_gen::my_redis::ResponseType;
use my_redis::read_file;

// lazy_static! {
//     static ref CLIENT: volo_gen::my_redis::ItemServiceClient = {
//         let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//         volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis")
//             .layer_outer(LogLayer)
//             .address(addr)
//             .build()
//     };
// }

#[volo::main]
async fn main() {
    
    tracing_subscriber::fmt::init();
    // let req = volo_gen::volo::example::GetItemRequest { id: 0 };
    let opcode;
    let mut args: Vec<String> = std::env::args().collect();
    let (config, pattern) = read_file::read_file(
        String::from("/home/ljy/code/Mini-Redis/my-redis/src/config.txt")
    );


    let string_name = args[1].to_string();
    let mut str: String = String::new();
    let index;
    if pattern == "cluster" {
        index = 2;
        for i in config {
            if i._type == "proxy" {
                str = format!("{}:{}", i.host, i.port);
                break;
            }
        }
        
    }
    else {
        index = 3;
        for i in config {
            if i.name == string_name {
                str = format!("{}:{}", i.host, i.port);
                break;
            }
        }
    }
    let addr: SocketAddr = str.parse().unwrap();
    let client = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis")
        .layer_outer(LogLayer)
        .address(addr)
        .build();

    
    let request = match args[index-1].to_uppercase().as_str() {
        "GET" => {
            opcode = 1;
            Item {
                key: Some(FastStr::from(Arc::new(args.remove(index)))),
                value: None,
                request_type: ItemType::Get,
                expire_time: None
            }
        },
        "SET" => {
            opcode = 2;
            Item {
                key: Some(FastStr::from(Arc::new(args.remove(index)))),
                value: Some(FastStr::from(Arc::new(args.remove(index)))),
                request_type: ItemType::Set,
                expire_time: None
            }
        },
        "DEL" => {
            opcode = 3;
            Item {
                key: Some(FastStr::from(Arc::new(args.remove(index)))),
                value: None,
                request_type: ItemType::Del,
                expire_time: None
            }
        },
        "PING" => {
            opcode = 4;
            let value = { 
                    if args.len() > 2 {
                        Some(FastStr::from(Arc::new(args.remove(index))))
                    }
                    else { Some("Pong".into()) } 
            };
            Item {
                key: None,
                value,
                request_type: ItemType::Ping,
                expire_time: None
            }
        },
        "SUBSCRIBE" => {
            opcode = 5;
            println!("Waiting for messages to be issued...");
            Item {
                key: Some(FastStr::from(Arc::new(args.remove(index)))),
                value: None,
                request_type: ItemType::Subscribe,
                expire_time: None
            }
        },
        "PUBLISH" => {
            opcode = 6;
            Item {
                key: Some(FastStr::from(Arc::new(args.remove(index)))),
                value: Some(FastStr::from(Arc::new(args.remove(index)))),
                request_type: ItemType::Publish,
                expire_time: None
            }
        },
        _ => {
            panic!("invalid request type");
        }
    };

    let resp = client.redis_command(request, false).await;
    match resp {
        Ok(info) => {
            match info.response_type {
                ResponseType::Success => {
                    println!("OK!");
                    if opcode == 1  {
                        println!("Value: {:?}", info.value.unwrap());
                    }
                    else if opcode == 5 || opcode == 6 || opcode == 4 {
                        println!("{:?}", info.value.unwrap());
                    }
                }
                ResponseType::Error => {
                    println!("Some error happens: {:?}", info.value.unwrap());
                }
            }
            // tracing::info!("{:?}", info);carog
        }
        Err(e) => tracing::error!("{:?}", e.to_string()),
    }
    
}

