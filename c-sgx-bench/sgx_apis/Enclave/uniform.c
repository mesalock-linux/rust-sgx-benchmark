#define PREFERRED_NUMBER_OF_BLOCKS_TO_USE 12
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include "sgx_utils.h"
#include "Enclave.h"
#include "Enclave_t.h"
#include "sgx_tprotected_fs.h"
//#include "user_types.h"

#define TEST_COUNT 1000000
//#define TEST_COUNT 1

#define FLAGS_NON_SECURITY_BITS     (0xFFFFFFFFFFFFC0ULL | SGX_FLAGS_MODE64BIT | SGX_FLAGS_PROVISION_KEY| SGX_FLAGS_EINITTOKEN_KEY)
#define TSEAL_DEFAULT_FLAGSMASK     (~FLAGS_NON_SECURITY_BITS)

//    struct my_timespec mt;
//    ocall_get_timestamp(&mt);
//    printf("%u %u\n", mt.tv_sec, mt.tv_nsec);
void bench_sgx_get_key() {
    const sgx_report_t *report = sgx_self_report();

    sgx_attributes_t attribute_mas = { 0xFF0000000000000B, 0 };
    sgx_key_id_t key_id = { {0} };

    sgx_key_request_t kr = {
        key_name: 4,
        key_policy: 2,
        isv_svn: report->body.isv_svn,
        reserved1: 0,
        cpu_svn: report->body.cpu_svn,
        attribute_mask: attribute_mas,
        key_id: key_id,
        misc_mask: 0,
        reserved2: {0}};

    sgx_key_128bit_t res;

    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
      sgx_status_t x = sgx_get_key(&kr, &res);
    }
    ocall_get_timestamp(&tend);
    printf("sgx_get_key, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

void bench_sgx_create_report() {
    sgx_target_info_t ti = {0};
    sgx_report_data_t rd = {0};
    sgx_report_t re = {0};
    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
        sgx_status_t x = sgx_create_report(&ti, &rd, &re);
    }
    ocall_get_timestamp(&tend);
    printf("sgx_create_report, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

void bench_sgx_verify_report() {
    sgx_target_info_t ti = {0};
    sgx_report_data_t rd = {0};
    sgx_report_t re = {0};
    struct my_timespec tstart, tend;
    sgx_create_report(&ti, &rd, &re);
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
        sgx_verify_report(&re);
    }
    ocall_get_timestamp(&tend);
    printf("sgx_verify_report, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

void bench_sgx_self_report() {
    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
        assert (0 != sgx_self_report());
    }
    ocall_get_timestamp(&tend);
    printf("sgx_self_report, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

void bench_sgx_seal_data() {
    uint8_t *add = NULL;
    uint8_t *payload = malloc(100000);
    uint8_t *result = malloc(110000);
    sgx_read_rand(payload, 100000);
    int sz = 1;
    struct my_timespec tstart, tend;
    for (int i = 0; i < 6; i ++) {
        ocall_get_timestamp(&tstart);
        uint32_t sealed_size = sgx_calc_sealed_data_size(0, sz);
        for(int i = 0; i < TEST_COUNT; i ++) {
            sgx_seal_data(0, add, sz, payload, sealed_size, result);
        }
        ocall_get_timestamp(&tend);
        printf("sgx_seal_data size = %d, %.9f\n", sz,
               ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
               ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
        sz *= 10;
    }
    free(payload);
    free(result);
}

void bench_sgx_unseal_data() {
    uint8_t *add = NULL;
    uint8_t *payload = malloc(100000);
    uint8_t *result = malloc(110000);
    sgx_read_rand(payload, 100000);
    int sz = 1;
    struct my_timespec tstart, tend;
    for (int i = 0; i < 6; i ++) {
        ocall_get_timestamp(&tstart);
        uint32_t sealed_size = sgx_calc_sealed_data_size(0, sz);
        sgx_seal_data(0, add, sz, payload, sealed_size, result);
        uint32_t result_len = sz;
        for(int i = 0; i < TEST_COUNT; i ++) {
            sgx_unseal_data(result, 0, 0, payload, &result_len);
        }
        ocall_get_timestamp(&tend);
        printf("sgx_unseal_data size = %d, %.9f\n", sz,
               ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
               ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
        sz *= 10;
    }
    free(payload);
    free(result);
}

void bench_sgx_fopen() {
    const sgx_report_t *report = sgx_self_report();

    sgx_attributes_t attribute_mas = { 0xFF0000000000000B, 0 };
    sgx_key_id_t key_id = { {0} };

    sgx_key_request_t kr = {
        key_name: 4,
        key_policy: 2,
        isv_svn: report->body.isv_svn,
        reserved1: 0,
        cpu_svn: report->body.cpu_svn,
        attribute_mask: attribute_mas,
        key_id: key_id,
        misc_mask: 0,
        reserved2: {0}};

    sgx_key_128bit_t res;

    sgx_get_key(&kr, &res);

    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
        SGX_FILE* s = sgx_fopen("sgx_file", "rw", &kr);
        sgx_fclose(s);
    }
    ocall_get_timestamp(&tend);
    printf("sgx_fopen, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

void bench_sgx_fopen_autokey() {
    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
        SGX_FILE* s = sgx_fopen_auto_key("sgx_file", "w");
        sgx_fclose(s);
    }
    ocall_get_timestamp(&tend);
    printf("sgx_fopen_auto_key, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

void bench_sgx_export_autokey() {
    SGX_FILE* s = sgx_fopen_auto_key("sgx_file", "w");
        sgx_fclose(s);
    sgx_key_128bit_t result_key;
    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
        sgx_fexport_auto_key("sgx_file", &result_key);
    }
    ocall_get_timestamp(&tend);
    printf("sgx_fexport_auto_key, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

void bench_sgx_import_autokey() {
    SGX_FILE* s = sgx_fopen_auto_key("sgx_file", "w");
        sgx_fclose(s);
    sgx_key_128bit_t result_key;
    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    for(int i = 0; i < TEST_COUNT; i ++) {
        sgx_fexport_auto_key("sgx_file", &result_key);
        sgx_fexport_auto_key("sgx_file", &result_key);
    }
    ocall_get_timestamp(&tend);
    printf("sgx_fimport_auto_key + sgx_fexport_autokey, %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
}

int uniform(){
    struct my_timespec tstart, tend;
    ocall_get_timestamp(&tstart);
    ocall_get_timestamp(&tend);
    printf("time measurement cost = %.9f\n",
           ((double)tend.tv_sec + 1.0e-9*tend.tv_nsec) -
           ((double)tstart.tv_sec + 1.0e-9*tstart.tv_nsec));
    ocall_get_timestamp(&tstart);
    bench_sgx_get_key();
    bench_sgx_create_report();
    bench_sgx_verify_report();
    bench_sgx_self_report();
    bench_sgx_seal_data();
    bench_sgx_unseal_data();
    bench_sgx_fopen();
    bench_sgx_fopen_autokey();
    bench_sgx_export_autokey();
    bench_sgx_import_autokey();
    return 0;
}
