
build:
	RUSTFLAGS='-C target-cpu=native' cargo build

run: 
	RUSTFLAGS='-C target-cpu=native' cargo run

release:
	RUSTFLAGS='-C target-cpu=native -C opt-level=3' cargo build --release

run-release:
	RUSTFLAGS='-C target-cpu=native -C opt-level=3' cargo run --release

doc:
	cargo doc --no-deps --open
