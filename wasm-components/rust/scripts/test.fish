#! /usr/bin/env fish

cargo insta test --review --test-runner nextest --target aarch64-apple-darwin
