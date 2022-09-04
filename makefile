build:
	make test
	cargo build --release
	mkdir -p ./bin
	mv ./target/release/been-there ./bin

test:
	cargo test
	rm -f ./been-there.utils.mock.json ./been-there.manager.mock.json
