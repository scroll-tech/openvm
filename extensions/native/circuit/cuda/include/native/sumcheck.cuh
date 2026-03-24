#pragma once

#include "primitives/constants.h"
#include "system/memory/offline_checker.cuh"

using namespace native;

template <typename T> struct HeaderSpecificCols {
    T pc;
    T registers[5];
    T prod_evals_id;
    T logup_evals_id;
    MemoryReadAuxCols<T> read_records[8];
    MemoryWriteAuxCols<T, EXT_DEG> write_records;
};

template <typename T> struct ProdSpecificCols {
    T data_ptr;
    T p[EXT_DEG * 2];
    T p_evals[EXT_DEG];
    MemoryWriteAuxCols<T, EXT_DEG> write_record;
    MemoryWriteAuxCols<T, EXT_DEG * 2> ps_record;
    T eval_rlc[EXT_DEG];
};

template <typename T> struct LogupSpecificCols {
    T data_ptr;
    T pq[EXT_DEG * 4];
    T p_evals[EXT_DEG];
    T q_evals[EXT_DEG];
    MemoryWriteAuxCols<T, EXT_DEG * 4> pqs_record;
    MemoryWriteAuxCols<T, EXT_DEG> write_records[2];
    T eval_rlc[EXT_DEG];
};

template <typename T> constexpr T constexpr_max(T a, T b) {
    return a > b ? a : b;
}

constexpr size_t COL_SPECIFIC_WIDTH = constexpr_max(
    sizeof(HeaderSpecificCols<uint8_t>),
    constexpr_max(sizeof(ProdSpecificCols<uint8_t>), sizeof(LogupSpecificCols<uint8_t>))
);

template <typename T> struct NativeSumcheckCols {
    T header_row;
    T prod_row;
    T logup_row;
    T is_end;

    T prod_continued;
    T logup_continued;

    T prod_in_round_evaluation;
    T prod_next_round_evaluation;
    T logup_in_round_evaluation;
    T logup_next_round_evaluation;

    T prod_acc;
    T logup_acc;

    T first_timestamp;
    T start_timestamp;
    T last_timestamp;

    T register_ptrs[5];

    T ctx[EXT_DEG * 2];

    T prod_nested_len;
    T logup_nested_len;

    T curr_prod_n;
    T curr_logup_n;

    T alpha[EXT_DEG];
    T challenges[EXT_DEG * 4];

    T max_round;
    T within_round_limit;
    T should_acc;

    T eval_acc[EXT_DEG];

    T is_writeback;

    T prod_hint_id;
    T logup_hint_id;

    T specific[COL_SPECIFIC_WIDTH];
};

