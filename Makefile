all: target/debug/libcpp_ffi.a
	g++ src/main.cpp --std=c++17 -L target/release -lpthread -ldl -lcpp_ffi -o main

target/debug/libcpp_ffi.a: src/lib.rs Cargo.toml
	cargo build --release

clean:
	rm -rf target
	rm -rf main