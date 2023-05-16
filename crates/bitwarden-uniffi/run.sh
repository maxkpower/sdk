#!/bin/env bash

cargo run --bin uniffi-bindgen generate src/sdk.udl --language swift
cargo build --package bitwarden-uniffi --target aarch64-apple-ios-sim --release

mkdir Headers
cp ./src/bitwardenFFI.h ./Headers/
cp ./src/bitwardenFFI.modulemap ./Headers/module.modulemap


xcodebuild -create-xcframework -library ../../target/aarch64-apple-ios-sim/release/libbitwarden_uniffi.a -headers ./Headers -output BitwardenSdk.xcframework

cp src/bitwarden.swift platforms/apple/Hello/Sources/Hello
