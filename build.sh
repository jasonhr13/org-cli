#! /bin/bash

cargo build --release
cargo build --target x86_64-apple-darwin --release

aws s3 cp target/release/fuzzy-cli s3://fuzzy-cli/arm64/fuzzy-cli
aws s3 cp target/x86_64-apple-darwin/release/fuzzy-cli s3://fuzzy-cli/x86_64/fuzzy-cli
