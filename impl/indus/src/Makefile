CRATE_ROOT=${PWD}
PLEB_RUST_FLAGS="${RUSTFLAGS} -A dead_code -A unused-variables -A unused_imports"

THIS_YEAR=`date +'%Y'`
THIS_DDMM=`date +'%d%m'`

.PHONY: clean pleb dev test

pleb:
	RUSTFLAGS=${PLEB_RUST_FLAGS} cargo build

pleb-run:
	RUSTFLAGS=${PLEB_RUST_FLAGS} RUST_BACKTRACE=1 RUST_LOG=info cargo run -- --show-output

pleb-test:
	RUSTFLAGS=${PLEB_RUST_FLAGS} RUST_BACKTRACE=1 RUST_LOG=info cargo test -- --show-output

test:
	RUSTFLAGS=${PLEB_RUST_FLAGS} RUST_BACKTRACE=1 RUST_LOG=info cargo test

dev:
	cargo build

dev-run:
	cargo run

clean:
	cargo clean --release --quiet --offline
	rm -rf log/*
