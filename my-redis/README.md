# HW5 - myredis

## 简介

本次作业使用 Rust [Volo 框架](https://github.com/cloudwego/volo)实现一个 Mini-Redis，支持 `GET` `SET` `DEL` `PING` 这四条基本命令，并写一个中间件 filter，可以过滤部分请求。并拓展了 `PUBLISH` `SUBSCRIBE` 命令。此外作业中还有一个中间件，用来过滤请求中带 `shabi` 的指令，对于这类指令会直接返回错误。通过演示 client 请求 server 返回对应结果来展示其功能。

redis 指令的使用规范可见[官方文档](https://redis.io/commands/)

## 如何在本地构建使用

可以通过下面的方式自行构建。

``` shell
$ git clone https://github.com/HobbitQia/Rust-2023-Homework.git
$ cd Rust-2023-Homework/hw5-myredis
$ cargo build                       # 构建工程
$ ./target/build/server             # 运行客户端
$ ./target/build/client [cmd]       # 运行服务端
```
`cmd` 具体格式与 redis 类似：

* `PING`
    ``` bash
    PING [message]
    ```
    若不带参数，则返回 `PONG`；若带参数，则返回参数。
    ``` bash
    $ ./server PING
    OK!
    "PONG"
    $ ./server PING Hello
    OK!
    "Hello"
    ```
* `SET`
    ``` bash
    SET key value
    ```
    将 `key` 的值设为 `value`。若 `key` 已存在，则覆盖原值。
    ``` bash
    $ ./server SET foo 1
    OK!
    $ ./server GET foo
    Value: "1"
    $ ./server SET foo 2
    OK!
    $ ./server GET foo
    Value: "2"
    ```
* `GET`
    ``` bash
    GET key
    ```
    返回 `key` 的值。若 `key` 不存在，则返回 `Key not found!`。
    ``` bash
    $ ./server SET foo 2
    OK!
    $ ./server GET foo
    Value: "2"
    $ ./server GET key_not_exist
    Some error happens: "Key not found!"
    ```
* `DEL`
    ``` bash
    DEL key
    ```
    删除 `key`。若 `key` 不存在，则返回 `Some error happens: "Key not found!"`，否则返回 `OK!`。
    ``` bash
    $ ./server SET foo 1
    OK!
    $ ./server GET foo
    Value: "1"
    $ ./server DEL foo
    Value: "1"
    $ ./server GET foo
    Some error happens: "Key not found!"
    $ ./server DEL foo
    OK!
    ```
* `SUBSCRIBE`
    ``` bash
    SUBSCRIBE channel
    ```
    > 由于框架 TCP 短连接的特性，客户端每次订阅只能获得一条消息。
    订阅 `channel`，等待下一条发送到 `channel` 的消息。
    ``` bash
    $ ./server SUBSCRIBE hobbitqia
    Waiting for messages to be issued...            # 等待消息发布
    OK!                                             # 消息发布后输出
    "God"
    ```
* `PUBLISH`
    ``` bash
    PUBLISH channel message
    ```
    向 `channel` 发送 `message`，并返回客户端数量。

    ``` bash
    $ ./server PUBLISH hobbitqia "God"
    OK!
    "1"                                             # 返回当前有多少个客户端订阅了
    ```
* 中间件  
    ``` bash
    $ ./client pubish hobbitqia "shabi"
    2023-09-12T13:14:38.864099Z ERROR client: "application error: service error, msg: No dirty word, please!"
    ```

## 文件结构

``` shell
hw5-myredis/
├── idl
│   └── volo_example.thrift     # Thrift IDL 文件
├── src
│    ├── lib.rs                 # 服务器端实现
│    └── bin
│         ├── client.rs         # 客户端程序
│         └── server.rs         # 服务器程序
├── volo-gen
│    ├── build.rs
│    ├── Cargo.toml
│    ├── src
│    │    └── lib.rs
│    └── volo.yml
├── Cargo.lock
├── Cargo.toml
└── README
```

## 示例

* 服务器：  
![](https://cdn.hobbitqia.cc/20230912171828.png)

* 基本的 `GET` `SET` 功能：  
![](https://cdn.hobbitqia.cc/20230912171924.png)

* `DEL`：  
![](https://cdn.hobbitqia.cc/20230912172009.png)

* `PING`：  
![](https://cdn.hobbitqia.cc/20230912180849.png)

* `SUBSCRIBE` & `PUBLISH`：  
这里我们先 subscribe 后，输出等待信息，同时我们在另一个终端运行客户端程序，执行 publish 指令，可以看到 subscribe 的终端输出了订阅的信息，而 publish 的终端输出了订阅的客户端数量。  
![](https://cdn.hobbitqia.cc/20230912181004.png)
![](https://cdn.hobbitqia.cc/20230912181027.png)

* 中间件  
这里如果消息里出现了 `shabi`，则会返回错误，并打印日志。
![](https://cdn.hobbitqia.cc/20230912211524.png)