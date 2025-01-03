| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+1490 [+2.9%])</span> 53,147 | <span style='color: red'>(+1490 [+2.9%])</span> 53,147 |
| ecrecover_program | <span style='color: red'>(+38 [+1.3%])</span> 2,960 | <span style='color: red'>(+38 [+1.3%])</span> 2,960 |
| leaf | <span style='color: red'>(+1452 [+3.0%])</span> 50,187 | <span style='color: red'>(+1452 [+3.0%])</span> 50,187 |


| ecrecover_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+38 [+1.3%])</span> 2,960 | <span style='color: red'>(+38 [+1.3%])</span> 2,960 | <span style='color: red'>(+38 [+1.3%])</span> 2,960 | <span style='color: red'>(+38 [+1.3%])</span> 2,960 |
| `main_cells_used     ` | <span style='color: green'>(-1846654 [-12.1%])</span> 13,383,383 | <span style='color: green'>(-1846654 [-12.1%])</span> 13,383,383 | <span style='color: green'>(-1846654 [-12.1%])</span> 13,383,383 | <span style='color: green'>(-1846654 [-12.1%])</span> 13,383,383 |
| `total_cycles        ` |  290,016 |  290,016 |  290,016 |  290,016 |
| `execute_time_ms     ` | <span style='color: green'>(-143 [-29.5%])</span> 342 | <span style='color: green'>(-143 [-29.5%])</span> 342 | <span style='color: green'>(-143 [-29.5%])</span> 342 | <span style='color: green'>(-143 [-29.5%])</span> 342 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+198 [+260.5%])</span> 274 | <span style='color: red'>(+198 [+260.5%])</span> 274 | <span style='color: red'>(+198 [+260.5%])</span> 274 | <span style='color: red'>(+198 [+260.5%])</span> 274 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-17 [-0.7%])</span> 2,344 | <span style='color: green'>(-17 [-0.7%])</span> 2,344 | <span style='color: green'>(-17 [-0.7%])</span> 2,344 | <span style='color: green'>(-17 [-0.7%])</span> 2,344 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-5 [-1.3%])</span> 393 | <span style='color: green'>(-5 [-1.3%])</span> 393 | <span style='color: green'>(-5 [-1.3%])</span> 393 | <span style='color: green'>(-5 [-1.3%])</span> 393 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-3 [-6.8%])</span> 41 | <span style='color: green'>(-3 [-6.8%])</span> 41 | <span style='color: green'>(-3 [-6.8%])</span> 41 | <span style='color: green'>(-3 [-6.8%])</span> 41 |
| `perm_trace_commit_time_ms` |  524 |  524 |  524 |  524 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+3 [+1.1%])</span> 275 | <span style='color: red'>(+3 [+1.1%])</span> 275 | <span style='color: red'>(+3 [+1.1%])</span> 275 | <span style='color: red'>(+3 [+1.1%])</span> 275 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-15 [-4.7%])</span> 302 | <span style='color: green'>(-15 [-4.7%])</span> 302 | <span style='color: green'>(-15 [-4.7%])</span> 302 | <span style='color: green'>(-15 [-4.7%])</span> 302 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+2 [+0.2%])</span> 805 | <span style='color: red'>(+2 [+0.2%])</span> 805 | <span style='color: red'>(+2 [+0.2%])</span> 805 | <span style='color: red'>(+2 [+0.2%])</span> 805 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+1452 [+3.0%])</span> 50,187 | <span style='color: red'>(+1452 [+3.0%])</span> 50,187 | <span style='color: red'>(+1452 [+3.0%])</span> 50,187 | <span style='color: red'>(+1452 [+3.0%])</span> 50,187 |
| `main_cells_used     ` | <span style='color: green'>(-45541984 [-10.2%])</span> 400,078,764 | <span style='color: green'>(-45541984 [-10.2%])</span> 400,078,764 | <span style='color: green'>(-45541984 [-10.2%])</span> 400,078,764 | <span style='color: green'>(-45541984 [-10.2%])</span> 400,078,764 |
| `total_cycles        ` |  9,777,679 |  9,777,679 |  9,777,679 |  9,777,679 |
| `execute_time_ms     ` | <span style='color: green'>(-2733 [-23.3%])</span> 8,979 | <span style='color: green'>(-2733 [-23.3%])</span> 8,979 | <span style='color: green'>(-2733 [-23.3%])</span> 8,979 | <span style='color: green'>(-2733 [-23.3%])</span> 8,979 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+5800 [+269.6%])</span> 7,951 | <span style='color: red'>(+5800 [+269.6%])</span> 7,951 | <span style='color: red'>(+5800 [+269.6%])</span> 7,951 | <span style='color: red'>(+5800 [+269.6%])</span> 7,951 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-1615 [-4.6%])</span> 33,257 | <span style='color: green'>(-1615 [-4.6%])</span> 33,257 | <span style='color: green'>(-1615 [-4.6%])</span> 33,257 | <span style='color: green'>(-1615 [-4.6%])</span> 33,257 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-401 [-6.4%])</span> 5,897 | <span style='color: green'>(-401 [-6.4%])</span> 5,897 | <span style='color: green'>(-401 [-6.4%])</span> 5,897 | <span style='color: green'>(-401 [-6.4%])</span> 5,897 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-1 [-0.1%])</span> 783 | <span style='color: green'>(-1 [-0.1%])</span> 783 | <span style='color: green'>(-1 [-0.1%])</span> 783 | <span style='color: green'>(-1 [-0.1%])</span> 783 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-348 [-5.6%])</span> 5,829 | <span style='color: green'>(-348 [-5.6%])</span> 5,829 | <span style='color: green'>(-348 [-5.6%])</span> 5,829 | <span style='color: green'>(-348 [-5.6%])</span> 5,829 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-485 [-6.0%])</span> 7,577 | <span style='color: green'>(-485 [-6.0%])</span> 7,577 | <span style='color: green'>(-485 [-6.0%])</span> 7,577 | <span style='color: green'>(-485 [-6.0%])</span> 7,577 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-369 [-7.2%])</span> 4,748 | <span style='color: green'>(-369 [-7.2%])</span> 4,748 | <span style='color: green'>(-369 [-7.2%])</span> 4,748 | <span style='color: green'>(-369 [-7.2%])</span> 4,748 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-11 [-0.1%])</span> 8,420 | <span style='color: green'>(-11 [-0.1%])</span> 8,420 | <span style='color: green'>(-11 [-0.1%])</span> 8,420 | <span style='color: green'>(-11 [-0.1%])</span> 8,420 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | num_children | keygen_time_ms | fri.log_blowup | commit_exe_time_ms |
| --- | --- | --- | --- | --- | --- |
| ecrecover_program | 1 |  | 1,169 | 2 | 7 | 
| leaf |  | 1 |  | 2 |  | 

| group | air_name | quotient_deg | interactions | constraints |
| --- | --- | --- | --- | --- |
| ecrecover_program | AccessAdapterAir<16> | 2 | 5 | 14 | 
| ecrecover_program | AccessAdapterAir<2> | 2 | 5 | 14 | 
| ecrecover_program | AccessAdapterAir<32> | 2 | 5 | 14 | 
| ecrecover_program | AccessAdapterAir<4> | 2 | 5 | 14 | 
| ecrecover_program | AccessAdapterAir<64> | 2 | 5 | 14 | 
| ecrecover_program | AccessAdapterAir<8> | 2 | 5 | 14 | 
| ecrecover_program | BitwiseOperationLookupAir<8> | 2 | 2 | 4 | 
| ecrecover_program | KeccakVmAir | 2 | 321 | 4,571 | 
| ecrecover_program | MemoryMerkleAir<8> | 2 | 4 | 40 | 
| ecrecover_program | PersistentBoundaryAir<8> | 2 | 3 | 6 | 
| ecrecover_program | PhantomAir | 2 | 3 | 5 | 
| ecrecover_program | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 2 | 1 | 286 | 
| ecrecover_program | ProgramAir | 1 | 1 | 4 | 
| ecrecover_program | RangeTupleCheckerAir<2> | 1 | 1 | 4 | 
| ecrecover_program | VariableRangeCheckerAir | 1 | 1 | 4 | 
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | 19 | 43 | 
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | 17 | 39 | 
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 2 | 23 | 90 | 
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | 11 | 25 | 
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 2 | 13 | 41 | 
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | 10 | 22 | 
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 2 | 15 | 17 | 
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 2 | 25 | 223 | 
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 2 | 16 | 20 | 
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 2 | 18 | 33 | 
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 2 | 17 | 38 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 2 | 25 | 88 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 2 | 24 | 38 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 2 | 19 | 26 | 
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 2 | 11 | 15 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 411 | 449 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 2 | 94 | 126 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 2 | 156 | 188 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 2 | 422 | 456 | 
| ecrecover_program | VmConnectorAir | 2 | 3 | 9 | 
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
| leaf | AccessAdapterAir<2> | 0 | 2,097,152 |  | 16 | 11 | 56,623,104 | 
| leaf | AccessAdapterAir<4> | 0 | 1,048,576 |  | 16 | 13 | 30,408,704 | 
| leaf | AccessAdapterAir<8> | 0 | 262,144 |  | 16 | 17 | 8,650,752 | 
| leaf | FriReducedOpeningAir | 0 | 1,048,576 |  | 76 | 64 | 146,800,640 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 131,072 |  | 36 | 348 | 50,331,648 | 
| leaf | PhantomAir | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | ProgramAir | 0 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 4,194,304 |  | 28 | 23 | 213,909,504 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 131,072 |  | 12 | 10 | 2,883,584 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 4,194,304 |  | 20 | 30 | 209,715,200 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 4,194,304 |  | 24 | 41 | 272,629,760 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 262,144 |  | 20 | 40 | 15,728,640 | 
| leaf | VmConnectorAir | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 0 | 2,097,152 |  | 8 | 11 | 39,845,888 | 

| group | air_name | segment | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | AccessAdapterAir<16> | 0 | 16,384 |  | 24 | 25 | 802,816 | 
| ecrecover_program | AccessAdapterAir<2> | 0 | 256 |  | 24 | 11 | 8,960 | 
| ecrecover_program | AccessAdapterAir<32> | 0 | 8,192 |  | 24 | 41 | 532,480 | 
| ecrecover_program | AccessAdapterAir<4> | 0 | 128 |  | 24 | 13 | 4,736 | 
| ecrecover_program | AccessAdapterAir<8> | 0 | 32,768 |  | 24 | 17 | 1,343,488 | 
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | 65,536 | 3 | 8 | 2 | 655,360 | 
| ecrecover_program | KeccakVmAir | 0 | 128 |  | 1,288 | 3,164 | 569,856 | 
| ecrecover_program | MemoryMerkleAir<8> | 0 | 4,096 |  | 20 | 32 | 212,992 | 
| ecrecover_program | PersistentBoundaryAir<8> | 0 | 4,096 |  | 12 | 20 | 131,072 | 
| ecrecover_program | PhantomAir | 0 | 64 |  | 12 | 6 | 1,152 | 
| ecrecover_program | Poseidon2PeripheryAir<BabyBearParameters>, 1> | 0 | 4,096 |  | 8 | 300 | 1,261,568 | 
| ecrecover_program | ProgramAir | 0 | 16,384 |  | 8 | 10 | 294,912 | 
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | 524,288 | 2 | 8 | 1 | 4,718,592 | 
| ecrecover_program | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | 131,072 |  | 80 | 36 | 15,204,352 | 
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | 2,048 |  | 40 | 37 | 157,696 | 
| ecrecover_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | 16,384 |  | 52 | 53 | 1,720,320 | 
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | 16,384 |  | 48 | 26 | 1,212,416 | 
| ecrecover_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | 32,768 |  | 56 | 32 | 2,883,584 | 
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | 4,096 |  | 44 | 18 | 253,952 | 
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | 256 |  | 36 | 26 | 15,872 | 
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | 4,096 |  | 56 | 166 | 909,312 | 
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | 8,192 |  | 36 | 28 | 524,288 | 
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | 4,096 |  | 76 | 35 | 454,656 | 
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | 131,072 |  | 72 | 40 | 14,680,064 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | 0 | 8 |  | 104 | 57 | 1,288 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | 8 |  | 100 | 39 | 1,112 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | 4,096 |  | 80 | 31 | 454,656 | 
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | 4,096 |  | 28 | 21 | 200,704 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 2,048 |  | 828 | 543 | 2,807,808 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | 8 |  | 192 | 199 | 3,128 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | 16 |  | 316 | 261 | 9,232 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 1,024 |  | 848 | 619 | 1,502,208 | 
| ecrecover_program | VmConnectorAir | 0 | 2 | 1 | 12 | 4 | 32 | 

| group | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | 0 | 7,951 | 50,187 | 9,777,679 | 1,069,222,360 | 33,257 | 7,577 | 4,748 | 5,829 | 8,420 | 5,897 | 400,078,764 | 783 | 8,979 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | 0 | 274 | 2,960 | 290,016 | 55,907,135 | 2,344 | 275 | 302 | 524 | 805 | 393 | 13,383,383 | 41 | 342 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/3faaaf0561c75433b09c8caa6a4d48fe30b821c8

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12605411083)
