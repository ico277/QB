PREFIX = /usr
EXEC = qb

.PHONY: build install uninstall

build:
	cargo build --release

install:
	install -m 755 ./target/release/qb $(PREFIX)/bin/$(EXEC)

uninstall:
	rm $(PREFIX)/bin/$(EXEC)

clean:
	cargo clean
