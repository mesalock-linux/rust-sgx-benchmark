test_cases=(fann \
            fasta \
            localattestation \
            mandelbrot \
            nbody \
            spectum)

# Todo: Tweak the remaining samples for perf bench

for i in ${test_cases[@]}
do
    echo "Testing ${i}" >> results.txt
    cd ${i} && make && ./app >> ../results.txt && make clean && cd ..
done
