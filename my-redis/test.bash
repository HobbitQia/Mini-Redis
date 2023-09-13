#!/bin/bash
chmod +x ./client && chmod +x ./server

# 启动 server
./server &
echo "Server started"

# 获取 server 进程的 PID（进程 ID）
server_pid=$!

# 定义一个函数，用于重启 server
restart_server() {
    # 杀死当前 server 进程
    kill $server_pid
    # 等待一段时间确保 server 进程已经终止
    sleep 2
    # 启动新的 server
    ./server &
    echo "Server restarted"
    server_pid=$!
}

# 启动 client
./client SET key value > out
./client SET 1 2 >> out
./client SET 2 3 >> out
./client GET key >> out
./client GET 1 >> out
./client GET 2 >> out
./client DEL 1 >> out

# 当 client 执行完毕后，重启 server
restart_server

./client GET 1
./client GET 2
./client GET key

# 输出 server 进程 pid，用于手动杀死进程，防止进程一直运行占用端口
echo $server_pid