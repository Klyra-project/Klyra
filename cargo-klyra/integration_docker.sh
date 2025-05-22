#!/usr/bin/env bash
cargo test -p cargo-klyra --all-features --test '*' -- needs_docker --nocapture
