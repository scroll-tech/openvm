| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+13 [+28.4%])</span> 57.37 | <span style='color: red'>(+13 [+28.4%])</span> 57.37 |
| ecrecover_program | <span style='color: red'>(+0 [+0.4%])</span> 2.64 | <span style='color: red'>(+0 [+0.4%])</span> 2.64 |
| leaf | <span style='color: red'>(+13 [+30.1%])</span> 54.74 | <span style='color: red'>(+13 [+30.1%])</span> 54.74 |


| ecrecover_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+10 [+0.4%])</span> 2,638 | <span style='color: red'>(+10 [+0.4%])</span> 2,638 | <span style='color: red'>(+10 [+0.4%])</span> 2,638 | <span style='color: red'>(+10 [+0.4%])</span> 2,638 |
| `main_cells_used     ` |  15,092,297 |  15,092,297 |  15,092,297 |  15,092,297 |
| `total_cycles        ` |  285,401 |  285,401 |  285,401 |  285,401 |
| `execute_time_ms     ` | <span style='color: green'>(-1 [-0.7%])</span> 148 | <span style='color: green'>(-1 [-0.7%])</span> 148 | <span style='color: green'>(-1 [-0.7%])</span> 148 | <span style='color: green'>(-1 [-0.7%])</span> 148 |
| `trace_gen_time_ms   ` | <span style='color: green'>(-1 [-0.4%])</span> 265 | <span style='color: green'>(-1 [-0.4%])</span> 265 | <span style='color: green'>(-1 [-0.4%])</span> 265 | <span style='color: green'>(-1 [-0.4%])</span> 265 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+12 [+0.5%])</span> 2,225 | <span style='color: red'>(+12 [+0.5%])</span> 2,225 | <span style='color: red'>(+12 [+0.5%])</span> 2,225 | <span style='color: red'>(+12 [+0.5%])</span> 2,225 |
| `main_trace_commit_time_ms` |  392 |  392 |  392 |  392 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-2 [-4.5%])</span> 42 | <span style='color: green'>(-2 [-4.5%])</span> 42 | <span style='color: green'>(-2 [-4.5%])</span> 42 | <span style='color: green'>(-2 [-4.5%])</span> 42 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-5 [-0.9%])</span> 525 | <span style='color: green'>(-5 [-0.9%])</span> 525 | <span style='color: green'>(-5 [-0.9%])</span> 525 | <span style='color: green'>(-5 [-0.9%])</span> 525 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-6 [-2.0%])</span> 297 | <span style='color: green'>(-6 [-2.0%])</span> 297 | <span style='color: green'>(-6 [-2.0%])</span> 297 | <span style='color: green'>(-6 [-2.0%])</span> 297 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+29 [+9.4%])</span> 336 | <span style='color: red'>(+29 [+9.4%])</span> 336 | <span style='color: red'>(+29 [+9.4%])</span> 336 | <span style='color: red'>(+29 [+9.4%])</span> 336 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-7 [-1.1%])</span> 628 | <span style='color: green'>(-7 [-1.1%])</span> 628 | <span style='color: green'>(-7 [-1.1%])</span> 628 | <span style='color: green'>(-7 [-1.1%])</span> 628 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+12664 [+30.1%])</span> 54,736 | <span style='color: red'>(+12664 [+30.1%])</span> 54,736 | <span style='color: red'>(+12664 [+30.1%])</span> 54,736 | <span style='color: red'>(+12664 [+30.1%])</span> 54,736 |
| `main_cells_used     ` | <span style='color: red'>(+24621088 [+5.6%])</span> 465,178,304 | <span style='color: red'>(+24621088 [+5.6%])</span> 465,178,304 | <span style='color: red'>(+24621088 [+5.6%])</span> 465,178,304 | <span style='color: red'>(+24621088 [+5.6%])</span> 465,178,304 |
| `total_cycles        ` | <span style='color: red'>(+1802923 [+18.7%])</span> 11,466,209 | <span style='color: red'>(+1802923 [+18.7%])</span> 11,466,209 | <span style='color: red'>(+1802923 [+18.7%])</span> 11,466,209 | <span style='color: red'>(+1802923 [+18.7%])</span> 11,466,209 |
| `execute_time_ms     ` | <span style='color: red'>(+296 [+10.9%])</span> 3,022 | <span style='color: red'>(+296 [+10.9%])</span> 3,022 | <span style='color: red'>(+296 [+10.9%])</span> 3,022 | <span style='color: red'>(+296 [+10.9%])</span> 3,022 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+892 [+12.6%])</span> 7,985 | <span style='color: red'>(+892 [+12.6%])</span> 7,985 | <span style='color: red'>(+892 [+12.6%])</span> 7,985 | <span style='color: red'>(+892 [+12.6%])</span> 7,985 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+11476 [+35.6%])</span> 43,729 | <span style='color: red'>(+11476 [+35.6%])</span> 43,729 | <span style='color: red'>(+11476 [+35.6%])</span> 43,729 | <span style='color: red'>(+11476 [+35.6%])</span> 43,729 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+1644 [+26.2%])</span> 7,914 | <span style='color: red'>(+1644 [+26.2%])</span> 7,914 | <span style='color: red'>(+1644 [+26.2%])</span> 7,914 | <span style='color: red'>(+1644 [+26.2%])</span> 7,914 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+139 [+18.7%])</span> 881 | <span style='color: red'>(+139 [+18.7%])</span> 881 | <span style='color: red'>(+139 [+18.7%])</span> 881 | <span style='color: red'>(+139 [+18.7%])</span> 881 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+1828 [+29.0%])</span> 8,123 | <span style='color: red'>(+1828 [+29.0%])</span> 8,123 | <span style='color: red'>(+1828 [+29.0%])</span> 8,123 | <span style='color: red'>(+1828 [+29.0%])</span> 8,123 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+2374 [+32.2%])</span> 9,750 | <span style='color: red'>(+2374 [+32.2%])</span> 9,750 | <span style='color: red'>(+2374 [+32.2%])</span> 9,750 | <span style='color: red'>(+2374 [+32.2%])</span> 9,750 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+2091 [+41.2%])</span> 7,162 | <span style='color: red'>(+2091 [+41.2%])</span> 7,162 | <span style='color: red'>(+2091 [+41.2%])</span> 7,162 | <span style='color: red'>(+2091 [+41.2%])</span> 7,162 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+3399 [+52.3%])</span> 9,896 | <span style='color: red'>(+3399 [+52.3%])</span> 9,896 | <span style='color: red'>(+3399 [+52.3%])</span> 9,896 | <span style='color: red'>(+3399 [+52.3%])</span> 9,896 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | keygen_time_ms | commit_exe_time_ms |
| --- | --- | --- | --- |
| ecrecover_program | 1 | 1,132 | 12 | 

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
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, EcDoubleCoreAir> | 2 | 411 | 513 | 
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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 4 | 15 | 24 | 
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
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 8,388,608 |  | 20 | 30 | 419,430,400 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 4,194,304 |  | 20 | 31 | 213,909,504 | 
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
| ecrecover_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | 8,192 |  | 44 | 18 | 507,904 | 
| ecrecover_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | 256 |  | 36 | 26 | 15,872 | 
| ecrecover_program | VmAirWrapper<Rv32IsEqualModAdapterAir<2, 1, 32, 32>, ModularIsEqualCoreAir<32, 4, 8> | 0 | 4,096 |  | 56 | 166 | 909,312 | 
| ecrecover_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | 8,192 |  | 36 | 28 | 524,288 | 
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | 0 | 4,096 |  | 76 | 35 | 454,656 | 
| ecrecover_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | 131,072 |  | 72 | 40 | 14,680,064 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | 0 | 8 |  | 100 | 39 | 1,112 | 
| ecrecover_program | VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | 0 | 4,096 |  | 80 | 31 | 454,656 | 
| ecrecover_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | 4,096 |  | 28 | 21 | 200,704 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, EcDoubleCoreAir> | 0 | 2,048 |  | 828 | 543 | 2,807,808 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | 16 |  | 192 | 199 | 6,256 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | 32 |  | 316 | 261 | 18,464 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 1,024 |  | 848 | 619 | 1,502,208 | 
| ecrecover_program | VmConnectorAir | 0 | 2 | 1 | 12 | 4 | 32 | 

| group | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | 0 | 7,985 | 54,736 | 11,466,209 | 1,220,217,304 | 43,729 | 9,750 | 7,162 | 8,123 | 9,896 | 7,914 | 465,178,304 | 881 | 3,022 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | 0 | 265 | 2,638 | 285,401 | 56,172,159 | 2,225 | 297 | 336 | 525 | 628 | 392 | 15,092,297 | 42 | 148 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/1d7e7e24acf4e4586a5158dbe63a31692b1e3e1d

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12685391144)
