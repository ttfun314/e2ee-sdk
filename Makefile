# Define the target library name
LIB_NAME = libe2ee_sdk.dylib

# Define the Rust project directory
RUST_PROJECT_DIR = .

# Define the output directory for the Java library
JAVA_LIBS_DIR = java/libs

# Run tests for the Rust project
test:
	cargo test --manifest-path $(RUST_PROJECT_DIR)/Cargo.toml

# Build the Rust project
build:
	cargo build --release --manifest-path $(RUST_PROJECT_DIR)/Cargo.toml

# Clean the Rust project
clean:
	cargo clean --manifest-path $(RUST_PROJECT_DIR)/Cargo.toml

## Java targets
# Copy the compiled library to the Java libs directory
copy-java:
	cp $(RUST_PROJECT_DIR)/target/release/$(LIB_NAME) $(JAVA_LIBS_DIR)/

# Run the Java program
run-java:
	cd java && javac Encryptor.java && java -Djava.library.path=libs Encryptor

## Android targets
build-android:
	cargo ndk -t x86_64 build --release 

copy-android:
	cp $(RUST_PROJECT_DIR)/target/x86_64-linux-android/release/libe2ee_sdk.so android/mylibrary/src/main/jniLibs/x86_64/

build-lib-android:
	cd android && ./gradlew :mylibrary:assembleRelease

# iOS targets
build-ios:
	cargo build --target x86_64-apple-ios --release

# Default target
test-java: test build copy-java run-java
android: test build-android copy-android build-lib-android
ios: test build-ios
