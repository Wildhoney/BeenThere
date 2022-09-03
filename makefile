build:
	cargo build --release
	mkdir -p ./bin
	mv ./target/release/been-there ./bin

test:
	cargo test
	rm -f ./been-there.mock.json
