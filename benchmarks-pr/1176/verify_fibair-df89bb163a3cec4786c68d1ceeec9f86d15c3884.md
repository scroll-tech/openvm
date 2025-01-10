| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(-0 [-4.0%])</span> 3.67 | <span style='color: green'>(-0 [-4.0%])</span> 3.67 |
| verify_fibair | <span style='color: green'>(-0 [-4.0%])</span> 3.67 | <span style='color: green'>(-0 [-4.0%])</span> 3.67 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-154 [-4.0%])</span> 3,672 | <span style='color: green'>(-154 [-4.0%])</span> 3,672 | <span style='color: green'>(-154 [-4.0%])</span> 3,672 | <span style='color: green'>(-154 [-4.0%])</span> 3,672 |
| `main_cells_used     ` | <span style='color: green'>(-2600930 [-8.7%])</span> 27,416,674 | <span style='color: green'>(-2600930 [-8.7%])</span> 27,416,674 | <span style='color: green'>(-2600930 [-8.7%])</span> 27,416,674 | <span style='color: green'>(-2600930 [-8.7%])</span> 27,416,674 |
| `total_cycles        ` | <span style='color: red'>(+3174 [+0.4%])</span> 749,761 | <span style='color: red'>(+3174 [+0.4%])</span> 749,761 | <span style='color: red'>(+3174 [+0.4%])</span> 749,761 | <span style='color: red'>(+3174 [+0.4%])</span> 749,761 |
| `execute_time_ms     ` | <span style='color: green'>(-4 [-2.6%])</span> 152 | <span style='color: green'>(-4 [-2.6%])</span> 152 | <span style='color: green'>(-4 [-2.6%])</span> 152 | <span style='color: green'>(-4 [-2.6%])</span> 152 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-8 [-1.8%])</span> 441 | <span style='color: green'>(-8 [-1.8%])</span> 441 | <span style='color: green'>(-8 [-1.8%])</span> 441 | <span style='color: green'>(-8 [-1.8%])</span> 441 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-142 [-4.4%])</span> 3,079 | <span style='color: green'>(-142 [-4.4%])</span> 3,079 | <span style='color: green'>(-142 [-4.4%])</span> 3,079 | <span style='color: green'>(-142 [-4.4%])</span> 3,079 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-40 [-6.4%])</span> 583 | <span style='color: green'>(-40 [-6.4%])</span> 583 | <span style='color: green'>(-40 [-6.4%])</span> 583 | <span style='color: green'>(-40 [-6.4%])</span> 583 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-13 [-14.4%])</span> 77 | <span style='color: green'>(-13 [-14.4%])</span> 77 | <span style='color: green'>(-13 [-14.4%])</span> 77 | <span style='color: green'>(-13 [-14.4%])</span> 77 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-33 [-6.1%])</span> 508 | <span style='color: green'>(-33 [-6.1%])</span> 508 | <span style='color: green'>(-33 [-6.1%])</span> 508 | <span style='color: green'>(-33 [-6.1%])</span> 508 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-74 [-11.3%])</span> 580 | <span style='color: green'>(-74 [-11.3%])</span> 580 | <span style='color: green'>(-74 [-11.3%])</span> 580 | <span style='color: green'>(-74 [-11.3%])</span> 580 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+7 [+1.2%])</span> 584 | <span style='color: red'>(+7 [+1.2%])</span> 584 | <span style='color: red'>(+7 [+1.2%])</span> 584 | <span style='color: red'>(+7 [+1.2%])</span> 584 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+11 [+1.5%])</span> 744 | <span style='color: red'>(+11 [+1.5%])</span> 744 | <span style='color: red'>(+11 [+1.5%])</span> 744 | <span style='color: red'>(+11 [+1.5%])</span> 744 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 4 | 65,536 | 68 | 3 | 14 | 0 | 34 | 16 | 

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
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> |  | 4 |  | 15 | 24 |  | 
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> |  | 4 |  | 15 | 23 |  | 
| VmConnectorAir |  | 4 |  | 3 | 8 |  | 
| VolatileBoundaryAir |  | 4 |  | 4 | 16 |  | 

| group | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | 441 | 3,672 | 749,761 | 82,499,608 | 3,079 | 580 | 584 | 508 | 744 | 583 | 27,416,674 | 77 | 152 | 

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
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 524,288 |  | 20 | 31 | 26,738,688 | 
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 8,192 |  | 20 | 40 | 491,520 | 
| verify_fibair | VmConnectorAir | 2 | 1 | 8 | 4 | 24 | 
| verify_fibair | VolatileBoundaryAir | 131,072 |  | 8 | 11 | 2,490,368 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/df89bb163a3cec4786c68d1ceeec9f86d15c3884

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12716481367)
