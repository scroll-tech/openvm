| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: green'>(-0 [-1.9%])</span> 1.41 | <span style='color: green'>(-0 [-1.9%])</span> 1.41 |
| verify_fibair | <span style='color: green'>(-0 [-1.9%])</span> 1.41 | <span style='color: green'>(-0 [-1.9%])</span> 1.41 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: green'>(-27 [-1.9%])</span> 1,408 | <span style='color: green'>(-27 [-1.9%])</span> 1,408 | <span style='color: green'>(-27 [-1.9%])</span> 1,408 | <span style='color: green'>(-27 [-1.9%])</span> 1,408 |
| `main_cells_used     ` |  8,026,812 |  8,026,812 |  8,026,812 |  8,026,812 |
| `total_cycles        ` |  195,126 |  195,126 |  195,126 |  195,126 |
| `execute_time_ms     ` | <span style='color: green'>(-14 [-23.7%])</span> 45 | <span style='color: green'>(-14 [-23.7%])</span> 45 | <span style='color: green'>(-14 [-23.7%])</span> 45 | <span style='color: green'>(-14 [-23.7%])</span> 45 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-2 [-1.5%])</span> 134 | <span style='color: green'>(-2 [-1.5%])</span> 134 | <span style='color: green'>(-2 [-1.5%])</span> 134 | <span style='color: green'>(-2 [-1.5%])</span> 134 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-11 [-0.9%])</span> 1,229 | <span style='color: green'>(-11 [-0.9%])</span> 1,229 | <span style='color: green'>(-11 [-0.9%])</span> 1,229 | <span style='color: green'>(-11 [-0.9%])</span> 1,229 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-13 [-6.2%])</span> 198 | <span style='color: green'>(-13 [-6.2%])</span> 198 | <span style='color: green'>(-13 [-6.2%])</span> 198 | <span style='color: green'>(-13 [-6.2%])</span> 198 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+2 [+8.7%])</span> 25 | <span style='color: red'>(+2 [+8.7%])</span> 25 | <span style='color: red'>(+2 [+8.7%])</span> 25 | <span style='color: red'>(+2 [+8.7%])</span> 25 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+3 [+1.7%])</span> 184 | <span style='color: red'>(+3 [+1.7%])</span> 184 | <span style='color: red'>(+3 [+1.7%])</span> 184 | <span style='color: red'>(+3 [+1.7%])</span> 184 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-4 [-1.8%])</span> 219 | <span style='color: green'>(-4 [-1.8%])</span> 219 | <span style='color: green'>(-4 [-1.8%])</span> 219 | <span style='color: green'>(-4 [-1.8%])</span> 219 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-7 [-2.8%])</span> 244 | <span style='color: green'>(-7 [-2.8%])</span> 244 | <span style='color: green'>(-7 [-2.8%])</span> 244 | <span style='color: green'>(-7 [-2.8%])</span> 244 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+10 [+2.9%])</span> 357 | <span style='color: red'>(+10 [+2.9%])</span> 357 | <span style='color: red'>(+10 [+2.9%])</span> 357 | <span style='color: red'>(+10 [+2.9%])</span> 357 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 4 | 32 | 10 | 0 | 1 | 0 | 3 | 5 | 

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
| verify_fibair | 134 | 1,408 | 195,126 | 23,304,216 | 1,229 | 219 | 244 | 184 | 357 | 198 | 8,026,812 | 25 | 45 | 

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


Commit: https://github.com/openvm-org/openvm/commit/54eaefcab3f708675cb12d2020fdf8313825ae1c

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12657489653)
