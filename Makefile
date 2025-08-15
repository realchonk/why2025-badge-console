TARGET = riscv32imafc-unknown-none-elf
CARGO = RUSTC_BOOTSTRAP=1 cargo
SRCS != find src

all: console-dbg.elf

clean:
	rm -rf target
	rm -f console.elf console-dbg.elf *.pkg.tgz *.core

bump:
	./scripts/bump.sh

package: console.elf
	./scripts/package.sh

push: package
	ssh hartbuch 'tar -xvzf - -C src/why2025/why2025-badge-firmware/' < console.pkg.tgz

publish: console.elf
	./scripts/publish.sh

doc:
	${CARGO} doc

console-dbg.elf: Cargo.toml .cargo/config.toml ${SRCS} build.rs
	${CARGO} build
	cp -f target/${TARGET}/debug/console $@

console.elf: Cargo.toml .cargo/config.toml ${SRCS} build.rs
	${CARGO} build --release
	cp -f target/${TARGET}/release/console $@
