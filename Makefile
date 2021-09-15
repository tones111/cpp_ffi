all: target/debug/libcpp_ffi.a
	g++ src/main.cpp -L target/debug -lpthread -ldl -lcpp_ffi -o main

target/debug/libcpp_ffi.a: src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf main