#! /usr/bin/env fish

cargo clean
fd --type dir deps -x rm -rf
