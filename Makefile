.PHONY: all release debug
.DEFAULT_GOAL := all

all: debug
# all: release

release:
	PATH='${HOME}/llvm/llvm-solang/bin:${PATH}' \
	LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
	LD_LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
	DYLD_LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
	RUSTFLAGS='-L ${HOME}/llvm/llvm-solang/lib' \
	cargo build --release

debug:
	PATH='${HOME}/llvm/llvm-solang/bin:${PATH}' \
	LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
	LD_LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
	DYLD_LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
	RUSTFLAGS='-L ${HOME}/llvm/llvm-solang/lib' \
	cargo build
