ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libdouble_input.$(EXT)
	g++ src/main.cpp -L ./target/debug/ -lndarray_to_img -o run
	LD_LIBRARY_PATH=./target/debug/ ./run

target/debug/libdouble_input.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf run
