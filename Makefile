
#files
pkg/rust_vcd_ts_bg.wasm: wasm_pack target/debug/librust_vcd_ts.dylib
target/debug/librust_vcd_ts.dylib: cargo_build

#generic targets

cargo_build: src/lib.rs
	RUST_LOG=wasm_bindgen_webidl cargo build

wasm_pack: cargo_build
	wasm-pack build --target web

test: all
	time bun run ./test/load_vcd.ts --fast

all: pkg/rust_vcd_ts_bg.wasm
