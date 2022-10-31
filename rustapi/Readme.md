# Generate rust.h C header file

cbindgen src/lib.rs -l c > rust.h

# Build for iOS device

cargo lipo --release --targets aarch64-apple-ios

