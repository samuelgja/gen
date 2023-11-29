test with print 
cargo test --release -- --nocapture should_test_block_header

rustup run +nightly cargo test --release  -- --nocapture bench_dot_product


    let iter = unsafe {
                    // SAFETY: We know this is safe because 'iter' will
                    // never outlive 'owned_data' since they are part of the same struct.
                    std::mem::transmute::<Iter<u8>, Iter<'static, u8>>(data.iter())
                };