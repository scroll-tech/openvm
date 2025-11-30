#include "launcher.cuh"
#include "native/sumcheck.cuh"
#include "native/utils.cuh"
#include "primitives/trace_access.h"
#include "system/memory/controller.cuh"

using namespace native;

__device__ void fill_sumcheck_specific(RowSlice row, MemoryAuxColsFactory &mem_helper) {
    RowSlice specific = row.slice_from(COL_INDEX(NativeSumcheckCols, specific));
    uint32_t start_timestamp = row[COL_INDEX(NativeSumcheckCols, start_timestamp)].asUInt32();

    if (row[COL_INDEX(NativeSumcheckCols, header_row)] == Fp::one()) {
        for (uint32_t i = 0; i < 7; ++i) {
            mem_fill_base(
                mem_helper,
                start_timestamp + i,
                specific.slice_from(COL_INDEX(HeaderSpecificCols, read_records[i].base))
            );
        }
        uint32_t last_timestamp = row[COL_INDEX(NativeSumcheckCols, last_timestamp)].asUInt32();
        mem_fill_base(
            mem_helper,
            last_timestamp - 1,
            specific.slice_from(COL_INDEX(HeaderSpecificCols, write_records.base))
        );
    } else if (row[COL_INDEX(NativeSumcheckCols, prod_row)] == Fp::one()) {
        mem_fill_base(
            mem_helper,
            start_timestamp,
            specific.slice_from(COL_INDEX(ProdSpecificCols, read_records[0].base))
        );
        if (row[COL_INDEX(NativeSumcheckCols, within_round_limit)] == Fp::one()) {
            mem_fill_base(
                mem_helper,
                start_timestamp + 1,
                specific.slice_from(COL_INDEX(ProdSpecificCols, read_records[1].base))
            );
            mem_fill_base(
                mem_helper,
                start_timestamp + 2,
                specific.slice_from(COL_INDEX(ProdSpecificCols, write_record.base))
            );
        }
    } else if (row[COL_INDEX(NativeSumcheckCols, logup_row)] == Fp::one()) {
        mem_fill_base(
            mem_helper,
            start_timestamp,
            specific.slice_from(COL_INDEX(LogupSpecificCols, read_records[0].base))
        );
        if (row[COL_INDEX(NativeSumcheckCols, within_round_limit)] == Fp::one()) {
            mem_fill_base(
                mem_helper,
                start_timestamp + 1,
                specific.slice_from(COL_INDEX(LogupSpecificCols, read_records[1].base))
            );
            mem_fill_base(
                mem_helper,
                start_timestamp + 2,
                specific.slice_from(COL_INDEX(LogupSpecificCols, write_records[0].base))
            );
            mem_fill_base(
                mem_helper,
                start_timestamp + 3,
                specific.slice_from(COL_INDEX(LogupSpecificCols, write_records[1].base))
            );
        }
    }
}

__global__ void native_sumcheck_tracegen(
    Fp *trace,
    size_t height,
    size_t width,
    const Fp *records,
    size_t rows_used,
    uint32_t *range_checker_ptr,
    uint32_t range_checker_num_bins,
    uint32_t timestamp_max_bits
) {
    uint32_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= height) {
        return;
    }

    RowSlice row(trace + idx, height);
    if (idx < rows_used) {
        const Fp *record = records + idx * width;
        for (uint32_t col = 0; col < width; ++col) {
            row[col] = record[col];
        }
        MemoryAuxColsFactory mem_helper(
            VariableRangeChecker(range_checker_ptr, range_checker_num_bins), timestamp_max_bits
        );
        fill_sumcheck_specific(row, mem_helper);
    } else {
        row.fill_zero(0, width);
        COL_WRITE_VALUE(row, NativeSumcheckCols, is_end, Fp::one());
    }
}

extern "C" int _native_sumcheck_tracegen(
    Fp *d_trace,
    size_t height,
    size_t width,
    const Fp *d_records,
    size_t rows_used,
    uint32_t *d_range_checker,
    uint32_t range_checker_num_bins,
    uint32_t timestamp_max_bits
) {
    assert((height & (height - 1)) == 0);
    assert(width == sizeof(NativeSumcheckCols<uint8_t>));
    auto [grid, block] = kernel_launch_params(height);
    native_sumcheck_tracegen<<<grid, block>>>(
        d_trace,
        height,
        width,
        d_records,
        rows_used,
        d_range_checker,
        range_checker_num_bins,
        timestamp_max_bits
    );
    return CHECK_KERNEL();
}
