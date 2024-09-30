#!/usr/bin/bash

# redis
curl -fsSL https://packages.redis.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg
sudo chmod 644 /usr/share/keyrings/redis-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/redis.list
sudo apt-get update
sudo apt-get install --yes redis
sudo systemctl start redis

# postgres
sudo apt-get install --yes postgresql postgresql-contrib
sudo systemctl start postgresql@13-main

# wasmCloud
curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | sudo bash
sudo apt install wash
