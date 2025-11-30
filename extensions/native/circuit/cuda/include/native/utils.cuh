#pragma once

#include "primitives/trace_access.h"
#include "system/memory/controller.cuh"

__device__ __forceinline__ void mem_fill_base(
    MemoryAuxColsFactory &mem_helper,
    uint32_t timestamp,
    RowSlice base_aux
) {
    uint32_t prev = base_aux[COL_INDEX(MemoryBaseAuxCols, prev_timestamp)].asUInt32();
    mem_helper.fill(base_aux, prev, timestamp);
}
