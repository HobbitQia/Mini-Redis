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