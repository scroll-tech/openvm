#include "launcher.cuh"
#include "primitives/trace_access.h"

// Columns layout matches HintSpaceProviderCols<T> in hint_space_provider.rs
// Fields: hint_id, offset, value, is_valid
template <typename T> struct HintSpaceProviderCols {
    T hint_id;
    T offset;
    T value;
    T is_valid;
};

constexpr uint32_t HINT_SPACE_PROVIDER_WIDTH = sizeof(HintSpaceProviderCols<uint8_t>);

__global__ void hint_space_provider_tracegen(
    Fp *trace,
    size_t height,
    const Fp *records,
    size_t rows_used
) {
    uint32_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= height) {
        return;
    }

    RowSlice row(trace + idx, height);
    if (idx < rows_used) {
        // Each record is a triple (hint_id, offset, value)
        const Fp *rec = records + idx * 3;
        COL_WRITE_VALUE(row, HintSpaceProviderCols, hint_id, rec[0]);
        COL_WRITE_VALUE(row, HintSpaceProviderCols, offset, rec[1]);
        COL_WRITE_VALUE(row, HintSpaceProviderCols, value, rec[2]);
        COL_WRITE_VALUE(row, HintSpaceProviderCols, is_valid, Fp::one());
    } else {
        row.fill_zero(0, HINT_SPACE_PROVIDER_WIDTH);
    }
}

extern "C" int _hint_space_provider_tracegen(
    Fp *d_trace,
    size_t height,
    size_t width,
    const Fp *d_records,
    size_t rows_used
) {
    assert((height & (height - 1)) == 0);
    assert(width == HINT_SPACE_PROVIDER_WIDTH);
    auto [grid, block] = kernel_launch_params(height);
    hint_space_provider_tracegen<<<grid, block>>>(
        d_trace,
        height,
        d_records,
        rows_used
    );
    return CHECK_KERNEL();
}
