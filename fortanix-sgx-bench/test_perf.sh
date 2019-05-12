test_cases=(fann \
            fasta \
            mandelbrot \
            nbody \
            spectum)

# Todo: Tweak the remaining samples for perf bench

rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly-2019-04-26
cargo install fortanix-sgx-tools sgxs-tools

rm result.txt

CUR=${PWD}

for i in ${test_cases[@]}
do
    echo "Testing ${i}" >> results.txt
    cd ${i} && \time -a -o ${CUR}/results.txt cargo run --target x86_64-fortanix-unknown-sgx --release && cargo clean && rm Cargo.lock && cd ..
done
