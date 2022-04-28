all: install

build:
	cargo build --release
install: build
	-rm "$(GK_BIN)/gk-yaml"
	cp -v target/release/gk-yaml "$(GK_BIN)"
test:
	cargo test
