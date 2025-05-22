#!/usr/bin/env bash
cargo test -p klyra-provisioner --all-features --test '*' -- needs_docker --nocapture
