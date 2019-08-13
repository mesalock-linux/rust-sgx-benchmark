cd destroy_enclave && \
make && \
for i in {0..1000}
do
    ./app 1>/dev/null 2>>time.txt
done
cd ../..
