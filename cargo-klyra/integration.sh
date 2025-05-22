#!/usr/bin/env bash
cargo test -p cargo-klyra --all-features --test '*' -- --skip needs_docker --nocapture
