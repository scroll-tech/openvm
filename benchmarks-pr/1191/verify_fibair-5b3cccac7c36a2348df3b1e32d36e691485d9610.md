| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+0 [+0.7%])</span> 3.82 | <span style='color: red'>(+0 [+0.7%])</span> 3.82 |
| verify_fibair | <span style='color: red'>(+0 [+0.7%])</span> 3.82 | <span style='color: red'>(+0 [+0.7%])</span> 3.82 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+26 [+0.7%])</span> 3,820 | <span style='color: red'>(+26 [+0.7%])</span> 3,820 | <span style='color: red'>(+26 [+0.7%])</span> 3,820 | <span style='color: red'>(+26 [+0.7%])</span> 3,820 |
| `main_cells_used     ` |  30,014,624 |  30,014,624 |  30,014,624 |  30,014,624 |
| `total_cycles        ` |  746,492 |  746,492 |  746,492 |  746,492 |
| `execute_time_ms     ` | <span style='color: green'>(-3 [-1.9%])</span> 154 | <span style='color: green'>(-3 [-1.9%])</span> 154 | <span style='color: green'>(-3 [-1.9%])</span> 154 | <span style='color: green'>(-3 [-1.9%])</span> 154 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-16 [-3.6%])</span> 429 | <span style='color: green'>(-16 [-3.6%])</span> 429 | <span style='color: green'>(-16 [-3.6%])</span> 429 | <span style='color: green'>(-16 [-3.6%])</span> 429 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+45 [+1.4%])</span> 3,237 | <span style='color: red'>(+45 [+1.4%])</span> 3,237 | <span style='color: red'>(+45 [+1.4%])</span> 3,237 | <span style='color: red'>(+45 [+1.4%])</span> 3,237 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+2 [+0.3%])</span> 619 | <span style='color: red'>(+2 [+0.3%])</span> 619 | <span style='color: red'>(+2 [+0.3%])</span> 619 | <span style='color: red'>(+2 [+0.3%])</span> 619 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+1 [+1.1%])</span> 89 | <span style='color: red'>(+1 [+1.1%])</span> 89 | <span style='color: red'>(+1 [+1.1%])</span> 89 | <span style='color: red'>(+1 [+1.1%])</span> 89 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-3 [-0.6%])</span> 533 | <span style='color: green'>(-3 [-0.6%])</span> 533 | <span style='color: green'>(-3 [-0.6%])</span> 533 | <span style='color: green'>(-3 [-0.6%])</span> 533 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+11 [+1.7%])</span> 650 | <span style='color: red'>(+11 [+1.7%])</span> 650 | <span style='color: red'>(+11 [+1.7%])</span> 650 | <span style='color: red'>(+11 [+1.7%])</span> 650 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+9 [+1.6%])</span> 582 | <span style='color: red'>(+9 [+1.6%])</span> 582 | <span style='color: red'>(+9 [+1.6%])</span> 582 | <span style='color: red'>(+9 [+1.6%])</span> 582 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+24 [+3.3%])</span> 760 | <span style='color: red'>(+24 [+3.3%])</span> 760 | <span style='color: red'>(+24 [+3.3%])</span> 760 | <span style='color: red'>(+24 [+3.3%])</span> 760 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 3 | 65,536 | 65 | 3 | 13 | 0 | 30 | 17 | 

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
| verify_fibair | 429 | 3,820 | 746,492 | 89,839,640 | 3,237 | 650 | 582 | 533 | 760 | 619 | 30,014,624 | 89 | 154 | 

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


Commit: https://github.com/openvm-org/openvm/commit/5b3cccac7c36a2348df3b1e32d36e691485d9610

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12700336385)
