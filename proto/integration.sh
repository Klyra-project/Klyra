#!/usr/bin/env bash
cargo test -p klyra-proto --all-features --test '*' -- --skip needs_docker --nocapture
