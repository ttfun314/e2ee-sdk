# Define the target library name
LIB_NAME = libe2ee_sdk.dylib

# Define the Rust project directory
RUST_PROJECT_DIR = .

# Define the output directory for the Java library
JAVA_LIBS_DIR = java/libs

# Default target
all: test build copy run-java


# Run tests for the Rust project
test:
	cargo test --manifest-path $(RUST_PROJECT_DIR)/Cargo.toml

# Build the Rust project
build:
	cargo build --release --manifest-path $(RUST_PROJECT_DIR)/Cargo.toml

# Copy the compiled library to the Java libs directory
copy:
	cp $(RUST_PROJECT_DIR)/target/release/$(LIB_NAME) $(JAVA_LIBS_DIR)/

run-java:
	cd java && javac Encryptor.java && java -Djava.library.path=libs Encryptor
# Clean the Rust project
clean:
	cargo clean --manifest-path $(RUST_PROJECT_DIR)/Cargo.toml

.PHONY: all build copy clean
