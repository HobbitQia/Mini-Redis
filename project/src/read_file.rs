use std::io::Read;

pub struct Config {
    name: String,
    _type: String,
    host: String,
    port: String,
    master_host: String,
    master_port: String, 
}

impl Config {
    pub fn new() -> Config {
        Config {
            name: String::from(""),
            _type: String::from(""),
            host: String::from(""),
            port: String::from(""),
            master_host: String::from(""),
            master_port: String::from(""),
        }
    }
    pub fn show(&self) {
        println!("name: {}",self.name);
        println!("type: {}",self._type);
        println!("host: {}",self.host);
        println!("port: {}",self.port);
        println!("master_host: {}",self.master_host);
        println!("master_port: {}",self.master_port);
    }
}

fn escape_null(buf: &str,mut i: usize) -> usize {
    while i < buf.len(){
        let c = buf.as_bytes()[i] as char;
        if c=='\n'||c=='\r'||c==' '||c==0 as char {
            i += 1;
            continue;
        }else{
            break;
        }
    }
    return i;
}

fn read_master(buf: &str, mut i: usize) -> (Config, usize) {
    let len = buf.len();

    let mut name = String::from("");
    let mut port = String::from("");
    let mut _type = String::from("");
    let mut host = String::from("");

    while i < len {
        i = escape_null(buf, i);
        let tmp_buf = &buf[i..];
        if tmp_buf.starts_with("name:") {
            i += 5;
            i = escape_null(buf, i);
            
            while i < len && buf.as_bytes()[i] as char != '\n' {
                name.push(buf.as_bytes()[i] as char);
                i += 1;
            }
        } else if tmp_buf.starts_with("type:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n'  {
                _type.push(buf.as_bytes()[i] as char);
                i += 1;
            }

        }  else if tmp_buf.starts_with("host:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n' {
                host.push(buf.as_bytes()[i] as char);
                i += 1;
            }
            
        } else if tmp_buf.starts_with("port:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n' {
                port.push(buf.as_bytes()[i] as char);
                i += 1;
            }
            return (Config {
                name,
                _type,
                host,
                port,
                master_host: String::from(""),
                master_port: String::from(""),
            }, i);
        } else {
            i += 1;
        }
    }

    return (Config::new(), i);
}

fn read_slave(buf: &str, mut i: usize) -> (Config, usize) {
    let len = buf.len();

    let mut name = String::from("");
    let mut port = String::from("");
    let mut _type = String::from("");
    let mut host = String::from("");
    let mut master_host = String::from("");
    let mut master_port = String::from("");

    while i < len {
        i = escape_null(buf, i);
        let tmp_buf = &buf[i..];
        if tmp_buf.starts_with("name:") {
            i += 5;
            i = escape_null(buf, i);
            
            while i < len && buf.as_bytes()[i] as char != '\n' {
                name.push(buf.as_bytes()[i] as char);
                i += 1;
            }
        } else if tmp_buf.starts_with("type:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n'  {
                _type.push(buf.as_bytes()[i] as char);
                i += 1;
            }

        }  else if tmp_buf.starts_with("host:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n' {
                host.push(buf.as_bytes()[i] as char);
                i += 1;
            }
            
        } else if tmp_buf.starts_with("port:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n' {
                port.push(buf.as_bytes()[i] as char);
                i += 1;
            }
        
        } else if tmp_buf.starts_with("master_host:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n' {
                master_host.push(buf.as_bytes()[i] as char);
                i += 1;
            }
        
        } else if tmp_buf.starts_with("master_port:") {
            i += 5;
            i = escape_null(buf, i);

            while i < len && buf.as_bytes()[i] as char != '\n' {
                master_port.push(buf.as_bytes()[i] as char);
                i += 1;
            }
            return (Config {
                name,
                _type,
                host,
                port,
                master_host,
                master_port,
            }, i);
        }    
        
        else {
            i += 1;
        }
    }

    return (Config::new(), i);
}

pub fn read_file(path: String) -> Vec<Config> {
    let mut file = std::fs::File::open(path).unwrap();

    let mut buf = String::from("");
    let mut ret: Vec<Config> = Vec::<Config>::new();
    let _ = file.read_to_string(&mut buf);
    // let buff: Vec<char> = buf.chars().collect();
    let buf = buf.as_str();

    // let mut i: usize = 0;
    let (ms, mut i) = read_master(buf, 0);
    i = escape_null(buf, i);
    ret.push(ms);
    // i = escape_null(buf, i);
    
    while i < buf.len() {
        let sl: Config;
        (sl, i) = read_slave(buf, i);
        ret.push(sl);
        i = escape_null(buf, i);
    } 
    ret 
}

