RUSTC = rustc
RUSTDOC = rustdoc
RUST_FLAGS = -O
RUST_TEST_FLAGS = -O --test
BUILD_DIR = dist
SRC_DIR = src

build:
	mkdir -p $(BUILD_DIR)
	rustc $(RUST_FLAGS) --out-dir $(BUILD_DIR) $(SRC_DIR)/lib.rs
	rustc $(RUST_FLAGS) -L $(BUILD_DIR) -o $(BUILD_DIR)/bf $(SRC_DIR)/main.rs

test: build
	rustc $(RUST_TEST_FLAGS) -L $(BUILD_DIR) -o $(BUILD_DIR)/test_lib $(SRC_DIR)/lib.rs
	# rustc $(RUST_TEST_FLAGS) -L $(BUILD_DIR) -o $(BUILD_DIR)/test $(SRC_DIR)/main.rs
	./$(BUILD_DIR)/test_lib
	# ./$(BUILD_DIR)/test

clean:
	rm -rf $(BUILD_DIR)
