#!/usr/bin/env bash
cargo test -p klyra-auth --all-features --test '*' -- needs_docker --nocapture
