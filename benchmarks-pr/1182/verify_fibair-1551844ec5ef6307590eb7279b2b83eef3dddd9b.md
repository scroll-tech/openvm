| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(-0 [-0.3%])</span> 1.43 | <span style='color: green'>(-0 [-0.3%])</span> 1.43 |
| verify_fibair | <span style='color: green'>(-0 [-0.3%])</span> 1.43 | <span style='color: green'>(-0 [-0.3%])</span> 1.43 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-4 [-0.3%])</span> 1,431 | <span style='color: green'>(-4 [-0.3%])</span> 1,431 | <span style='color: green'>(-4 [-0.3%])</span> 1,431 | <span style='color: green'>(-4 [-0.3%])</span> 1,431 |
| `main_cells_used     ` |  8,027,972 |  8,027,972 |  8,027,972 |  8,027,972 |
| `total_cycles        ` |  195,158 |  195,158 |  195,158 |  195,158 |
| `execute_time_ms     ` | <span style='color: green'>(-1 [-1.7%])</span> 58 | <span style='color: green'>(-1 [-1.7%])</span> 58 | <span style='color: green'>(-1 [-1.7%])</span> 58 | <span style='color: green'>(-1 [-1.7%])</span> 58 |
| `trace_gen_time_ms   ` |  136 |  136 |  136 |  136 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-3 [-0.2%])</span> 1,237 | <span style='color: green'>(-3 [-0.2%])</span> 1,237 | <span style='color: green'>(-3 [-0.2%])</span> 1,237 | <span style='color: green'>(-3 [-0.2%])</span> 1,237 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-1 [-0.5%])</span> 210 | <span style='color: green'>(-1 [-0.5%])</span> 210 | <span style='color: green'>(-1 [-0.5%])</span> 210 | <span style='color: green'>(-1 [-0.5%])</span> 210 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+6 [+26.1%])</span> 29 | <span style='color: red'>(+6 [+26.1%])</span> 29 | <span style='color: red'>(+6 [+26.1%])</span> 29 | <span style='color: red'>(+6 [+26.1%])</span> 29 |
| `perm_trace_commit_time_ms` |  181 |  181 |  181 |  181 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-10 [-4.5%])</span> 213 | <span style='color: green'>(-10 [-4.5%])</span> 213 | <span style='color: green'>(-10 [-4.5%])</span> 213 | <span style='color: green'>(-10 [-4.5%])</span> 213 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-5 [-2.0%])</span> 246 | <span style='color: green'>(-5 [-2.0%])</span> 246 | <span style='color: green'>(-5 [-2.0%])</span> 246 | <span style='color: green'>(-5 [-2.0%])</span> 246 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+7 [+2.0%])</span> 354 | <span style='color: red'>(+7 [+2.0%])</span> 354 | <span style='color: red'>(+7 [+2.0%])</span> 354 | <span style='color: red'>(+7 [+2.0%])</span> 354 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 4 | 32 | 9 | 0 | 1 | 0 | 2 | 5 | 

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
| verify_fibair | 136 | 1,431 | 195,158 | 23,304,216 | 1,237 | 213 | 246 | 181 | 354 | 210 | 8,027,972 | 29 | 58 | 

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


Commit: https://github.com/openvm-org/openvm/commit/1551844ec5ef6307590eb7279b2b83eef3dddd9b

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12656634146)
