all: target/debug/libcpp_ffi.a
	g++ src/main.cpp --std=c++17 -fsanitize=thread -L target/debug -lpthread -ldl -lcpp_ffi -o main

target/debug/libcpp_ffi.a: src/lib.rs Cargo.toml
#	RUSTFLAGS="-Z sanitizer=thread" cargo +nightly build
#	cargo build
	cargo +1.39.0 build

clean:
	rm -rf target
	rm -rf main