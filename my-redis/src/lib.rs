#![feature(impl_trait_in_assoc_type)]
use std::path::PathBuf;
lazy_static::lazy_static! {
    // #[derive(Debug)]
    pub static ref CWD: PathBuf = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
}

pub mod aof;

use aof::Command;

use volo_gen::my_redis::*;
use std::cell::RefCell;
use std::collections::HashMap;
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
    pub proxy_box: Vec<(volo_gen::my_redis::ItemServiceClient, volo_gen::my_redis::ItemServiceClient)>,
    pub proxy_type: bool,
    pub server_index: char,
}
pub struct WrappedS {
    pub db: HashMap<String, String>,
    pub channel: ChannelType,
}

unsafe impl Send for S {}
unsafe impl Sync for S {}

// pub static mut PROXY_BOX: Vec<(volo_gen::my_redis::ItemServiceClient, volo_gen::my_redis::ItemServiceClient)> = Vec::new();
pub const MOD: u16 = 16383;

impl S {
    pub async fn dispatch(&self, req: Item) {
        println!("len::{}", self.slave_vec.len());
        for s in &self.slave_vec {
            let addr: SocketAddr = s.parse().unwrap();
            println!("{}",addr);
            let client = volo_gen::my_redis::ItemServiceClientBuilder::new("my-redis")
                .layer_outer(LogLayer)
                .address(addr)
                .build();
            let _ = client.redis_command(Item {
                key: req.key.clone(),
                value: req.value.clone(),
                expire_time: req.expire_time,
                request_type: req.request_type,
            }, true).await;
        }
    }

    pub async fn proxy_dispatch(&self, req: Item) -> Result<ItemResponse, ::volo_thrift::AnyhowError> {
        let hash = State::<ARC>::calculate(req.key.clone().unwrap().as_bytes()) % MOD;
        let size = self.proxy_box.len() as u16;
        // println!("{}",size);
        let index = hash / (MOD / size); 
        match req.request_type {
            ItemType::Set | ItemType::Del => {
                // println!("set index:{}", index);
                let client = self.proxy_box.get(index as usize).clone().unwrap().0.clone();
                let res = client.redis_command(Item {
                    key: req.key.clone(),
                    value: req.value.clone(),
                    expire_time: req.expire_time,
                    request_type: req.request_type,
                }, false).await;
                match res {
                    Ok(v) => {
                        return Ok(v);
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
            _ => {
                // println!("get index:{}", index);
                let client = self.proxy_box.get(index as usize).clone().unwrap().1.clone();
                let res = client.redis_command(Item {
                    key: req.key.clone(),
                    value: req.value.clone(),
                    expire_time: req.expire_time,
                    request_type: req.request_type,
                }, false).await;
                // println!("Connecting222!");
                match res {
                    Ok(v) => {
                        return Ok(v);
                    }
                    Err(e) => {
                        return Err(e.into());
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
	) -> ::core::result::Result<ItemResponse, ::volo_thrift::AnyhowError> 
    where 

    {
        // println!("Entetrrrrrrrrrrrrrrrr");
        if self.proxy_type == true {
            return self.proxy_dispatch(req).await;
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
                
                
                if self.master_type == true {
                    let mut path = (*CWD).clone();
                    path.push(format!("../../src/log/log_{}.aof", self.server_index));
                    match aof::write_command_to_aof(&command, path.to_str().unwrap().to_string()).await {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                    self.dispatch(req).await;
                }
                Ok(
                    ItemResponse {
                        response_type: ResponseType::Success,
                        value: Some("OK!".into())
                    }
                )
            }
            ItemType::Get => {
                // println!("GETTTT!");
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

                if self.master_type == true {
                    let mut path = (*CWD).clone();
                    path.push(format!("../../src/log/log_{}.aof", self.server_index));
                    match aof::write_command_to_aof(&command, path.to_str().unwrap().to_string()).await {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                    self.dispatch(req.clone()).await;
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
