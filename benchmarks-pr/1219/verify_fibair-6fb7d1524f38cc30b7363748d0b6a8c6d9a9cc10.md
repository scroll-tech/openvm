| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(-1 [-31.4%])</span> 2.39 | <span style='color: green'>(-1 [-31.4%])</span> 2.39 |
| verify_fibair | <span style='color: green'>(-1 [-31.4%])</span> 2.39 | <span style='color: green'>(-1 [-31.4%])</span> 2.39 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-1094 [-31.4%])</span> 2,392 | <span style='color: green'>(-1094 [-31.4%])</span> 2,392 | <span style='color: green'>(-1094 [-31.4%])</span> 2,392 | <span style='color: green'>(-1094 [-31.4%])</span> 2,392 |
| `main_cells_used     ` | <span style='color: green'>(-6703242 [-26.3%])</span> 18,806,033 | <span style='color: green'>(-6703242 [-26.3%])</span> 18,806,033 | <span style='color: green'>(-6703242 [-26.3%])</span> 18,806,033 | <span style='color: green'>(-6703242 [-26.3%])</span> 18,806,033 |
| `total_cycles        ` | <span style='color: green'>(-216918 [-30.5%])</span> 494,199 | <span style='color: green'>(-216918 [-30.5%])</span> 494,199 | <span style='color: green'>(-216918 [-30.5%])</span> 494,199 | <span style='color: green'>(-216918 [-30.5%])</span> 494,199 |
| `execute_time_ms     ` | <span style='color: green'>(-48 [-33.1%])</span> 97 | <span style='color: green'>(-48 [-33.1%])</span> 97 | <span style='color: green'>(-48 [-33.1%])</span> 97 | <span style='color: green'>(-48 [-33.1%])</span> 97 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-80 [-18.6%])</span> 350 | <span style='color: green'>(-80 [-18.6%])</span> 350 | <span style='color: green'>(-80 [-18.6%])</span> 350 | <span style='color: green'>(-80 [-18.6%])</span> 350 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-966 [-33.2%])</span> 1,945 | <span style='color: green'>(-966 [-33.2%])</span> 1,945 | <span style='color: green'>(-966 [-33.2%])</span> 1,945 | <span style='color: green'>(-966 [-33.2%])</span> 1,945 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-173 [-33.8%])</span> 339 | <span style='color: green'>(-173 [-33.8%])</span> 339 | <span style='color: green'>(-173 [-33.8%])</span> 339 | <span style='color: green'>(-173 [-33.8%])</span> 339 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-20 [-30.3%])</span> 46 | <span style='color: green'>(-20 [-30.3%])</span> 46 | <span style='color: green'>(-20 [-30.3%])</span> 46 | <span style='color: green'>(-20 [-30.3%])</span> 46 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-165 [-31.9%])</span> 352 | <span style='color: green'>(-165 [-31.9%])</span> 352 | <span style='color: green'>(-165 [-31.9%])</span> 352 | <span style='color: green'>(-165 [-31.9%])</span> 352 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-203 [-35.3%])</span> 372 | <span style='color: green'>(-203 [-35.3%])</span> 372 | <span style='color: green'>(-203 [-35.3%])</span> 372 | <span style='color: green'>(-203 [-35.3%])</span> 372 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-171 [-31.3%])</span> 375 | <span style='color: green'>(-171 [-31.3%])</span> 375 | <span style='color: green'>(-171 [-31.3%])</span> 375 | <span style='color: green'>(-171 [-31.3%])</span> 375 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-234 [-33.8%])</span> 459 | <span style='color: green'>(-234 [-33.8%])</span> 459 | <span style='color: green'>(-234 [-33.8%])</span> 459 | <span style='color: green'>(-234 [-33.8%])</span> 459 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 3 | 65,536 | 66 | 3 | 13 | 0 | 32 | 17 | 

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
| VerifyBatchAir<BabyBearParameters>, 1> |  | 4 |  | 145 | 566 |  | 
| VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> |  | 2 |  | 11 | 23 |  | 
| VmAirWrapper<JalNativeAdapterAir, JalCoreAir> |  | 4 |  | 7 | 6 |  | 
| VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> |  | 4 |  | 11 | 22 |  | 
| VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> |  | 4 |  | 15 | 23 |  | 
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> |  | 4 |  | 15 | 20 |  | 
| VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> |  | 4 |  | 15 | 20 |  | 
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> |  | 4 |  | 15 | 23 |  | 
| VmConnectorAir |  | 4 |  | 3 | 8 |  | 
| VolatileBoundaryAir |  | 4 |  | 4 | 16 |  | 

| group | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | 350 | 2,392 | 494,199 | 49,191,064 | 1,945 | 372 | 375 | 352 | 459 | 339 | 18,806,033 | 46 | 97 | 

| group | air_name | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | AccessAdapterAir<2> | 65,536 |  | 16 | 11 | 1,769,472 | 
| verify_fibair | AccessAdapterAir<4> | 32,768 |  | 16 | 13 | 950,272 | 
| verify_fibair | AccessAdapterAir<8> | 128 |  | 16 | 17 | 4,224 | 
| verify_fibair | FriReducedOpeningAir | 512 |  | 76 | 64 | 71,680 | 
| verify_fibair | NativePoseidon2Air<BabyBearParameters>, 1> | 32 |  | 36 | 348 | 12,288 | 
| verify_fibair | PhantomAir | 16,384 |  | 8 | 6 | 229,376 | 
| verify_fibair | ProgramAir | 8,192 |  | 8 | 10 | 147,456 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| verify_fibair | VerifyBatchAir<BabyBearParameters>, 1> | 16,384 |  | 296 | 398 | 11,370,496 | 
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 131,072 |  | 28 | 23 | 6,684,672 | 
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 16,384 |  | 12 | 10 | 360,448 | 
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 262,144 |  | 20 | 30 | 13,107,200 | 
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 131,072 |  | 36 | 25 | 7,995,392 | 
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<4>, NativeLoadStoreCoreAir<4> | 16,384 |  | 36 | 34 | 1,146,880 | 
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 8,192 |  | 20 | 40 | 491,520 | 
| verify_fibair | VmConnectorAir | 2 | 1 | 8 | 4 | 24 | 
| verify_fibair | VolatileBoundaryAir | 131,072 |  | 8 | 11 | 2,490,368 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/6fb7d1524f38cc30b7363748d0b6a8c6d9a9cc10

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12799210811)
