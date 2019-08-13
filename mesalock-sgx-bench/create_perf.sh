rm -f create_enclave/bin/time.txt
cd create_enclave && \
make && \
cd bin && \
for i in {0..1000}
do
    ./app 1>/dev/null 2>>time.txt
done
cd ../..
