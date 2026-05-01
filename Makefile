.PHONY: build install sync push

JOBS != getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4

install: build
	mkdir -p ${HOME}/bin
	-@mv ${HOME}/bin/codex ${HOME}/bin/codex.bak 2>/dev/null || true
	cp codex-rs/target/release/codex ${HOME}/bin/codex

build:
	cd codex-rs && cargo build --release -j ${JOBS} -p codex-cli --bin codex

push:
	git push -f akfaew HEAD
