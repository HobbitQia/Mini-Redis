use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Set { key: String, value: String },
    Del { key: String },
}

impl Command {
    fn to_string(&self) -> String {
        match self {
            Command::Set { key, value } => format!("SET {} {}\n", key, value),
            Command::Del { key } => format!("DEL {}\n", key),
        }
    }

    fn from_string(s: &str) -> Option<Command> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        let cmd = match parts[0] {
            "SET" => {
                if parts.len() >= 3 {
                    Command::Set {
                        key: parts[1].to_string(),
                        value: parts[2..].join(" "),
                    }
                } else {
                    return None;
                }
            }
            "DEL" => {
                Command::Del {
                    key: parts[1].to_string(),
                }
            }
            _ => return None,
        };
        Some(cmd)
    }
}

pub async fn write_command_to_aof(command: &Command, path: String) -> Result<(), tokio::io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .await?;

    let cmd_string = command.to_string();
    file.write_all(cmd_string.as_bytes()).await?;

    
    Ok(())
}

pub async fn recover_from_aof(path: String) -> Result<Vec<Command>, std::io::Error> {
    // println!("In recover func: {:?}", path);
    let file = match File::open(path).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return Ok(Vec::new());
        }
    };
        
    let reader = BufReader::new(file);
    let mut commands = Vec::new();

    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if let Some(cmd) = Command::from_string(&line) {
            commands.push(cmd);
        }
    }

    Ok(commands)
}

lazy_static::lazy_static! {
    // #[derive(Debug)]
    pub static ref CWD: PathBuf = std::env::current_dir().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn main() {
        println!("cwd: {:?}", *CWD);
        // let command1 = Command::Set {
        //     key: "ZJU".to_string(),
        //     value: "ZHE".to_string(),
        // };
        // let command2 = Command::Set {
        //     key: "key2".to_string(),
        //     value: "value2".to_string(),
        // };
        // write_command_to_aof(&command1).await.unwrap();
        // write_command_to_aof(&command2).await.unwrap();
    }
}