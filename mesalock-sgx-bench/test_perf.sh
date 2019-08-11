test_cases=(fann \
            fasta \
            localattestation \
            mandelbrot \
            nbody \
            spectum \
            thread)

# Todo: Tweak the remaining samples for perf bench

for i in ${test_cases[@]}
do
    echo "Testing ${i}" >> results.txt
    cd ${i} && make && cd bin && ./app >> ../../results.txt && cd .. && make clean && cd ..
done
