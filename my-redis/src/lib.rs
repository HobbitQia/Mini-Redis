#![feature(impl_trait_in_assoc_type)]

pub mod aof;

use aof::Command;

use volo_gen::my_redis::*;
use std::cell::RefCell;
use std::collections::HashMap;
use futures::executor::block_on;
use std::net::SocketAddr;
use volo::FastStr;
use std::sync::RwLock;
use tokio::sync::broadcast::Sender;
// use anyhow::Error;
use crc16::*;
pub mod read_file;

use anyhow::anyhow;

type ChannelType = HashMap<String, Sender<String>>;

pub struct S {
    pub s_box: RwLock<RefCell<WrappedS>>,
    pub master_type: bool,
    pub slave_vec: Vec<String>,
    pub proxy_type: bool,
}
pub struct WrappedS {
    pub db: HashMap<String, String>,
    pub channel: ChannelType,
}

unsafe impl Send for S {}
unsafe impl Sync for S {}

pub static mut PROXY_BOX: Vec<(String, String)> = Vec::new();
pub const MOD: u16 = 16383;

impl S {
    pub fn dispatch(&self, req: Item) {
        println!("len::{}", self.slave_vec.len());
        for s in &self.slave_vec {
            let addr: SocketAddr = s.parse().unwrap();
            println!("{}",addr);
            let client = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis")
                .layer_outer(LogLayer)
                .address(addr)
                .build();
            let res = block_on(client.redis_command(Item {
                key: req.key.clone(),
                value: req.value.clone(),
                expire_time: req.expire_time,
                request_type: req.request_type,
            }, true));
            // match res {
            //     Ok(_) => { println!("okkkkkkkkkkkkkkkkkkk!"); }
            //     Err(_) => {println!("shabiiiiiiiiiiiiiiiii!!!");}
            // }
        }
    }

    pub fn proxy_dispatch(&self, req: Item) -> Result<ItemResponse, ::volo_thrift::AnyhowError> {
        let hash = State::<ARC>::calculate(req.key.clone().unwrap().as_bytes()) % MOD;
        let size = unsafe { PROXY_BOX.len() } as u16;
        println!("{}",size);
        let index = hash / (MOD / size); 
        match req.request_type {
            ItemType::Set | ItemType::Del => {

                let (str, _) = unsafe { PROXY_BOX.get(index as usize) }.clone().unwrap();
                let addr: SocketAddr = str.parse().unwrap();
                let client = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis")
                .layer_outer(LogLayer)
                .address(addr)
                .build();
                let res = block_on(client.redis_command(Item {
                    key: req.key.clone(),
                    value: req.value.clone(),
                    expire_time: req.expire_time,
                    request_type: req.request_type,
                }, true));
                match res {
                    Ok(v) => {
                        Ok(v)
                    }
                    Err(e) => {
                        Err(e.into())
                    }
                }
            }
            _ => {
                println!("indexx:{}", index);
                let (_, str) = unsafe { PROXY_BOX.get(index as usize) }.clone().unwrap();
                let addr: SocketAddr = str.parse().unwrap();
                println!("addr:{}", addr);
                let client = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis")
                .layer_outer(LogLayer)
                .address(addr)
                .build();
                println!("Connecting!");
                let res = block_on(client.redis_command(Item {
                    key: req.key.clone(),
                    value: req.value.clone(),
                    expire_time: req.expire_time,
                    request_type: req.request_type,
                }, false));
                println!("Connecting222!");
                match res {
                    Ok(v) => {
                        Ok(v)
                    }
                    Err(e) => {
                        Err(e.into())
                    }
                }
            }
        }
    }
}


#[volo::async_trait]
impl volo_gen::my_redis::ItemService for S {
	async fn redis_command(
		&self, 
		req: Item,
        is_from_master: bool
	) -> ::core::result::Result<ItemResponse, ::volo_thrift::AnyhowError> {
        println!("Entetrrrrrrrrrrrrrrrr");
        if self.proxy_type == true {
            return self.proxy_dispatch(req);
        }
        match req.request_type {
            ItemType::Set => {
                if self.master_type == false && is_from_master == false {
                    return Ok(ItemResponse {
                        response_type: ResponseType::Error,
                        value: Some("You can not set values into slave server.".into())
                    });
                }

                self.s_box.write().unwrap().borrow_mut().db.insert(
                    req.clone().key.unwrap().into_string(),
                    req.value.clone().unwrap().into_string());
                
                // write log file
                let command = Command::Set {
                    key: req.key.clone().unwrap().into_string(),
                    value: req.value.clone().unwrap().into_string(),
                };
                match aof::write_command_to_aof(&command).await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }
                // println!("OUTTTTTTTTTTTT");
                if self.master_type == true {
                    // println!("TYPEEEEEEEEEEEEE");
                    self.dispatch(req);
                    // block_on(self.dispatch(req));
                }
                Ok(
                    ItemResponse {
                        response_type: ResponseType::Success,
                        value: Some("OK!".into())
                    }
                )
            }
            ItemType::Get => {
                println!("GETTTT!");
                match self.s_box.read().unwrap().borrow().db.get(&req.key.unwrap().into_string()) {
                    Some(v) => {
                        Ok(
                            ItemResponse {
                                response_type: ResponseType::Success,
                                value: Some(FastStr::from(v.clone()))
                            }
                        )
                    },
                    None => {
                        Ok(
                            ItemResponse {
                                response_type: ResponseType::Error,
                                value: Some("Key not found!".into())
                            }
                        )
                    }
                }
            }
            ItemType::Del => {
                if self.master_type == false && is_from_master == false {
                    return Ok(ItemResponse {
                        response_type: ResponseType::Error,
                        value: Some("You can not delete values from slave server.".into())
                    });
                }

                let command = Command::Del {
                    key: req.key.clone().unwrap().into_string(),
                };

                match aof::write_command_to_aof(&command).await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }
                if self.master_type == true {
                    // println!("TYPEEEEEEEEEEEEE");
                    self.dispatch(req.clone());
                    // block_on(self.dispatch(req));
                }
                match self.s_box.write().unwrap().borrow_mut().db.remove(&req.key.clone().unwrap().into_string()) {
                    Some(_) => {
                        Ok(
                            ItemResponse {
                                response_type: ResponseType::Success,
                                value: Some("OK!".into())
                            }
                        )
                    },
                    None => {
                        Ok(
                            ItemResponse {
                                response_type: ResponseType::Error,
                                value: Some("Key not found!".into())
                            }
                        )
                    }
                }
                
            }
            ItemType::Ping => {
                Ok(
                    ItemResponse {
                        response_type: ResponseType::Success,
                        value: req.value
                    }
                )
            }
            ItemType::Subscribe => { 
                let mut response_ans: ItemResponse = Default::default();
                let key = req.key.unwrap().into_string();
                let (tx, mut rx) = tokio::sync::broadcast::channel(16);
                let channel_exist ;
                {
                    match self.s_box.read().unwrap().borrow().channel.get(&key) {
                        Some(tmp) => {
                            rx = tmp.subscribe();
                            channel_exist = true;
                            
                        },
                        None => {
                            channel_exist = false;
                        }
                    }
                }
                if channel_exist {
                    let ans = rx.recv().await;
                    match ans {
                        Ok(v) => {
                            response_ans.value = Some(v.into());
                            response_ans.response_type = ResponseType::Success;
                        },
                        Err(e) => {
                            response_ans.value = Some(e.to_string().into());
                            response_ans.response_type = ResponseType::Error;
                        }
                    }
                }
                else {
                    self.s_box.write().unwrap().borrow_mut().channel.insert(key, tx);
                    let ans = rx.recv().await;
                    match ans {
                        Ok(v) => {
                            response_ans.value = Some(v.into());
                            response_ans.response_type = ResponseType::Success;
                        },
                        Err(e) => {
                            response_ans.value = Some(e.to_string().into());
                            response_ans.response_type = ResponseType::Error;
                        }
                    }
                }
                Ok(response_ans)
            }
            ItemType::Publish => {
                let mut response_ans: ItemResponse = Default::default();
                let key = req.key.unwrap().into_string();
                if let Some(tx) = self.s_box.read().unwrap().borrow().channel.get(&key) {
                    let info = tx.send(req.value.unwrap().into_string());
                    match info {
                        Ok(num) => {
                            let tmp: String = "Subscibers: ".to_string() + &num.to_string();
                            response_ans.value = Some(tmp.into());
                            response_ans.response_type = ResponseType::Success;
                        }
                        Err(e) => {
                            response_ans.value = Some(e.to_string().into());
                            response_ans.response_type = ResponseType::Error;
                        }
                    }
                }
                else {
                    response_ans.value = Some("Nobody subscribes.".to_string().into());
                    response_ans.response_type = ResponseType::Error;
                }
                Ok(response_ans)
            }
        }
    }
}



#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
    
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        // let now = std::time::Instant::now();
        // tracing::debug!("Received request {:?}", &req);
        // let resp = self.0.call(cx, req).await;
        // tracing::debug!("Sent response {:?}", &resp);
        // tracing::info!("Request took {}ms", now.elapsed().as_millis());
        // resp

        let info = format!("{:?}", req);
		if info.contains("shabi") {
            Err(anyhow!("No dirty word, please!").into())
			// Err(S::Error::from(Error::msg("Genshin is not allowed")))
		} else {
			self.0.call(cx, req).await
		}
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}
