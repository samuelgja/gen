

cargo fmt; # format the code
cargo clippy --fix --allow-dirty;   # check the code
cargo check; # check the code
cargo test --release; # run the tests
# Build is not needed as it is inside custom build CI/CD pipeline
# cargo build --release; # check build --release
