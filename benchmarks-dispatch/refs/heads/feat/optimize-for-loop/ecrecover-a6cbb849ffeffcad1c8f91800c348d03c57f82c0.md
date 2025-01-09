| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+223 [+499.1%])</span> 267.80 | <span style='color: red'>(+223 [+499.1%])</span> 267.80 |
| ecrecover_program | <span style='color: red'>(+2 [+74.0%])</span> 4.57 | <span style='color: red'>(+2 [+74.0%])</span> 4.57 |
| leaf | <span style='color: red'>(+221 [+525.7%])</span> 263.23 | <span style='color: red'>(+221 [+525.7%])</span> 263.23 |


| ecrecover_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+1945 [+74.0%])</span> 4,573 | <span style='color: red'>(+1945 [+74.0%])</span> 4,573 | <span style='color: red'>(+1945 [+74.0%])</span> 4,573 | <span style='color: red'>(+1945 [+74.0%])</span> 4,573 |
| `main_cells_used     ` | <span style='color: red'>(+155632 [+1.0%])</span> 15,247,929 | <span style='color: red'>(+155632 [+1.0%])</span> 15,247,929 | <span style='color: red'>(+155632 [+1.0%])</span> 15,247,929 | <span style='color: red'>(+155632 [+1.0%])</span> 15,247,929 |
| `total_cycles        ` | <span style='color: red'>(+4847 [+1.7%])</span> 290,248 | <span style='color: red'>(+4847 [+1.7%])</span> 290,248 | <span style='color: red'>(+4847 [+1.7%])</span> 290,248 | <span style='color: red'>(+4847 [+1.7%])</span> 290,248 |
| `execute_time_ms     ` | <span style='color: red'>(+1949 [+1308.1%])</span> 2,098 | <span style='color: red'>(+1949 [+1308.1%])</span> 2,098 | <span style='color: red'>(+1949 [+1308.1%])</span> 2,098 | <span style='color: red'>(+1949 [+1308.1%])</span> 2,098 |
| `trace_gen_time_ms   ` |  266 |  266 |  266 |  266 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-4 [-0.2%])</span> 2,209 | <span style='color: green'>(-4 [-0.2%])</span> 2,209 | <span style='color: green'>(-4 [-0.2%])</span> 2,209 | <span style='color: green'>(-4 [-0.2%])</span> 2,209 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-3 [-0.8%])</span> 389 | <span style='color: green'>(-3 [-0.8%])</span> 389 | <span style='color: green'>(-3 [-0.8%])</span> 389 | <span style='color: green'>(-3 [-0.8%])</span> 389 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-3 [-6.8%])</span> 41 | <span style='color: green'>(-3 [-6.8%])</span> 41 | <span style='color: green'>(-3 [-6.8%])</span> 41 | <span style='color: green'>(-3 [-6.8%])</span> 41 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+5 [+0.9%])</span> 535 | <span style='color: red'>(+5 [+0.9%])</span> 535 | <span style='color: red'>(+5 [+0.9%])</span> 535 | <span style='color: red'>(+5 [+0.9%])</span> 535 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-10 [-3.3%])</span> 293 | <span style='color: green'>(-10 [-3.3%])</span> 293 | <span style='color: green'>(-10 [-3.3%])</span> 293 | <span style='color: green'>(-10 [-3.3%])</span> 293 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+5 [+1.6%])</span> 312 | <span style='color: red'>(+5 [+1.6%])</span> 312 | <span style='color: red'>(+5 [+1.6%])</span> 312 | <span style='color: red'>(+5 [+1.6%])</span> 312 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+2 [+0.3%])</span> 637 | <span style='color: red'>(+2 [+0.3%])</span> 637 | <span style='color: red'>(+2 [+0.3%])</span> 637 | <span style='color: red'>(+2 [+0.3%])</span> 637 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+221157 [+525.7%])</span> 263,229 | <span style='color: red'>(+221157 [+525.7%])</span> 263,229 | <span style='color: red'>(+221157 [+525.7%])</span> 263,229 | <span style='color: red'>(+221157 [+525.7%])</span> 263,229 |
| `main_cells_used     ` | <span style='color: red'>(+955376325 [+216.9%])</span> 1,395,933,541 | <span style='color: red'>(+955376325 [+216.9%])</span> 1,395,933,541 | <span style='color: red'>(+955376325 [+216.9%])</span> 1,395,933,541 | <span style='color: red'>(+955376325 [+216.9%])</span> 1,395,933,541 |
| `total_cycles        ` | <span style='color: red'>(+26976181 [+279.2%])</span> 36,639,467 | <span style='color: red'>(+26976181 [+279.2%])</span> 36,639,467 | <span style='color: red'>(+26976181 [+279.2%])</span> 36,639,467 | <span style='color: red'>(+26976181 [+279.2%])</span> 36,639,467 |
| `execute_time_ms     ` | <span style='color: red'>(+123391 [+4526.4%])</span> 126,117 | <span style='color: red'>(+123391 [+4526.4%])</span> 126,117 | <span style='color: red'>(+123391 [+4526.4%])</span> 126,117 | <span style='color: red'>(+123391 [+4526.4%])</span> 126,117 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+13231 [+186.5%])</span> 20,324 | <span style='color: red'>(+13231 [+186.5%])</span> 20,324 | <span style='color: red'>(+13231 [+186.5%])</span> 20,324 | <span style='color: red'>(+13231 [+186.5%])</span> 20,324 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+84535 [+262.1%])</span> 116,788 | <span style='color: red'>(+84535 [+262.1%])</span> 116,788 | <span style='color: red'>(+84535 [+262.1%])</span> 116,788 | <span style='color: red'>(+84535 [+262.1%])</span> 116,788 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+13272 [+211.7%])</span> 19,542 | <span style='color: red'>(+13272 [+211.7%])</span> 19,542 | <span style='color: red'>(+13272 [+211.7%])</span> 19,542 | <span style='color: red'>(+13272 [+211.7%])</span> 19,542 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+1514 [+204.0%])</span> 2,256 | <span style='color: red'>(+1514 [+204.0%])</span> 2,256 | <span style='color: red'>(+1514 [+204.0%])</span> 2,256 | <span style='color: red'>(+1514 [+204.0%])</span> 2,256 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+12401 [+197.0%])</span> 18,696 | <span style='color: red'>(+12401 [+197.0%])</span> 18,696 | <span style='color: red'>(+12401 [+197.0%])</span> 18,696 | <span style='color: red'>(+12401 [+197.0%])</span> 18,696 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+19807 [+268.5%])</span> 27,183 | <span style='color: red'>(+19807 [+268.5%])</span> 27,183 | <span style='color: red'>(+19807 [+268.5%])</span> 27,183 | <span style='color: red'>(+19807 [+268.5%])</span> 27,183 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+16142 [+318.3%])</span> 21,213 | <span style='color: red'>(+16142 [+318.3%])</span> 21,213 | <span style='color: red'>(+16142 [+318.3%])</span> 21,213 | <span style='color: red'>(+16142 [+318.3%])</span> 21,213 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+21398 [+329.4%])</span> 27,895 | <span style='color: red'>(+21398 [+329.4%])</span> 27,895 | <span style='color: red'>(+21398 [+329.4%])</span> 27,895 | <span style='color: red'>(+21398 [+329.4%])</span> 27,895 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | keygen_time_ms | commit_exe_time_ms |
| --- | --- | --- | --- |
| ecrecover_program | 1 | 1,180 | 11 | 

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

| group | air_name | dsl_ir | idx | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | BNE | 92 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | BNE | 27,991 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | BNE | 3,519 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | BEQ | 23 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | BNE | 17,230,404 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | BNE | 20,262,402 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | BNE | 40,533,659 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | BEQ | 20,262,448 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | BEQ | 4,255 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | ZipFor | 0 | BNE | 5,615,726 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | 10 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | For | 0 | JAL | 253,310 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | JAL | 20 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | ZipFor | 0 | JAL | 15,680 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | ADD | 52,953,720 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | ADD | 52,867,530 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | ADD | 68,670 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | ADD | 133,976,340 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | ADD | 1,559,040 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | MUL | 811,830 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | ADD | 21,714,510 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | ADD | 26,447,730 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | MUL | 13,410 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | 0 | ADD | 30 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | ADD | 178,290 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | MUL | 174,900 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | MUL | 4,440 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | MUL | 5,280 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | MUL | 1,110 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | MUL | 80,220 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | ADD | 52,883,310 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | MUL | 17,400 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | ADD | 21,673,260 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | ADD | 68,220 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | MUL | 63,870 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | SUB | 1,110 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | SUB | 26,429,280 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | 0 | ADD | 1,110 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | ZipFor | 0 | ADD | 13,884,990 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | 31 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | 0 | LOADW | 27,310,256 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | 0 | STOREW | 81,930,768 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | 0 | LOADW | 1,611,008 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | 0 | LOADW | 2,418 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | 0 | STOREW | 782,843 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | 0 | STOREW | 124 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | 0 | STOREW | 5,270 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | 0 | STOREW | 95,623,003 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | 0 | LOADW | 108,128 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | 0 | LOADW | 81,950,639 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | 0 | LOADW | 14,726,271 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | 0 | STOREW | 109,241,024 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | 0 | STOREW | 27,422,724 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | 0 | STOREW | 54,650,024 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | 0 | SHINTW | 23,167,819 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | 0 | STOREW | 1,510,847 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ZipFor | 0 | LOADW | 48,639 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | FE4ADD | 26,430,760 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | BBE4MUL | 35,239,040 | 
| leaf | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 76,655,352 | 
| leaf | PhantomAir | CT-InitializePcsConst | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-ReadProofsFromInput | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-VerifyProofs | 0 | PHANTOM | 6 | 
| leaf | PhantomAir | CT-stage-c-build-rounds | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-stage-d-verifier-verify | 0 | PHANTOM | 6 | 
| leaf | PhantomAir | CT-stage-d-verify-pcs | 0 | PHANTOM | 6 | 
| leaf | PhantomAir | HintInputVec | 0 | PHANTOM | 149,442 | 

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | 2,645,532 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | 559,512 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | 250,740 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | 318,600 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | 900 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | 74,407 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | 228,536 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | 0 | 238,023 | 
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | 275,912 | 
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | 124,202 | 
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | 29,600 | 
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | 0 | 384 | 
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | 719,648 | 
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | 22,734 | 
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | 50,292 | 
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | 5,564 | 
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | IS_EQ | 0 | 531,698 | 
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> |  | SETUP_ISEQ | 0 | 332 | 
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | 186,060 | 
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> |  | LOADB | 0 | 132,300 | 
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | 0 | 98,000 | 
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | 553,840 | 
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | 0 | 1,037,520 | 
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | 2,702,880 | 
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> |  | DIVU | 0 | 285 | 
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> |  | MULHU | 0 | 195 | 
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> |  | MUL | 0 | 79,329 | 
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | 71,022 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,EcDoubleCoreAir> |  | EcDouble | 0 | 690,153 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> |  | ModularAddSub | 0 | 2,388 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> |  | ModularMulDiv | 0 | 8,352 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> |  | EcAddNe | 0 | 449,394 | 
| ecrecover_program | KeccakVmAir |  | KECCAK256 | 0 | 379,680 | 
| ecrecover_program | PhantomAir |  | PHANTOM | 0 | 270 | 

| group | air_name | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | AccessAdapterAir<2> | 0 | 16,777,216 |  | 16 | 11 | 452,984,832 | 
| leaf | AccessAdapterAir<4> | 0 | 8,388,608 |  | 16 | 13 | 243,269,632 | 
| leaf | AccessAdapterAir<8> | 0 | 524,288 |  | 16 | 17 | 17,301,504 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 262,144 |  | 36 | 348 | 100,663,296 | 
| leaf | PhantomAir | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | ProgramAir | 0 | 1,048,576 |  | 8 | 10 | 18,874,368 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 8,388,608 |  | 28 | 23 | 427,819,008 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 32,768 |  | 12 | 10 | 720,896 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 16,777,216 |  | 20 | 30 | 838,860,800 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 16,777,216 |  | 20 | 31 | 855,638,016 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 2,097,152 |  | 20 | 40 | 125,829,120 | 
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
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>, EcDoubleCoreAir> | 0 | 2,048 |  | 828 | 543 | 2,807,808 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularAddSubCoreAir> | 0 | 16 |  | 192 | 199 | 6,256 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>, ModularMulDivCoreAir> | 0 | 32 |  | 316 | 261 | 18,464 | 
| ecrecover_program | VmAirWrapper<Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>, FieldExpressionCoreAir> | 0 | 1,024 |  | 848 | 619 | 1,502,208 | 
| ecrecover_program | VmConnectorAir | 0 | 2 | 1 | 12 | 4 | 32 | 

| group | chip_name | idx | rows_used |
| --- | --- | --- | --- |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 4,519,153 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | 0 | 26,902 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 13,529,320 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 16,777,157 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1,541,745 | 
| leaf | AccessAdapter<2> | 0 | 11,453,154 | 
| leaf | AccessAdapter<4> | 0 | 5,726,578 | 
| leaf | AccessAdapter<8> | 0 | 440,550 | 
| leaf | Arc<BabyBearParameters>, 1> | 0 | 220,274 | 
| leaf | Boundary | 0 | 1,682,041 | 
| leaf | PhantomAir | 0 | 24,916 | 
| leaf | ProgramChip | 0 | 593,726 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 
| leaf | VmConnectorAir | 0 | 2 | 

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| ecrecover_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | 104,869 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | 2,011 | 
| ecrecover_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | 8,803 | 
| ecrecover_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | 15,389 | 
| ecrecover_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | 23,426 | 
| ecrecover_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | 4,057 | 
| ecrecover_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | 214 | 
| ecrecover_program | <Rv32IsEqualModAdapterAir<2, 1, 32, 32>,ModularIsEqualCoreAir<32, 4, 8>> | 0 | 3,194 | 
| ecrecover_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | 6,645 | 
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadSignExtendCoreAir<4, 8>> | 0 | 3,780 | 
| ecrecover_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | 109,806 | 
| ecrecover_program | <Rv32MultAdapterAir,DivRemCoreAir<4, 8>> | 0 | 5 | 
| ecrecover_program | <Rv32MultAdapterAir,MulHCoreAir<4, 8>> | 0 | 5 | 
| ecrecover_program | <Rv32MultAdapterAir,MultiplicationCoreAir<4, 8>> | 0 | 2,559 | 
| ecrecover_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | 3,383 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<1, 2, 2, 32, 32>,EcDoubleCoreAir> | 0 | 1,271 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularAddSubCoreAir> | 0 | 11 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 1, 1, 32, 32>,ModularMulDivCoreAir> | 0 | 21 | 
| ecrecover_program | <Rv32VecHeapAdapterAir<2, 2, 2, 32, 32>,FieldExpressionCoreAir> | 0 | 726 | 
| ecrecover_program | AccessAdapter<16> | 0 | 13,306 | 
| ecrecover_program | AccessAdapter<2> | 0 | 132 | 
| ecrecover_program | AccessAdapter<32> | 0 | 6,654 | 
| ecrecover_program | AccessAdapter<4> | 0 | 68 | 
| ecrecover_program | AccessAdapter<8> | 0 | 27,216 | 
| ecrecover_program | Arc<BabyBearParameters>, 1> | 0 | 2,060 | 
| ecrecover_program | BitwiseOperationLookupAir<8> | 0 | 65,536 | 
| ecrecover_program | Boundary | 0 | 2,990 | 
| ecrecover_program | KeccakVmAir | 0 | 120 | 
| ecrecover_program | Merkle | 0 | 3,288 | 
| ecrecover_program | PhantomAir | 0 | 45 | 
| ecrecover_program | ProgramChip | 0 | 8,624 | 
| ecrecover_program | RangeTupleCheckerAir<2> | 0 | 524,288 | 
| ecrecover_program | VariableRangeCheckerAir | 0 | 262,144 | 
| ecrecover_program | VmConnectorAir | 0 | 2 | 

| group | dsl_ir | idx | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf |  | 0 | JAL | 1 | 
| leaf |  | 0 | STOREW | 2 | 
| leaf | AddE | 0 | FE4ADD | 660,769 | 
| leaf | AddEFFI | 0 | LOADW | 880,976 | 
| leaf | AddEFFI | 0 | STOREW | 2,642,928 | 
| leaf | AddEI | 0 | ADD | 1,765,124 | 
| leaf | AddFI | 0 | ADD | 1,762,251 | 
| leaf | AddV | 0 | ADD | 2,289 | 
| leaf | AddVI | 0 | ADD | 4,465,878 | 
| leaf | Alloc | 0 | ADD | 51,968 | 
| leaf | Alloc | 0 | LOADW | 51,968 | 
| leaf | Alloc | 0 | MUL | 27,061 | 
| leaf | AssertEqEI | 0 | BNE | 4 | 
| leaf | AssertEqV | 0 | BNE | 1,217 | 
| leaf | AssertEqVI | 0 | BNE | 153 | 
| leaf | AssertNeVI | 0 | BEQ | 1 | 
| leaf | CT-InitializePcsConst | 0 | PHANTOM | 2 | 
| leaf | CT-ReadProofsFromInput | 0 | PHANTOM | 2 | 
| leaf | CT-VerifyProofs | 0 | PHANTOM | 1 | 
| leaf | CT-stage-c-build-rounds | 0 | PHANTOM | 2 | 
| leaf | CT-stage-d-verifier-verify | 0 | PHANTOM | 1 | 
| leaf | CT-stage-d-verify-pcs | 0 | PHANTOM | 1 | 
| leaf | For | 0 | ADD | 723,817 | 
| leaf | For | 0 | BNE | 749,148 | 
| leaf | For | 0 | JAL | 25,331 | 
| leaf | For | 0 | LOADW | 78 | 
| leaf | For | 0 | STOREW | 25,253 | 
| leaf | HintInputVec | 0 | PHANTOM | 24,907 | 
| leaf | IfEq | 0 | BNE | 880,974 | 
| leaf | IfEqI | 0 | BNE | 1,762,333 | 
| leaf | IfNe | 0 | BEQ | 880,976 | 
| leaf | IfNe | 0 | JAL | 2 | 
| leaf | IfNeI | 0 | BEQ | 185 | 
| leaf | ImmE | 0 | STOREW | 4 | 
| leaf | ImmF | 0 | STOREW | 170 | 
| leaf | ImmV | 0 | STOREW | 3,084,613 | 
| leaf | LoadE | 0 | LOADW | 3,488 | 
| leaf | LoadF | 0 | ADD | 881,591 | 
| leaf | LoadF | 0 | LOADW | 2,643,569 | 
| leaf | LoadF | 0 | MUL | 447 | 
| leaf | LoadHeapPtr | 0 | ADD | 1 | 
| leaf | LoadV | 0 | ADD | 5,943 | 
| leaf | LoadV | 0 | LOADW | 475,041 | 
| leaf | LoadV | 0 | MUL | 5,830 | 
| leaf | MulEF | 0 | MUL | 148 | 
| leaf | MulEI | 0 | BBE4MUL | 880,976 | 
| leaf | MulEI | 0 | STOREW | 3,523,904 | 
| leaf | MulF | 0 | MUL | 176 | 
| leaf | MulFI | 0 | MUL | 37 | 
| leaf | MulVI | 0 | MUL | 2,674 | 
| leaf | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 220,274 | 
| leaf | StoreE | 0 | STOREW | 884,604 | 
| leaf | StoreF | 0 | ADD | 1,762,777 | 
| leaf | StoreF | 0 | MUL | 580 | 
| leaf | StoreF | 0 | STOREW | 1,762,904 | 
| leaf | StoreHintWord | 0 | ADD | 722,442 | 
| leaf | StoreHintWord | 0 | SHINTW | 747,349 | 
| leaf | StoreV | 0 | ADD | 2,274 | 
| leaf | StoreV | 0 | MUL | 2,129 | 
| leaf | StoreV | 0 | STOREW | 48,737 | 
| leaf | SubV | 0 | SUB | 37 | 
| leaf | SubVI | 0 | SUB | 880,976 | 
| leaf | UnsafeCastVF | 0 | ADD | 37 | 
| leaf | ZipFor | 0 | ADD | 462,833 | 
| leaf | ZipFor | 0 | BNE | 244,162 | 
| leaf | ZipFor | 0 | JAL | 1,568 | 
| leaf | ZipFor | 0 | LOADW | 1,569 | 

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| ecrecover_program |  | ADD | 0 | 73,487 | 
| ecrecover_program |  | AND | 0 | 15,542 | 
| ecrecover_program |  | AUIPC | 0 | 3,383 | 
| ecrecover_program |  | BEQ | 0 | 10,612 | 
| ecrecover_program |  | BGEU | 0 | 925 | 
| ecrecover_program |  | BLT | 0 | 12 | 
| ecrecover_program |  | BLTU | 0 | 22,489 | 
| ecrecover_program |  | BNE | 0 | 4,777 | 
| ecrecover_program |  | DIVU | 0 | 5 | 
| ecrecover_program |  | EcAddNe | 0 | 726 | 
| ecrecover_program |  | EcDouble | 0 | 1,271 | 
| ecrecover_program |  | HINT_STOREW | 0 | 214 | 
| ecrecover_program |  | IS_EQ | 0 | 3,203 | 
| ecrecover_program |  | JAL | 0 | 1,263 | 
| ecrecover_program |  | JALR | 0 | 6,645 | 
| ecrecover_program |  | KECCAK256 | 0 | 5 | 
| ecrecover_program |  | LOADB | 0 | 3,780 | 
| ecrecover_program |  | LOADBU | 0 | 2,450 | 
| ecrecover_program |  | LOADW | 0 | 13,846 | 
| ecrecover_program |  | LUI | 0 | 2,794 | 
| ecrecover_program |  | MUL | 0 | 2,559 | 
| ecrecover_program |  | MULHU | 0 | 5 | 
| ecrecover_program |  | ModularAddSub | 0 | 12 | 
| ecrecover_program |  | ModularMulDiv | 0 | 32 | 
| ecrecover_program |  | OR | 0 | 6,965 | 
| ecrecover_program |  | PHANTOM | 0 | 45 | 
| ecrecover_program |  | SETUP_ISEQ | 0 | 2 | 
| ecrecover_program |  | SLL | 0 | 4,312 | 
| ecrecover_program |  | SLTU | 0 | 2,011 | 
| ecrecover_program |  | SRL | 0 | 4,491 | 
| ecrecover_program |  | STOREB | 0 | 25,938 | 
| ecrecover_program |  | STOREW | 0 | 67,572 | 
| ecrecover_program |  | SUB | 0 | 8,850 | 
| ecrecover_program |  | XOR | 0 | 25 | 

| group | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | 0 | 20,324 | 263,229 | 36,639,467 | 3,124,625,432 | 116,788 | 27,183 | 21,213 | 18,696 | 27,895 | 19,542 | 1,395,933,541 | 2,256 | 126,117 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ecrecover_program | 0 | 266 | 4,573 | 290,248 | 55,919,495 | 2,209 | 293 | 312 | 535 | 637 | 389 | 15,247,929 | 41 | 2,098 | 

</details>


<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-ecrecover_program.dsl_ir.opcode.frequency.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/ecrecover-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/a6cbb849ffeffcad1c8f91800c348d03c57f82c0

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12683783240)
