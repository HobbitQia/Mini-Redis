# 获取当前工作目录，并进入工作目录
workdir=$(cd $(dirname $0); pwd)

echo $workdir
cargo build --bin client

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