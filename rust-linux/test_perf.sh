test_cases=(fann \
            fasta \
            mandelbrot \
            nbody \
            spectum)

# Todo: Tweak the remaining samples for perf bench

rm result.txt

CUR=${PWD}

for i in ${test_cases[@]}
do
    echo "Testing ${i}" >> results.txt
    cd ${i} && cargo b --release && \time -a -o ${CUR}/results.txt ./target/release/${i} --release && cargo clean && cd ..
done
