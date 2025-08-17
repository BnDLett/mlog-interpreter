# Generate PGO data — make sure to run the program after compilation to fully generate pgo-data.
RUSTFLAGS="-Cprofile-generate=pgo-data" cargo build --release

# For building with PGO
RUSTFLAGS="-Cprofile-use=pgo-data -Cllvm-args=-pgo-warn-missing-function" cargo build --release

# For performance analysis
RUSTFLAGS="-Cprofile-use=pgo-data -Cllvm-args=-pgo-warn-missing-function" cargo flamegraph -o profile.svg -F 20000
