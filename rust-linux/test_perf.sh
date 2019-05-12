test_cases=(fann \
            fasta \
            mandelbrot \
            nbody \
            spectum)

# Todo: Tweak the remaining samples for perf bench

rm result.txt

for i in ${test_cases[@]}
do
    echo "Testing ${i}" >> results.txt
    cd ${i} && \time -a -o ../results.txt cargo run --release && cargo clean && cd ..
done
