# $port=6379
# echo $(netstat -nlp | grep :6379)
lsof -t -i :6378 | xargs kill
# echo $(netstat -nlp | grep :6379)

# $port=6380
# echo $(netstat -nlp | grep :6380)
lsof -t -i :6379 | xargs kill
# echo $(netstat -nlp | grep :6380)

lsof -t -i :6380 | xargs kill
lsof -t -i :6381 | xargs kill
lsof -t -i :6382 | xargs kill
