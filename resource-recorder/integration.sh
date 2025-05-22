#!/usr/bin/env bash
cargo test -p klyra-resource-recorder --all-features --test '*' -- --skip needs_docker --nocapture
