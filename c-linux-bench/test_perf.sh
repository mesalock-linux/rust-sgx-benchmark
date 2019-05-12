test_cases=(fannkuchredux.gcc-5 \
            fasta.gcc-5 \
            mandelbrot.gcc-2 \
            nbody \
            spectralnorm.gcc-3)

# Todo: Tweak the remaining samples for perf bench

CUR=${PWD}

for i in ${test_cases[@]}
do
    echo "Testing ${i}" >> results.txt
    gcc -O2 -o ${i} ${i}.c -lm && \time -a -o ${CUR}/results.txt ./${i} && rm ${i}
done
