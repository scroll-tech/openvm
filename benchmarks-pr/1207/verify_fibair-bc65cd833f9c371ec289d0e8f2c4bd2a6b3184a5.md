| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(-0 [-0.5%])</span> 3.79 | <span style='color: green'>(-0 [-0.5%])</span> 3.79 |
| verify_fibair | <span style='color: green'>(-0 [-0.5%])</span> 3.79 | <span style='color: green'>(-0 [-0.5%])</span> 3.79 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-18 [-0.5%])</span> 3,786 | <span style='color: green'>(-18 [-0.5%])</span> 3,786 | <span style='color: green'>(-18 [-0.5%])</span> 3,786 | <span style='color: green'>(-18 [-0.5%])</span> 3,786 |
| `main_cells_used     ` |  30,012,414 |  30,012,414 |  30,012,414 |  30,012,414 |
| `total_cycles        ` |  746,376 |  746,376 |  746,376 |  746,376 |
| `execute_time_ms     ` | <span style='color: green'>(-1 [-0.6%])</span> 156 | <span style='color: green'>(-1 [-0.6%])</span> 156 | <span style='color: green'>(-1 [-0.6%])</span> 156 | <span style='color: green'>(-1 [-0.6%])</span> 156 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-20 [-4.5%])</span> 429 | <span style='color: green'>(-20 [-4.5%])</span> 429 | <span style='color: green'>(-20 [-4.5%])</span> 429 | <span style='color: green'>(-20 [-4.5%])</span> 429 |
| `stark_prove_excluding_trace_time_ms` |  3,201 |  3,201 |  3,201 |  3,201 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-3 [-0.5%])</span> 616 | <span style='color: green'>(-3 [-0.5%])</span> 616 | <span style='color: green'>(-3 [-0.5%])</span> 616 | <span style='color: green'>(-3 [-0.5%])</span> 616 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-10 [-9.6%])</span> 94 | <span style='color: green'>(-10 [-9.6%])</span> 94 | <span style='color: green'>(-10 [-9.6%])</span> 94 | <span style='color: green'>(-10 [-9.6%])</span> 94 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+23 [+4.3%])</span> 556 | <span style='color: red'>(+23 [+4.3%])</span> 556 | <span style='color: red'>(+23 [+4.3%])</span> 556 | <span style='color: red'>(+23 [+4.3%])</span> 556 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-17 [-2.7%])</span> 624 | <span style='color: green'>(-17 [-2.7%])</span> 624 | <span style='color: green'>(-17 [-2.7%])</span> 624 | <span style='color: green'>(-17 [-2.7%])</span> 624 |
| `quotient_poly_commit_time_ms` |  569 |  569 |  569 |  569 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+10 [+1.4%])</span> 740 | <span style='color: red'>(+10 [+1.4%])</span> 740 | <span style='color: red'>(+10 [+1.4%])</span> 740 | <span style='color: red'>(+10 [+1.4%])</span> 740 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 3 | 65,536 | 69 | 3 | 13 | 0 | 36 | 15 | 

| air_name | rows | quotient_deg | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- |
| AccessAdapterAir<2> |  | 4 |  | 5 | 12 |  | 
| AccessAdapterAir<4> |  | 4 |  | 5 | 12 |  | 
| AccessAdapterAir<8> |  | 4 |  | 5 | 12 |  | 
| FibonacciAir | 32,768 | 1 | 2 |  | 5 | 65,536 | 
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
| verify_fibair | 429 | 3,786 | 746,376 | 89,839,640 | 3,201 | 624 | 569 | 556 | 740 | 616 | 30,012,414 | 94 | 156 | 

| group | air_name | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | AccessAdapterAir<2> | 131,072 |  | 16 | 11 | 3,538,944 | 
| verify_fibair | AccessAdapterAir<4> | 65,536 |  | 16 | 13 | 1,900,544 | 
| verify_fibair | AccessAdapterAir<8> | 32,768 |  | 16 | 17 | 1,081,344 | 
| verify_fibair | FriReducedOpeningAir | 512 |  | 76 | 64 | 71,680 | 
| verify_fibair | NativePoseidon2Air<BabyBearParameters>, 1> | 8,192 |  | 36 | 348 | 3,145,728 | 
| verify_fibair | PhantomAir | 16,384 |  | 8 | 6 | 229,376 | 
| verify_fibair | ProgramAir | 8,192 |  | 8 | 10 | 147,456 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 262,144 |  | 28 | 23 | 13,369,344 | 
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 32,768 |  | 12 | 10 | 720,896 | 
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 524,288 |  | 20 | 30 | 26,214,400 | 
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 524,288 |  | 24 | 41 | 34,078,720 | 
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 8,192 |  | 20 | 40 | 491,520 | 
| verify_fibair | VmConnectorAir | 2 | 1 | 8 | 4 | 24 | 
| verify_fibair | VolatileBoundaryAir | 131,072 |  | 8 | 11 | 2,490,368 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/bc65cd833f9c371ec289d0e8f2c4bd2a6b3184a5

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12725418771)
