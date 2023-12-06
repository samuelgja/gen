# Run cargo build release mode
cargo build --release


# Create releases/latest/download directory
mkdir -p releases/latest/download
# Move binary to releases/latest/download/gen
cp target/release/gen releases/latest/download/gen