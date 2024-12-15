#!/bin/bash

rustup target add x86_64-pc-windows-msvc

cargo build --target=x86_64-pc-windows-msvc --release
