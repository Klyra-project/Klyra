#!/usr/bin/env bash
cargo test -p klyra-service --all-features --test '*' -- --skip needs_docker --nocapture
