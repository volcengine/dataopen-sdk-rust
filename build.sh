#!/bin/sh

rm -rf release/dataopen-sdk-rust*
mkdir release/dataopen-sdk-rust

cp -rf LICENSE release/dataopen-sdk-rust/
cp -rf README.md release/dataopen-sdk-rust/
cp -rf ./src/client.rs release/dataopen-sdk-rust/
cp -rf ./src/lib.rs release/dataopen-sdk-rust/
cp -rf Cargo.lock release/dataopen-sdk-rust/
cp -rf Cargo.toml release/dataopen-sdk-rust/

cd release
zip -r dataopen-sdk-rust.zip dataopen-sdk-rust/*

rm -rf dataopen-sdk-rust

cd ../