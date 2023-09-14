# 获取当前工作目录，并进入工作目录
workdir=$(cd $(dirname $0); pwd)

echo $workdir

cd $workdir/../target/debug && ./client redis-master-1 set THU TsinghuaUniversity
cd $workdir/../target/debug && ./client redis-slave-1 get THU
cd $workdir/../target/debug && ./client redis-slave-1 set ZJU ZhengjiangUniversity
cd $workdir/../target/debug && ./client redis-master-1 get ZJU
cd $workdir/../target/debug && ./client redis-master-1 set ZJU ZhejiangUniversity
cd $workdir/../target/debug && ./client redis-slave-1 get ZJU
cd $workdir/../target/debug && ./client redis-slave-1 del ZJU
cd $workdir/../target/debug && ./client redis-master-1 del ZJU
cd $workdir/../target/debug && ./client redis-master-1 get ZJU