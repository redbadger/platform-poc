#!/usr/bin/env fish

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "starting redis"
brew services start redis

section "starting postgresql@15"
brew services start postgresql@15

section "starting nats-server"
brew services start nats-server
