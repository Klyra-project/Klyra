#!/usr/bin/env bash
cargo test -p klyra-logger --all-features --test '*' -- needs_docker --nocapture
