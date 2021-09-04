CRATE_ROOT=${PWD}
PLEB_RUST_FLAGS="${RUSTFLAGS} -A dead_code -A unused-variables -A unused_imports"

THIS_YEAR=`date +'%Y'`
THIS_DDMM=`date +'%d%m'`

.PHONY: clean pleb dev

pleb:
	RUSTFLAGS=${PLEB_RUST_FLAGS} cargo build

pleb-run:
	RUSTFLAGS=${PLEB_RUST_FLAGS} cargo run

dev:
	cargo build

dev-run:
	cargo run

clean:
	rm -f log/*
