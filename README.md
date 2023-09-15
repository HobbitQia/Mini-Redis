# Mini-Redis
浙江大学 2023 短学期《Rust 开发实训》课程大作业仓库

我们基于之前[手写的 Mini-Redis](https://github.com/HobbitQia/Rust-2023-Homework/tree/master/hw5-myredis)，拓展了更高阶的能力：

* AOF（Append-only File）实现持久化
* Redis 主从架构
* Redis Cluster
* Graceful exit 

## 如何在本地构建使用

``` shell
$ git clone https://github.com/HobbitQia/Mini-Redis.git
$ cd Mini-Redis/my-redis
$ bash ./test.bash                       # 测试 Part 1 AOF 
$ bash ./src/run.bash                    # 脚本批量启动 Server 
$ bash ./src/client_part2_test.bash      # 测试 Part 2 主从
$ bash ./src/client_part3_test.bash      # 测试 Part 3 cluster
```

> 需要注意的是，Part2 和 Part3 共享一份服务器启动脚本 `./src/run.bash` 。但是在启动服务器前需要配置好配置文件 `config.txt`，具体格式见下文示例。（我们在 `./src` 目录下提供了两个模式的参考文件，如 `cfg_ms.txt`，需要用时直接复制进 `config.txt` 即可）
> 
> 注意，`./src/client_part2_test.bash` 和 `./src/client_part3_test.bash` 并不能一次性直接测完，测试每部分时需要先修改配置文件 `config.txt` 启动对应部分服务器。
> 
> 同时需要注意为了解决多个服务器占用同一端口问题，每次测试完毕后请使用 `bash ./src/tools/kill-ports` 杀死之前的服务端进程，再进行后续测试。

此外，多次使用程序可能导致同一个端口被多个 server 占用。这里我们提供了一个批量杀死指定端口的脚本文件，可见 `tools/kill-ports.bash`。

### 配置文件格式

``` bash
pattern: [master-slave/cluster]

name: [name]
type: [master/slave/proxy]
host: [host]
port: [port]
master_host(opt): [master_host]
master_port(opt): [master_port]
```
* `pattern` 决定构建服务器时是以那种架构构建的，本项目有两个选项，  
  * `master-slave`: 以主从架构构建
  * `cluster`: 以集群架构构建 
* `type` 有三种类型：  
  * `master`: 表示主服务端
  * `slave`: 表示从服务端
  * `proxy`: 表示代理服务端
* 当且仅当 `type` = "slave" 的时候，`master_host` 和 `master_port` 的值才有效

## 文件结构

``` shell
Mini-Redis/my-redis
├── idl
│   └── volo_example.thrift     # Thrift IDL 文件
├── src
│    ├── lib.rs                 # 服务器端实现
│    └── bin
│         ├── client.rs         # 客户端程序
│         ├── read_file.rs      # 读配置文件程序
│         └── server.rs         # 服务器程序
├── tools
│    └── kill-ports.bash        # 批量杀死指定端口的脚本文件
├── lib.rs                      # 实现 server 端逻辑
├── log                         # 用来存放 AOF 可持久化的文件
├── aof.rs                      # Part 1: 实现 AOF 功能
├── config.txt                  # 会被载入程序的配置文件
├── cfg_cluster.txt             # cluster 模式的配置文件
├── cfg_ms.txt                  # 主从模式的配置文件
├── client_part2_test.bash      # Part 2: 主从对应的测试文件
├── client_part3_test.bash      # Part 3: 集群对应的测试文件
├── run.bash                    # 批量启动服务器的脚本文件
├── read_file.rs                # 读配置文件程序
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

## 设计思路

### Part 1: AOF

* 考虑到只有 `SET` 和 `DEL` 需要被记录，所以设计一个 `enum`:  

   ```rust
   pub enum Command {
       Set { key: String, value: String },
       Del { key: String },
   }
   ```

* 每次收到一次指令就创建一个 Command 并格式化为字符串 `SET {} {}` 或 `DEL {}` 后追加到 `log.aof` 文件中。`log.aof` 的文件示例如下：

   ```
   SET ZJU ZhengjiangUniversity
   SET ZJU ZhejiangUniversity
   DEL ZJU
   DEL ZJU
   ```
   
* 每次启动 redis 时读取 `log.aof` 文件读取所有指令，重复执行这些指令恢复redis中的数据。

### Part 2: 主从模式

* 一个主服务端，一个从属服务端
  * 主服务端负责处理 `SET`, `DEL`, `GET` 服务请求
  * 从服务端负责处理 `GET` 服务请求，对于 `SET`, `DEL` 请求返回错误
* 服务器运行时，采用增量复制的方法同步数据，每当主服务端接收到写、删除操作请求时，在处理完请求后，将同样的请求信息发送给从服务端，从服务端执行对应的操作，从而达到数据同步的效果。
* 服务器重启时，采用重演历史的方法恢复数据

优点：

* 实现简单
* 数据稳定

缺点：

* 运行效率低

### Part 3: cluster 模式

* 本质上是实现多个主从架构一起对外服务，通过哈希算法实现负载均衡
  * 哈希算法：对 key 计算出一个 hash 值，然后用哈希值对 master 数量进行取模。由此就可以将 key 负载均衡到每一个 Redis 节点上去 
* 最外层 proxy 服务端接收到 key 后，会根据上述的哈希算法找到对应的主从架构服务器，对其发送服务请求，从而执行对应操作。
* 每一个主从架构都有一个独立的备份日志

## 测试结果

### Part 1: AOF 

脚本如下：

```bash
# 启动 client
echo "fisrt set data" > out
./client SET key value >> out # {"key": "value"}
./client SET 1 2 >> out # {"1": "2"}
./client SET 2 3 >> out # {"2": "3"}
./client GET key >> out
./client GET 1 >> out
./client GET 2 >> out
./client DEL 1 >> out #delete {"1": "2"}

# 当 client 执行完毕后，重启 server
restart_server

echo "Get restored data"
./client GET 1
./client GET 2
./client GET key

# 输出 server 进程 pid，用于手动杀死进程，防止进程一直运行占用端口
echo $server_pid
```

</div>
<div class="col">

<div align=center><img src="https://cdn.jsdelivr.net/gh/MYJOKERML/imgbed//taishi/image-20230914223204608.png"></div>

### Part 2: 主从模式

测试脚本：

测试思路：先往 master 写入数据 `{"THU": "TsinghuaUniversity"}` ，再测试三个 slave 是否能正确读取数据。
然后测试往 slave1 里写入数据和删除数据，均被拒绝，只能通过 master 进行写操作。

``` bash
# 获取当前工作目录，并进入工作目录
workdir=$(cd $(dirname $0); pwd)
echo $workdir
echo "redis-master-1 set THU TsinghuaUniversity: " && cd $workdir/../target/debug && ./client redis-master-1 set THU TsinghuaUniversity # 往 master 里面写入数据 {"THU": "TsinghuaUniversity"}
echo "redis-slave-1 get THU: " && cd $workdir/../target/debug && ./client redis-slave-1 get THU # 从 slave1 里面读取数据 {"THU": "TsinghuaUniversity"}
echo "redis-slave-2 get THU: " && cd $workdir/../target/debug && ./client redis-slave-2 get THU # 从 slave2 里面读取数据 {"THU": "TsinghuaUniversity"}
echo "redis-slave-3 get THU: " && cd $workdir/../target/debug && ./client redis-slave-3 get THU # 从 slave3 里面读取数据 {"THU": "TsinghuaUniversity"}
echo "redis-slave-1 set ZJU ZhengjiangUniversity" && cd $workdir/../target/debug && ./client redis-slave-1 set ZJU ZhengjiangUniversity # 往 slave1 里面写入数据，返回错误信息
echo "redis-slave-1 get ZJU" && cd $workdir/../target/debug && ./client redis-slave-1 get ZJU # 从 master 里面读取数据，返回错误信息
echo "redis-master-1 get ZJU" && cd $workdir/../target/debug && ./client redis-master-1 get ZJU # 从 master 里面读取数据，未成功写入，返回错误信息
echo "redis-master-1 set ZJU ZhejiangUniversity" && cd $workdir/../target/debug && ./client redis-master-1 set ZJU ZhejiangUniversity # 往 master 里面写入数据 {"ZJU": "ZhejiangUniversity"}
echo "redis-slave-1 get ZJU" && cd $workdir/../target/debug && ./client redis-slave-1 get ZJU # 从 slave1 里面读取数据 {"ZJU": "ZhejiangUniversity"}
echo "redis-slave-1 del ZJU" && cd $workdir/../target/debug && ./client redis-slave-1 del ZJU # 从 slave1 里面删除数据,被拒绝
echo "redis-master-1 del ZJU" && cd $workdir/../target/debug && ./client redis-master-1 del ZJU # 从 master 里面删除数据 {"ZJU": "ZhejiangUniversity"}
echo "redis-master-1 get ZJU" && cd $workdir/../target/debug && ./client redis-master-1 get ZJU # 从 master 里面读取数据，成功删除，返回错误信息
```

<div align = center><img src="https://cdn.jsdelivr.net/gh/MYJOKERML/imgbed//taishi/image-20230914232717570.png"></div>

### Part 3: cluster 模式

测试脚本：

```bash
# 获取当前工作目录，并进入工作目录
workdir=$(cd $(dirname $0); pwd)

echo $workdir

cd $workdir/../target/debug && ./client set THU TsinghuaUniversity
cd $workdir/../target/debug && ./client get THU
cd $workdir/../target/debug && ./client set ZJU ZhengjiangUniversity
cd $workdir/../target/debug && ./client get ZJU
cd $workdir/../target/debug && ./client set ZJU ZhejiangUniversity
cd $workdir/../target/debug && ./client get ZJU
cd $workdir/../target/debug && ./client del ZJU
cd $workdir/../target/debug && ./client del ZJU
cd $workdir/../target/debug && ./client get ZJU
cd $workdir/../target/debug && ./client set key value
cd $workdir/../target/debug && ./client set rust cargo
```

<div align=center><img src="https://cdn.jsdelivr.net/gh/MYJOKERML/imgbed//taishi/image-20230914235335111.png"></div>
<div align=center><img src="https://cdn.jsdelivr.net/gh/MYJOKERML/imgbed//taishi/image-20230914235423833.png" style="zoom: 80%;"></div>
<div align=center><img src="https://cdn.jsdelivr.net/gh/MYJOKERML/imgbed//taishi/image-20230914235446953.png" style="zoom: 80%;"></div>

### Bonus: graceful exit

主从架构下测试结果：

<div align=center><img src="https://s2.loli.net/2023/09/15/7lAqLoUscWehnj4.png"></div>

Cluster 下测试结果：

<div align=center><img src="https://s2.loli.net/2023/09/15/xN56mJX8wRjnSOb.png"></div>

## 总结

总体上我们实现了所有要求，成功实现了基于AOF机制的持久化策略，保证数据的稳定行；成功实现了主从架构，每次写操作同步主从服务器的数据，进一步保证数据稳定；同时实现了基于主从架构的mini-redis cluster，实现了一个Redis Proxy，通过hash值来分配每一个请求对应的服务器，以提高服务器的性能。实现了graceful exit。

有待优化的地方：

* 注释较少
* 缺少大规模数据的测试
* 代码可移植性低
* 若多次通过脚本启动服务端会导致端口复用，多个服务端共用同一个端口，数据却并无法保持一致。
* 缺少事务模块
