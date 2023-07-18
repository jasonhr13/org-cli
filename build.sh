#! /bin/bash

cargo build --release
cargo build --target x86_64-apple-darwin --release

aws s3 cp target/release/org-cli S3_LOCATION
aws s3 cp target/x86_64-apple-darwin/release/org-cli S3_LOCATION
