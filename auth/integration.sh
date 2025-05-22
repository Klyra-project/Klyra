#!/usr/bin/env bash
cargo test -p klyra-auth --all-features --test '*' -- --skip needs_docker --nocapture
