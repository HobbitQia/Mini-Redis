#!/bin/bash

workdir=$(cd $(dirname $0); pwd)

echo $workdir

if [ ! -d $workdir/log ]; then
    mkdir $workdir/log
fi

echo "start redis server" > $workdir/log/redis.log

cd $workdir/../target/debug && ./read_file
./client 