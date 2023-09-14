$port=6379
echo $(netstat -nlp | grep :6379)
lsof -t -i :$port | xargs kill
echo $(netstat -nlp | grep :6379)

$port=6380
echo $(netstat -nlp | grep :6379)
lsof -t -i :$port | xargs kill
echo $(netstat -nlp | grep :6379)