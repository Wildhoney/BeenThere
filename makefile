build:
	make format
	make test
	cargo build --release
	mkdir -p ./bin
	mv ./target/release/been-there ./bin

test:
	cargo test -- --test-threads 1
	rm -f ./been-there.utils.mock.json ./been-there.manager.mock.json

format:
	cargo fmt

b: build
t: test
f: format
