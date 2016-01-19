test:
	cargo test

build:
	cargo build

docs:
	cp src/lib.rs code.bak
	cat README.md | sed -e 's/^/\/\/! /g' > readme.bak
	sed -i '/\/\/ DOCS/r readme.bak' src/lib.rs
	(cargo doc --no-deps && make clean) || (make clean && false)

clean:
	mv code.bak src/lib.rs || true
	rm *.bak || true

upload:
	echo '<!doctype html><title>rust-webicon</title><meta http-equiv="refresh" content="0; ./webicon/">' \
		> ./target/doc/index.htm
	rsync -av --chmod=755 ./target/doc/ untispace:~/virtual/rust-webicon.unterwaditzer.net/
