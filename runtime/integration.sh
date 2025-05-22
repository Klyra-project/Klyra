#!/usr/bin/env bash
cargo test -p klyra-runtime --all-features --test '*' -- --skip needs_docker --nocapture
