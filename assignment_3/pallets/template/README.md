License: Unlicense

cargo build --release --features runtime-benchmarks
cargo build --features runtime-benchmarks --help
./target/release/node-template benchmark --help

./target/release/node-template benchmark \
    --chain dev \               # Configurable Chain Spec
    --execution wasm \          # Always test with Wasm
    --wasm-execution compiled \ # Always used `wasm-time`
    --pallet pallet_example \   # Select the pallet
    --extrinsic '*' \          # Select the benchmark case name, using '*' for all
    --steps 20 \                # Number of steps across component ranges
    --repeat 10 \               # Number of times we repeat a benchmark
    --raw \                     # Optionally output raw benchmark data to stdout
    --output ./                 # Output results into a Rust file


    ./target/release/node-template benchmark --help




./target/release/node-template benchmark \
    --chain dev \
    --execution wasm \
    --wasm-execution compiled \
    --pallet pallet_template \
    --extrinsic '*' \
    --steps 20 \
    --repeat 10 \
    --raw \
    --output ./pallets/template/src/weights.rs
