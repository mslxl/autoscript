build:
	cargo build --release

gcd: build
	cargo run --release -- --print-ast ./sample/gcd.aa

flow: build
	cargo run --release -- --print-ast ./sample/controlFlow.aa