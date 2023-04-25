#!/bin/bash

all:
	SLINT_STYLE=native cargo build --release

build:
	SLINT_STYLE=native cargo build --release

build-debug:
	SLINT_STYLE=native cargo build

run:
	SLINT_STYLE=native RUST_LOG=error,warn,info,debug,reqwest=on proxychains cargo run

run-noproxy:
	SLINT_STYLE=native RUST_LOG=error,warn,info,debug,reqwest=on cargo run

run-local:
	RUST_LOG=error,warn,info,debug,reqwest=on ./target/debug/chatbox

clippy:
	cargo clippy

clean:
	cargo clean

slint-view:
	slint-viewer --style native --auto-reload -I chatbox/ui ./chatbox/ui/appwindow.slint
