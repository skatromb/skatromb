ssh root@174.138.104.51
cd /Users/skatromb/PycharmProjects/Python/src/docker/python_ml
docker build -t python:ML .
sudo service docker stop
sudo dockerd -H tcp://0.0.0.0:2375
