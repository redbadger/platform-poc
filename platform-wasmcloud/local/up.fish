#!/usr/bin/env fish

set --local SCRIPT_DIR (dirname (realpath (status -f)))

function section
    echo
    string pad --right --char=— -w$COLUMNS "———— $argv ————"
end

section "starting redis"
brew services start redis

section "starting postgresql@15"
brew services start postgresql@15
