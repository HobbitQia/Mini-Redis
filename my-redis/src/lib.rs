#![feature(impl_trait_in_assoc_type)]

pub mod aof;

use aof::Command;

use volo_gen::my_redis::*;
use std::cell::RefCell;
use std::collections::HashMap;
use volo::FastStr;
use std::sync::RwLock;
use tokio::sync::broadcast::Sender;
// use anyhow::Error;
use anyhow::anyhow;

type ChannelType = HashMap<String, Sender<String>>;

pub struct S {
    pub s_box: RwLock<RefCell<WrappedS>>
}
pub struct WrappedS {
    pub db: HashMap<String, String>,
    pub channel: ChannelType,
}

unsafe impl Send for S {}
unsafe impl Sync for S {}

#[volo::async_trait]
impl volo_gen::my_redis::ItemService for S {
	async fn redis_command(
		&self, 
		req: Item
	) -> ::core::result::Result<ItemResponse, ::volo_thrift::AnyhowError> {
        match req.request_type {
            ItemType::Set => {
                self.s_box.write().unwrap().borrow_mut().db.insert(
                    req.clone().key.unwrap().into_string(),
                    req.value.clone().unwrap().into_string());
                
                // write log file
                let command = Command::Set {
                    key: req.key.unwrap().into_string(),
                    value: req.value.unwrap().into_string(),
                };
                match aof::write_command_to_aof(&command).await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }

                Ok(
                    ItemResponse {
                        response_type: ResponseType::Success,
                        value: Some("OK!".into())
                    }
                )
            }
            ItemType::Get => {
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
                let command = Command::Del {
                    key: req.key.clone().unwrap().into_string(),
                };

                match aof::write_command_to_aof(&command).await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }

                match self.s_box.write().unwrap().borrow_mut().db.remove(&req.key.unwrap().into_string()) {
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
