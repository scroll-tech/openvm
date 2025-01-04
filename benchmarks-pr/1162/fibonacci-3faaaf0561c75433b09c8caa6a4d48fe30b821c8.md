| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+1124 [+4.4%])</span> 26,562 | <span style='color: red'>(+1124 [+4.4%])</span> 26,562 |
| fibonacci_program | <span style='color: red'>(+300 [+4.0%])</span> 7,714 | <span style='color: red'>(+300 [+4.0%])</span> 7,714 |
| leaf | <span style='color: red'>(+824 [+4.6%])</span> 18,848 | <span style='color: red'>(+824 [+4.6%])</span> 18,848 |


| fibonacci_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+300 [+4.0%])</span> 7,714 | <span style='color: red'>(+300 [+4.0%])</span> 7,714 | <span style='color: red'>(+300 [+4.0%])</span> 7,714 | <span style='color: red'>(+300 [+4.0%])</span> 7,714 |
| `main_cells_used     ` | <span style='color: green'>(-79730 [-0.2%])</span> 51,425,372 | <span style='color: green'>(-79730 [-0.2%])</span> 51,425,372 | <span style='color: green'>(-79730 [-0.2%])</span> 51,425,372 | <span style='color: green'>(-79730 [-0.2%])</span> 51,425,372 |
| `total_cycles        ` |  1,500,137 |  1,500,137 |  1,500,137 |  1,500,137 |
| `execute_time_ms     ` | <span style='color: green'>(-395 [-24.0%])</span> 1,250 | <span style='color: green'>(-395 [-24.0%])</span> 1,250 | <span style='color: green'>(-395 [-24.0%])</span> 1,250 | <span style='color: green'>(-395 [-24.0%])</span> 1,250 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+671 [+262.1%])</span> 927 | <span style='color: red'>(+671 [+262.1%])</span> 927 | <span style='color: red'>(+671 [+262.1%])</span> 927 | <span style='color: red'>(+671 [+262.1%])</span> 927 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+24 [+0.4%])</span> 5,537 | <span style='color: red'>(+24 [+0.4%])</span> 5,537 | <span style='color: red'>(+24 [+0.4%])</span> 5,537 | <span style='color: red'>(+24 [+0.4%])</span> 5,537 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-29 [-3.4%])</span> 827 | <span style='color: green'>(-29 [-3.4%])</span> 827 | <span style='color: green'>(-29 [-3.4%])</span> 827 | <span style='color: green'>(-29 [-3.4%])</span> 827 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+2 [+1.1%])</span> 180 | <span style='color: red'>(+2 [+1.1%])</span> 180 | <span style='color: red'>(+2 [+1.1%])</span> 180 | <span style='color: red'>(+2 [+1.1%])</span> 180 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+12 [+0.7%])</span> 1,757 | <span style='color: red'>(+12 [+0.7%])</span> 1,757 | <span style='color: red'>(+12 [+0.7%])</span> 1,757 | <span style='color: red'>(+12 [+0.7%])</span> 1,757 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+23 [+2.7%])</span> 866 | <span style='color: red'>(+23 [+2.7%])</span> 866 | <span style='color: red'>(+23 [+2.7%])</span> 866 | <span style='color: red'>(+23 [+2.7%])</span> 866 |
| `quotient_poly_commit_time_ms` |  494 |  494 |  494 |  494 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+15 [+1.1%])</span> 1,410 | <span style='color: red'>(+15 [+1.1%])</span> 1,410 | <span style='color: red'>(+15 [+1.1%])</span> 1,410 | <span style='color: red'>(+15 [+1.1%])</span> 1,410 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+824 [+4.6%])</span> 18,848 | <span style='color: red'>(+824 [+4.6%])</span> 18,848 | <span style='color: red'>(+824 [+4.6%])</span> 18,848 | <span style='color: red'>(+824 [+4.6%])</span> 18,848 |
| `main_cells_used     ` | <span style='color: green'>(-11681513 [-9.1%])</span> 117,180,053 | <span style='color: green'>(-11681513 [-9.1%])</span> 117,180,053 | <span style='color: green'>(-11681513 [-9.1%])</span> 117,180,053 | <span style='color: green'>(-11681513 [-9.1%])</span> 117,180,053 |
| `total_cycles        ` |  3,171,813 |  3,171,813 |  3,171,813 |  3,171,813 |
| `execute_time_ms     ` | <span style='color: green'>(-581 [-16.0%])</span> 3,044 | <span style='color: green'>(-581 [-16.0%])</span> 3,044 | <span style='color: green'>(-581 [-16.0%])</span> 3,044 | <span style='color: green'>(-581 [-16.0%])</span> 3,044 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+1692 [+250.7%])</span> 2,367 | <span style='color: red'>(+1692 [+250.7%])</span> 2,367 | <span style='color: red'>(+1692 [+250.7%])</span> 2,367 | <span style='color: red'>(+1692 [+250.7%])</span> 2,367 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-287 [-2.1%])</span> 13,437 | <span style='color: green'>(-287 [-2.1%])</span> 13,437 | <span style='color: green'>(-287 [-2.1%])</span> 13,437 | <span style='color: green'>(-287 [-2.1%])</span> 13,437 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-160 [-6.5%])</span> 2,299 | <span style='color: green'>(-160 [-6.5%])</span> 2,299 | <span style='color: green'>(-160 [-6.5%])</span> 2,299 | <span style='color: green'>(-160 [-6.5%])</span> 2,299 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+7 [+2.2%])</span> 331 | <span style='color: red'>(+7 [+2.2%])</span> 331 | <span style='color: red'>(+7 [+2.2%])</span> 331 | <span style='color: red'>(+7 [+2.2%])</span> 331 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-148 [-6.9%])</span> 1,990 | <span style='color: green'>(-148 [-6.9%])</span> 1,990 | <span style='color: green'>(-148 [-6.9%])</span> 1,990 | <span style='color: green'>(-148 [-6.9%])</span> 1,990 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+10 [+0.3%])</span> 3,354 | <span style='color: red'>(+10 [+0.3%])</span> 3,354 | <span style='color: red'>(+10 [+0.3%])</span> 3,354 | <span style='color: red'>(+10 [+0.3%])</span> 3,354 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-80 [-3.8%])</span> 2,029 | <span style='color: green'>(-80 [-3.8%])</span> 2,029 | <span style='color: green'>(-80 [-3.8%])</span> 2,029 | <span style='color: green'>(-80 [-3.8%])</span> 2,029 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+85 [+2.5%])</span> 3,432 | <span style='color: red'>(+85 [+2.5%])</span> 3,432 | <span style='color: red'>(+85 [+2.5%])</span> 3,432 | <span style='color: red'>(+85 [+2.5%])</span> 3,432 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | num_children | keygen_time_ms | fri.log_blowup | commit_exe_time_ms |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program | 1 |  | 353 | 2 | 6 | 
| leaf |  | 1 |  | 2 |  | 

| group | air_name | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| fibonacci_program | AccessAdapterAir<16> | 2 | 5 | 14 | 
| fibonacci_program | AccessAdapterAir<2> | 2 | 5 | 14 | 
| fibonacci_program | AccessAdapterAir<32> | 2 | 5 | 14 | 
| fibonacci_program | AccessAdapterAir<4> | 2 | 5 | 14 | 
| fibonacci_program | AccessAdapterAir<64> | 2 | 5 | 14 | 
| fibonacci_program | AccessAdapterAir<8> | 2 | 5 | 14 | 
| fibonacci_program | BitwiseOperationLookupAir<8> | 2 | 2 | 4 | 
| fibonacci_program | MemoryMerkleAir<8> | 2 | 4 | 40 | 
| fibonacci_program | PersistentBoundaryAir<8> | 2 | 3 | 6 | 
| fibonacci_program | PhantomAir | 2 | 3 | 5 | 
| fibonacci_program | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 2 | 1 | 286 | 
| fibonacci_program | ProgramAir | 1 | 1 | 4 | 
| fibonacci_program | RangeTupleCheckerAir<2> | 1 | 1 | 4 | 
| fibonacci_program | VariableRangeCheckerAir | 1 | 1 | 4 | 
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | 19 | 43 | 
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | 17 | 39 | 
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 2 | 23 | 90 | 
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | 11 | 25 | 
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 2 | 13 | 41 | 
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | 10 | 22 | 
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 2 | 15 | 17 | 
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 2 | 16 | 20 | 
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 2 | 18 | 33 | 
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 2 | 17 | 38 | 
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 2 | 25 | 88 | 
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 2 | 24 | 38 | 
| fibonacci_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 2 | 19 | 26 | 
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 2 | 11 | 15 | 
| fibonacci_program | VmConnectorAir | 2 | 3 | 9 | 
| leaf | AccessAdapterAir<2> | 4 | 5 | 12 | 
| leaf | AccessAdapterAir<4> | 4 | 5 | 12 | 
| leaf | AccessAdapterAir<8> | 4 | 5 | 12 | 
| leaf | FriReducedOpeningAir | 4 | 35 | 59 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 4 | 31 | 302 | 
| leaf | PhantomAir | 4 | 3 | 4 | 
| leaf | ProgramAir | 1 | 1 | 4 | 
| leaf | VariableRangeCheckerAir | 1 | 1 | 4 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 11 | 23 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 4 | 7 | 6 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 4 | 11 | 23 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 4 | 15 | 23 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 4 | 19 | 31 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 4 | 15 | 23 | 
| leaf | VmConnectorAir | 4 | 3 | 8 | 
| leaf | VolatileBoundaryAir | 4 | 4 | 16 | 

| group | air_name | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | AccessAdapterAir<2> | 0 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<4> | 0 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<8> | 0 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | FriReducedOpeningAir | 0 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | PhantomAir | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | ProgramAir | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 131,072 |  | 12 | 10 | 2,883,584 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 2,097,152 |  | 24 | 41 | 136,314,880 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 32,768 |  | 20 | 40 | 1,966,080 | 
| leaf | VmConnectorAir | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 0 | 524,288 |  | 8 | 11 | 9,961,472 | 

| group | air_name | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | AccessAdapterAir<8> | 0 | 64 |  | 24 | 17 | 2,624 | 
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| fibonacci_program | MemoryMerkleAir<8> | 0 | 512 |  | 20 | 32 | 26,624 | 
| fibonacci_program | PersistentBoundaryAir<8> | 0 | 64 |  | 12 | 20 | 2,048 | 
| fibonacci_program | PhantomAir | 0 | 2 |  | 12 | 6 | 36 | 
| fibonacci_program | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | 256 |  | 8 | 300 | 78,848 | 
| fibonacci_program | ProgramAir | 0 | 4,096 |  | 8 | 10 | 73,728 | 
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| fibonacci_program | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | 1,048,576 |  | 80 | 36 | 121,634,816 | 
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | 524,288 |  | 40 | 37 | 40,370,176 | 
| fibonacci_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | 2 |  | 52 | 53 | 210 | 
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | 262,144 |  | 48 | 26 | 19,398,656 | 
| fibonacci_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | 8 |  | 56 | 32 | 704 | 
| fibonacci_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | 131,072 |  | 44 | 18 | 8,126,464 | 
| fibonacci_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | 4 |  | 36 | 26 | 248 | 
| fibonacci_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | 16 |  | 36 | 28 | 1,024 | 
| fibonacci_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | 32 |  | 72 | 40 | 3,584 | 
| fibonacci_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | 16 |  | 28 | 21 | 784 | 
| fibonacci_program | VmConnectorAir | 0 | 2 | 1 | 12 | 4 | 32 | 

| group | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | 0 | 2,367 | 18,848 | 3,171,813 | 369,494,488 | 13,437 | 3,354 | 2,029 | 1,990 | 3,432 | 2,299 | 117,180,053 | 331 | 3,044 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 | 927 | 7,714 | 1,500,137 | 197,453,854 | 5,537 | 866 | 494 | 1,757 | 1,410 | 827 | 51,425,372 | 180 | 1,250 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/3faaaf0561c75433b09c8caa6a4d48fe30b821c8

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12612927935)
