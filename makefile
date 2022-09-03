build:
	cargo build --release
	mkdir -p ./bin
	mv ./target/release/been-there ./bin
