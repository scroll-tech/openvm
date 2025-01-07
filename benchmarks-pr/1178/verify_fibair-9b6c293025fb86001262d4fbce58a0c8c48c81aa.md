| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+0 [+4.0%])</span> 1.53 | <span style='color: red'>(+0 [+4.0%])</span> 1.53 |
| verify_fibair | <span style='color: red'>(+0 [+4.0%])</span> 1.53 | <span style='color: red'>(+0 [+4.0%])</span> 1.53 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+59 [+4.0%])</span> 1,530 | <span style='color: red'>(+59 [+4.0%])</span> 1,530 | <span style='color: red'>(+59 [+4.0%])</span> 1,530 | <span style='color: red'>(+59 [+4.0%])</span> 1,530 |
| `main_cells_used     ` | <span style='color: green'>(-13900 [-0.2%])</span> 8,012,912 | <span style='color: green'>(-13900 [-0.2%])</span> 8,012,912 | <span style='color: green'>(-13900 [-0.2%])</span> 8,012,912 | <span style='color: green'>(-13900 [-0.2%])</span> 8,012,912 |
| `total_cycles        ` | <span style='color: green'>(-372 [-0.2%])</span> 194,754 | <span style='color: green'>(-372 [-0.2%])</span> 194,754 | <span style='color: green'>(-372 [-0.2%])</span> 194,754 | <span style='color: green'>(-372 [-0.2%])</span> 194,754 |
| `execute_time_ms     ` | <span style='color: red'>(+24 [+40.0%])</span> 84 | <span style='color: red'>(+24 [+40.0%])</span> 84 | <span style='color: red'>(+24 [+40.0%])</span> 84 | <span style='color: red'>(+24 [+40.0%])</span> 84 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-92 [-66.2%])</span> 47 | <span style='color: green'>(-92 [-66.2%])</span> 47 | <span style='color: green'>(-92 [-66.2%])</span> 47 | <span style='color: green'>(-92 [-66.2%])</span> 47 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+127 [+10.0%])</span> 1,399 | <span style='color: red'>(+127 [+10.0%])</span> 1,399 | <span style='color: red'>(+127 [+10.0%])</span> 1,399 | <span style='color: red'>(+127 [+10.0%])</span> 1,399 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-8 [-3.6%])</span> 216 | <span style='color: green'>(-8 [-3.6%])</span> 216 | <span style='color: green'>(-8 [-3.6%])</span> 216 | <span style='color: green'>(-8 [-3.6%])</span> 216 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-2 [-7.4%])</span> 25 | <span style='color: green'>(-2 [-7.4%])</span> 25 | <span style='color: green'>(-2 [-7.4%])</span> 25 | <span style='color: green'>(-2 [-7.4%])</span> 25 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-10 [-5.2%])</span> 184 | <span style='color: green'>(-10 [-5.2%])</span> 184 | <span style='color: green'>(-10 [-5.2%])</span> 184 | <span style='color: green'>(-10 [-5.2%])</span> 184 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+28 [+12.6%])</span> 251 | <span style='color: red'>(+28 [+12.6%])</span> 251 | <span style='color: red'>(+28 [+12.6%])</span> 251 | <span style='color: red'>(+28 [+12.6%])</span> 251 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+12 [+4.8%])</span> 263 | <span style='color: red'>(+12 [+4.8%])</span> 263 | <span style='color: red'>(+12 [+4.8%])</span> 263 | <span style='color: red'>(+12 [+4.8%])</span> 263 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+107 [+30.5%])</span> 458 | <span style='color: red'>(+107 [+30.5%])</span> 458 | <span style='color: red'>(+107 [+30.5%])</span> 458 | <span style='color: red'>(+107 [+30.5%])</span> 458 |



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
| verify_fibair | 47 | 1,530 | 194,754 | 23,304,216 | 1,399 | 251 | 263 | 184 | 458 | 216 | 8,012,912 | 25 | 84 | 

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


Commit: https://github.com/openvm-org/openvm/commit/9b6c293025fb86001262d4fbce58a0c8c48c81aa

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12656094038)
