#!/usr/bin/env fish

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "stopping redis"
brew services stop redis

section "stopping postgresql@15"
brew services stop postgresql@15
