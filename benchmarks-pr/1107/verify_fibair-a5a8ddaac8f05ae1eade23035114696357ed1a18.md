| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(-2 [-61.2%])</span> 1.49 | <span style='color: green'>(-2 [-61.2%])</span> 1.49 |
| verify_fibair | <span style='color: green'>(-2 [-61.2%])</span> 1.49 | <span style='color: green'>(-2 [-61.2%])</span> 1.49 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-2340 [-61.2%])</span> 1,485 | <span style='color: green'>(-2340 [-61.2%])</span> 1,485 | <span style='color: green'>(-2340 [-61.2%])</span> 1,485 | <span style='color: green'>(-2340 [-61.2%])</span> 1,485 |
| `main_cells_used     ` | <span style='color: green'>(-21988032 [-73.3%])</span> 8,027,672 | <span style='color: green'>(-21988032 [-73.3%])</span> 8,027,672 | <span style='color: green'>(-21988032 [-73.3%])</span> 8,027,672 | <span style='color: green'>(-21988032 [-73.3%])</span> 8,027,672 |
| `total_cycles        ` | <span style='color: green'>(-551360 [-73.9%])</span> 195,184 | <span style='color: green'>(-551360 [-73.9%])</span> 195,184 | <span style='color: green'>(-551360 [-73.9%])</span> 195,184 | <span style='color: green'>(-551360 [-73.9%])</span> 195,184 |
| `execute_time_ms     ` | <span style='color: green'>(-176 [-74.6%])</span> 60 | <span style='color: green'>(-176 [-74.6%])</span> 60 | <span style='color: green'>(-176 [-74.6%])</span> 60 | <span style='color: green'>(-176 [-74.6%])</span> 60 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-350 [-71.4%])</span> 140 | <span style='color: green'>(-350 [-71.4%])</span> 140 | <span style='color: green'>(-350 [-71.4%])</span> 140 | <span style='color: green'>(-350 [-71.4%])</span> 140 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-1814 [-58.5%])</span> 1,285 | <span style='color: green'>(-1814 [-58.5%])</span> 1,285 | <span style='color: green'>(-1814 [-58.5%])</span> 1,285 | <span style='color: green'>(-1814 [-58.5%])</span> 1,285 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-338 [-60.1%])</span> 224 | <span style='color: green'>(-338 [-60.1%])</span> 224 | <span style='color: green'>(-338 [-60.1%])</span> 224 | <span style='color: green'>(-338 [-60.1%])</span> 224 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-75 [-75.8%])</span> 24 | <span style='color: green'>(-75 [-75.8%])</span> 24 | <span style='color: green'>(-75 [-75.8%])</span> 24 | <span style='color: green'>(-75 [-75.8%])</span> 24 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-307 [-61.6%])</span> 191 | <span style='color: green'>(-307 [-61.6%])</span> 191 | <span style='color: green'>(-307 [-61.6%])</span> 191 | <span style='color: green'>(-307 [-61.6%])</span> 191 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-430 [-65.3%])</span> 229 | <span style='color: green'>(-430 [-65.3%])</span> 229 | <span style='color: green'>(-430 [-65.3%])</span> 229 | <span style='color: green'>(-430 [-65.3%])</span> 229 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-295 [-53.3%])</span> 258 | <span style='color: green'>(-295 [-53.3%])</span> 258 | <span style='color: green'>(-295 [-53.3%])</span> 258 | <span style='color: green'>(-295 [-53.3%])</span> 258 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-367 [-50.7%])</span> 357 | <span style='color: green'>(-367 [-50.7%])</span> 357 | <span style='color: green'>(-367 [-50.7%])</span> 357 | <span style='color: green'>(-367 [-50.7%])</span> 357 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 4 | 32 | 11 | 0 | 1 | 0 | 3 | 5 | 

| air_name | rows | quotient_deg | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- |
| AccessAdapterAir<2> |  | 4 |  | 5 | 12 |  | 
| AccessAdapterAir<4> |  | 4 |  | 5 | 12 |  | 
| AccessAdapterAir<8> |  | 4 |  | 5 | 12 |  | 
| FibonacciAir | 16 | 1 | 2 |  | 5 | 32 | 
| FriReducedOpeningAir |  | 4 |  | 35 | 59 |  | 
| NativePoseidon2Air<BabyBearParameters>, 1> |  | 4 |  | 31 | 302 |  | 
| PhantomAir |  | 4 |  | 3 | 4 |  | 
| ProgramAir |  | 1 |  | 1 | 4 |  | 
| VariableRangeCheckerAir |  | 1 |  | 1 | 4 |  | 
| VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> |  | 2 |  | 11 | 23 |  | 
| VmAirWrapper<JalNativeAdapterAir, JalCoreAir> |  | 4 |  | 7 | 6 |  | 
| VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> |  | 4 |  | 11 | 22 |  | 
| VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> |  | 4 |  | 15 | 23 |  | 
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> |  | 4 |  | 19 | 31 |  | 
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> |  | 4 |  | 15 | 23 |  | 
| VmConnectorAir |  | 4 |  | 3 | 8 |  | 
| VolatileBoundaryAir |  | 4 |  | 4 | 16 |  | 

| group | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | 140 | 1,485 | 195,184 | 23,304,216 | 1,285 | 229 | 258 | 191 | 357 | 224 | 8,027,672 | 24 | 60 | 

| group | air_name | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | AccessAdapterAir<2> | 32,768 |  | 16 | 11 | 884,736 | 
| verify_fibair | AccessAdapterAir<4> | 16,384 |  | 16 | 13 | 475,136 | 
| verify_fibair | AccessAdapterAir<8> | 4,096 |  | 16 | 17 | 135,168 | 
| verify_fibair | FriReducedOpeningAir | 512 |  | 76 | 64 | 71,680 | 
| verify_fibair | NativePoseidon2Air<BabyBearParameters>, 1> | 2,048 |  | 36 | 348 | 786,432 | 
| verify_fibair | PhantomAir | 2,048 |  | 8 | 6 | 28,672 | 
| verify_fibair | ProgramAir | 8,192 |  | 8 | 10 | 147,456 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 32,768 |  | 28 | 23 | 1,671,168 | 
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 8,192 |  | 12 | 10 | 180,224 | 
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 131,072 |  | 20 | 30 | 6,553,600 | 
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 131,072 |  | 24 | 41 | 8,519,680 | 
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 4,096 |  | 20 | 40 | 245,760 | 
| verify_fibair | VmConnectorAir | 2 | 1 | 8 | 4 | 24 | 
| verify_fibair | VolatileBoundaryAir | 65,536 |  | 8 | 11 | 1,245,184 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/a5a8ddaac8f05ae1eade23035114696357ed1a18

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12667141727)
