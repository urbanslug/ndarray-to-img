FOLDER_BUILD=build

ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libdouble_input.$(EXT)
	g++ examples/main.cpp -L ./target/debug/ -lndarray_to_img -o build/run
	LD_LIBRARY_PATH=./target/debug/ ./build/run

setup:
	@mkdir -p $(FOLDER_BUILD)

target/debug/libdouble_input.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf $(FOLDER_BUILD)
	rm -rf target
	rm -rf run
