# Attention

C programs are **not the same** to Rust programs. Comparing the performance between Rust and C directly is meaningless. The only meaningful measurement is the **slowdown** of porting.

# Instructions

```
cd mesalock-sgx-bench
./test_perf.sh
cd ..

cd rust-linux
./test_perf.sh
cd ..

cd fortanix-sgx-bench
./test_perf.sh
cd ..

cd c-sgx-bench
./test_perf.sh
cd ..

cd c-linux-bench
./test_perf.sh
cd ..
```

Then process the generated results.txt


# Results

## Yu's 9900k+64GB Non ECC desktop. 2.5 toolchain

|                          | ML-Rust-SGX| Fortanix-Rust-SGX | Rust-Linux  | C-SGX    | C-Linux |
| ------------------------ | ---------- | ----------------- | ----------- | -------- | ------- |
|  fann                    | 21.16858   |    24.50          |  21.67      | 19.66372 | 19.59   |
|  fasta                   | 25.25682   |    28.25          |  25.58      | 10.588   | 10.567  |
|  mandel                  | 5.77548    |    8.85           |  5.788      | 23.640   | 20.66   |
|  nbody                   | 28.43456   |    30.67          |  30.80      | 31.9817  | 31.68   |
| spectum                  | 23.23975   |    25.57          |  23.87      | 17.58745 | 17.47   |
| localattest              | 19.49004   |                   |             | 19.47614 |         |
| switchless-normal-ocall  | 9.98110    |                   |             | 9.889182 |         |
| switchless-ocall         | 1.02866    |                   |             | 1.099590 |         |
| switchless-normal-ecall  | 11.82719   |                   |             | 11.594337|         |
| switchless-ecall         | 1.39487    |                   |             | 1.587854 |         |

## Yu's Xeon E-2186G + 64GB ECC server. 2.5 toolchain

|                          | ML-Rust-SGX| Fortanix-Rust-SGX | Rust-Linux  | C-SGX    | C-Linux |
| ------------------------ | ---------- | ----------------- | ----------- | -------- | ------- |
|  fann                    | 22.42678   |    26.50          |  23.18      | 21.22987 | 21.00   |
|  fasta                   | 27.04513   |    30.27          |  27.31      | 11.31318 | 11.32   |
|  mandel                  | 6.18571    |    9.52           |  6.16       | 25.25697 | 22.47   |
|  nbody                   | 30.84538   |    32.83          |  30.38      | 34.15880 | 34.13   |
| spectum                  | 24.55704   |    27.22          |  25.04      | 18.78764 | 18.74   |
| localattest              | 19.84860   |                   |             | 19.777   |         |
| switchless-normal-ocall  | 13.36452   |                   |             | 13.62250 |         |
| switchless-ocall         | 1.16512    |                   |             | 1.19675  |         |
| switchless-normal-ecall  | 15.73456   |                   |             | 15.46765 |         |
| switchless-ecall         | 1.53158    |                   |             | 1.68042  |         |
