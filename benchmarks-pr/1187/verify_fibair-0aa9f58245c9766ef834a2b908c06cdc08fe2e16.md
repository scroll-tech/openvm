| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+2 [+163.5%])</span> 3.85 | <span style='color: red'>(+2 [+163.5%])</span> 3.85 |
| verify_fibair | <span style='color: red'>(+2 [+163.5%])</span> 3.85 | <span style='color: red'>(+2 [+163.5%])</span> 3.85 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+2391 [+163.5%])</span> 3,853 | <span style='color: red'>(+2391 [+163.5%])</span> 3,853 | <span style='color: red'>(+2391 [+163.5%])</span> 3,853 | <span style='color: red'>(+2391 [+163.5%])</span> 3,853 |
| `main_cells_used     ` | <span style='color: red'>(+21985102 [+273.8%])</span> 30,013,314 | <span style='color: red'>(+21985102 [+273.8%])</span> 30,013,314 | <span style='color: red'>(+21985102 [+273.8%])</span> 30,013,314 | <span style='color: red'>(+21985102 [+273.8%])</span> 30,013,314 |
| `total_cycles        ` | <span style='color: red'>(+551158 [+282.4%])</span> 746,333 | <span style='color: red'>(+551158 [+282.4%])</span> 746,333 | <span style='color: red'>(+551158 [+282.4%])</span> 746,333 | <span style='color: red'>(+551158 [+282.4%])</span> 746,333 |
| `execute_time_ms     ` | <span style='color: red'>(+168 [+271.0%])</span> 230 | <span style='color: red'>(+168 [+271.0%])</span> 230 | <span style='color: red'>(+168 [+271.0%])</span> 230 | <span style='color: red'>(+168 [+271.0%])</span> 230 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+355 [+255.4%])</span> 494 | <span style='color: red'>(+355 [+255.4%])</span> 494 | <span style='color: red'>(+355 [+255.4%])</span> 494 | <span style='color: red'>(+355 [+255.4%])</span> 494 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+1868 [+148.1%])</span> 3,129 | <span style='color: red'>(+1868 [+148.1%])</span> 3,129 | <span style='color: red'>(+1868 [+148.1%])</span> 3,129 | <span style='color: red'>(+1868 [+148.1%])</span> 3,129 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+364 [+172.5%])</span> 575 | <span style='color: red'>(+364 [+172.5%])</span> 575 | <span style='color: red'>(+364 [+172.5%])</span> 575 | <span style='color: red'>(+364 [+172.5%])</span> 575 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+55 [+171.9%])</span> 87 | <span style='color: red'>(+55 [+171.9%])</span> 87 | <span style='color: red'>(+55 [+171.9%])</span> 87 | <span style='color: red'>(+55 [+171.9%])</span> 87 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+318 [+171.0%])</span> 504 | <span style='color: red'>(+318 [+171.0%])</span> 504 | <span style='color: red'>(+318 [+171.0%])</span> 504 | <span style='color: red'>(+318 [+171.0%])</span> 504 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+437 [+196.0%])</span> 660 | <span style='color: red'>(+437 [+196.0%])</span> 660 | <span style='color: red'>(+437 [+196.0%])</span> 660 | <span style='color: red'>(+437 [+196.0%])</span> 660 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+310 [+126.5%])</span> 555 | <span style='color: red'>(+310 [+126.5%])</span> 555 | <span style='color: red'>(+310 [+126.5%])</span> 555 | <span style='color: red'>(+310 [+126.5%])</span> 555 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+384 [+106.4%])</span> 745 | <span style='color: red'>(+384 [+106.4%])</span> 745 | <span style='color: red'>(+384 [+106.4%])</span> 745 | <span style='color: red'>(+384 [+106.4%])</span> 745 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 3 | 65,536 | 66 | 3 | 13 | 0 | 31 | 17 | 

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
| verify_fibair | 494 | 3,853 | 746,333 | 89,839,640 | 3,129 | 660 | 555 | 504 | 745 | 575 | 30,013,314 | 87 | 230 | 

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


Commit: https://github.com/openvm-org/openvm/commit/0aa9f58245c9766ef834a2b908c06cdc08fe2e16

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12665035643)
