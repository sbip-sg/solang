# SOLANG-SBIP

This README is for the SBIP-customized version of Solang.

## Compilation

To compile Solang from source code, you need to use its own customization of
LLVM, whose pre-built binaries can be downloaded from its [release page](https://github.com/hyperledger/solang/releases).

Please always download the latest LLVM pre-built package, and copy it to
`$HOME/llvm/llvm-solang` (see the [Makefile](Makefile) for more details).

Then, you can either run `cargo build` or `make` to compile Solang:

- Using `cargo`:

  ```sh
  cd solang

  PATH='${HOME}/llvm/llvm-solang/bin:${PATH}' \
  LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
  LD_LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
  DYLD_LIBRARY_PATH='${HOME}/llvm/llvm-solang/lib' \
  RUSTFLAGS='-L ${HOME}/llvm/llvm-solang/lib' \
  cargo build --release
  ```

- Using `make`:


  ```sh
  cd solang
  make release
  ```

The final output file will be compiled to `target/release/solang`.
