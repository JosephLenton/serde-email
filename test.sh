#!/bin/sh

set -e

cargo check
cargo check --features=all

cargo test --features=all
