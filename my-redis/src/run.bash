#!/bin/bash

# 获取当前工作目录，并进入工作目录
workdir=$(cd $(dirname $0); pwd)

echo $workdir

if [ ! -d $workdir/log ]; then
    mkdir $workdir/log
fi

# 读取config文件并将启动主从服务器的命令重定向到m-s.bash中
cargo run --bin read_file > $workdir/m-s.bash

# 启动主从服务器
cd $workdir/../ && bash $workdir/m-s.bash

echo "Server start!"
