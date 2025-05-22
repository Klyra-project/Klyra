#!/usr/bin/env bash
cargo test -p klyra-deployer --all-features --test '*' -- --skip needs_docker --nocapture
