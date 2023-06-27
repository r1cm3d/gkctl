all: install

build:
	cargo build --release
install: build
	-rm "$(GK_BIN)/gkctl"
	cp -v target/release/gkctl "$(GK_BIN)"
test:
	cargo test
