| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+248 [+1192.7%])</span> 269.19 | <span style='color: red'>(+248 [+1192.7%])</span> 269.19 |
| fibonacci_program | <span style='color: red'>(+5 [+77.3%])</span> 10.69 | <span style='color: red'>(+5 [+77.3%])</span> 10.69 |
| leaf | <span style='color: red'>(+244 [+1647.2%])</span> 258.50 | <span style='color: red'>(+244 [+1647.2%])</span> 258.50 |


| fibonacci_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+4663 [+77.3%])</span> 10,692 | <span style='color: red'>(+4663 [+77.3%])</span> 10,692 | <span style='color: red'>(+4663 [+77.3%])</span> 10,692 | <span style='color: red'>(+4663 [+77.3%])</span> 10,692 |
| `main_cells_used     ` |  51,503,940 |  51,503,940 |  51,503,940 |  51,503,940 |
| `total_cycles        ` |  1,500,137 |  1,500,137 |  1,500,137 |  1,500,137 |
| `execute_time_ms     ` | <span style='color: red'>(+4614 [+1483.6%])</span> 4,925 | <span style='color: red'>(+4614 [+1483.6%])</span> 4,925 | <span style='color: red'>(+4614 [+1483.6%])</span> 4,925 | <span style='color: red'>(+4614 [+1483.6%])</span> 4,925 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+18 [+2.2%])</span> 825 | <span style='color: red'>(+18 [+2.2%])</span> 825 | <span style='color: red'>(+18 [+2.2%])</span> 825 | <span style='color: red'>(+18 [+2.2%])</span> 825 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+31 [+0.6%])</span> 4,942 | <span style='color: red'>(+31 [+0.6%])</span> 4,942 | <span style='color: red'>(+31 [+0.6%])</span> 4,942 | <span style='color: red'>(+31 [+0.6%])</span> 4,942 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+4 [+0.5%])</span> 801 | <span style='color: red'>(+4 [+0.5%])</span> 801 | <span style='color: red'>(+4 [+0.5%])</span> 801 | <span style='color: red'>(+4 [+0.5%])</span> 801 |
| `generate_perm_trace_time_ms` |  178 |  178 |  178 |  178 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+33 [+2.1%])</span> 1,622 | <span style='color: red'>(+33 [+2.1%])</span> 1,622 | <span style='color: red'>(+33 [+2.1%])</span> 1,622 | <span style='color: red'>(+33 [+2.1%])</span> 1,622 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-10 [-1.3%])</span> 770 | <span style='color: green'>(-10 [-1.3%])</span> 770 | <span style='color: green'>(-10 [-1.3%])</span> 770 | <span style='color: green'>(-10 [-1.3%])</span> 770 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+24 [+4.8%])</span> 521 | <span style='color: red'>(+24 [+4.8%])</span> 521 | <span style='color: red'>(+24 [+4.8%])</span> 521 | <span style='color: red'>(+24 [+4.8%])</span> 521 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-21 [-2.0%])</span> 1,047 | <span style='color: green'>(-21 [-2.0%])</span> 1,047 | <span style='color: green'>(-21 [-2.0%])</span> 1,047 | <span style='color: green'>(-21 [-2.0%])</span> 1,047 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+243700 [+1647.2%])</span> 258,495 | <span style='color: red'>(+243700 [+1647.2%])</span> 258,495 | <span style='color: red'>(+243700 [+1647.2%])</span> 258,495 | <span style='color: red'>(+243700 [+1647.2%])</span> 258,495 |
| `main_cells_used     ` | <span style='color: red'>(+1276519995 [+990.6%])</span> 1,405,385,482 | <span style='color: red'>(+1276519995 [+990.6%])</span> 1,405,385,482 | <span style='color: red'>(+1276519995 [+990.6%])</span> 1,405,385,482 | <span style='color: red'>(+1276519995 [+990.6%])</span> 1,405,385,482 |
| `total_cycles        ` | <span style='color: red'>(+33386653 [+1052.5%])</span> 36,558,667 | <span style='color: red'>(+33386653 [+1052.5%])</span> 36,558,667 | <span style='color: red'>(+33386653 [+1052.5%])</span> 36,558,667 | <span style='color: red'>(+33386653 [+1052.5%])</span> 36,558,667 |
| `execute_time_ms     ` | <span style='color: red'>(+123659 [+18238.8%])</span> 124,337 | <span style='color: red'>(+123659 [+18238.8%])</span> 124,337 | <span style='color: red'>(+123659 [+18238.8%])</span> 124,337 | <span style='color: red'>(+123659 [+18238.8%])</span> 124,337 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+18153 [+916.8%])</span> 20,133 | <span style='color: red'>(+18153 [+916.8%])</span> 20,133 | <span style='color: red'>(+18153 [+916.8%])</span> 20,133 | <span style='color: red'>(+18153 [+916.8%])</span> 20,133 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+101888 [+839.5%])</span> 114,025 | <span style='color: red'>(+101888 [+839.5%])</span> 114,025 | <span style='color: red'>(+101888 [+839.5%])</span> 114,025 | <span style='color: red'>(+101888 [+839.5%])</span> 114,025 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+16489 [+674.7%])</span> 18,933 | <span style='color: red'>(+16489 [+674.7%])</span> 18,933 | <span style='color: red'>(+16489 [+674.7%])</span> 18,933 | <span style='color: red'>(+16489 [+674.7%])</span> 18,933 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+1737 [+544.5%])</span> 2,056 | <span style='color: red'>(+1737 [+544.5%])</span> 2,056 | <span style='color: red'>(+1737 [+544.5%])</span> 2,056 | <span style='color: red'>(+1737 [+544.5%])</span> 2,056 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+15101 [+715.3%])</span> 17,212 | <span style='color: red'>(+15101 [+715.3%])</span> 17,212 | <span style='color: red'>(+15101 [+715.3%])</span> 17,212 | <span style='color: red'>(+15101 [+715.3%])</span> 17,212 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+24389 [+901.6%])</span> 27,094 | <span style='color: red'>(+24389 [+901.6%])</span> 27,094 | <span style='color: red'>(+24389 [+901.6%])</span> 27,094 | <span style='color: red'>(+24389 [+901.6%])</span> 27,094 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+17609 [+834.5%])</span> 19,719 | <span style='color: red'>(+17609 [+834.5%])</span> 19,719 | <span style='color: red'>(+17609 [+834.5%])</span> 19,719 | <span style='color: red'>(+17609 [+834.5%])</span> 19,719 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+26562 [+1085.9%])</span> 29,008 | <span style='color: red'>(+26562 [+1085.9%])</span> 29,008 | <span style='color: red'>(+26562 [+1085.9%])</span> 29,008 | <span style='color: red'>(+26562 [+1085.9%])</span> 29,008 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | keygen_time_ms | commit_exe_time_ms |
| --- | --- | --- | --- |
| fibonacci_program | 1 | 348 | 6 | 

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
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 4 | 15 | 24 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 4 | 15 | 23 | 
| leaf | VmConnectorAir | 4 | 3 | 8 | 
| leaf | VolatileBoundaryAir | 4 | 4 | 16 | 

| group | air_name | dsl_ir | idx | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | BNE | 92 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | BNE | 24,656 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | BNE | 1,955 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | BEQ | 23 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | BNE | 5,253,591 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | BNE | 20,950,424 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | BNE | 41,906,345 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | BEQ | 20,950,424 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | BEQ | 2,300 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | ZipFor | 0 | BNE | 5,743,675 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | 10 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | For | 0 | JAL | 230,160 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | ZipFor | 0 | JAL | 16,100 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | ADD | 54,753,480 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | ADD | 54,659,280 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | ADD | 59,400 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | ADD | 138,433,770 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | ADD | 1,424,970 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | MUL | 741,600 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | ADD | 6,162,030 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | ADD | 27,336,930 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | MUL | 7,290 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | 0 | ADD | 30 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | ADD | 13,760,520 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | MUL | 13,758,420 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | MUL | 2,400 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | MUL | 2,730 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | MUL | 600 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | MUL | 78,660 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | ADD | 54,667,410 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | MUL | 9,240 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | ADD | 6,139,980 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | ADD | 37,110 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | MUL | 34,800 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | SUB | 600 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | SUB | 27,326,640 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | 0 | ADD | 600 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | ZipFor | 0 | ADD | 14,274,960 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | 31 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | 0 | LOADW | 28,237,528 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | 0 | STOREW | 84,712,584 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | 0 | LOADW | 1,472,469 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | 0 | LOADW | 1,364 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | 0 | STOREW | 712,132 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | 0 | STOREW | 124 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | 0 | STOREW | 4,743 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | 0 | STOREW | 98,853,606 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | 0 | LOADW | 109,120 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | 0 | LOADW | 84,724,116 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | 0 | LOADW | 15,024,553 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | 0 | STOREW | 112,950,112 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | 0 | STOREW | 28,348,880 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | 0 | STOREW | 56,493,594 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | 0 | SHINTW | 7,050,795 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | 0 | STOREW | 1,345,245 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ZipFor | 0 | LOADW | 49,941 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | FE4ADD | 27,327,440 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | BBE4MUL | 36,435,520 | 
| leaf | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 79,253,520 | 
| leaf | PhantomAir | CT-InitializePcsConst | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-ReadProofsFromInput | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-VerifyProofs | 0 | PHANTOM | 6 | 
| leaf | PhantomAir | CT-stage-c-build-rounds | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-stage-d-verifier-verify | 0 | PHANTOM | 6 | 
| leaf | PhantomAir | CT-stage-d-verify-pcs | 0 | PHANTOM | 6 | 
| leaf | PhantomAir | HintInputVec | 0 | PHANTOM | 136,674 | 

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | 32,401,620 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | 72 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | 36 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | 144 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | 72 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | 11,100,074 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | 106 | 
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | 2,600,104 | 
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | 2,600,130 | 
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | 96 | 
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | 64 | 
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | 1,800,018 | 
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | 162 | 
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | 78 | 
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | 364 | 
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | 520 | 
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | 600 | 
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | 168 | 
| fibonacci_program | PhantomAir |  | PHANTOM | 0 | 12 | 

| group | air_name | idx | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | AccessAdapterAir<2> | 0 | 16,777,216 |  | 16 | 11 | 452,984,832 | 
| leaf | AccessAdapterAir<4> | 0 | 8,388,608 |  | 16 | 13 | 243,269,632 | 
| leaf | AccessAdapterAir<8> | 0 | 524,288 |  | 16 | 17 | 17,301,504 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 262,144 |  | 36 | 348 | 100,663,296 | 
| leaf | PhantomAir | 0 | 32,768 |  | 8 | 6 | 458,752 | 
| leaf | ProgramAir | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 4,194,304 |  | 28 | 23 | 213,909,504 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 32,768 |  | 12 | 10 | 720,896 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 16,777,216 |  | 20 | 30 | 838,860,800 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 16,777,216 |  | 20 | 31 | 855,638,016 | 
| leaf | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 2,097,152 |  | 20 | 40 | 125,829,120 | 
| leaf | VmConnectorAir | 0 | 2 | 1 | 8 | 4 | 24 | 
| leaf | VolatileBoundaryAir | 0 | 2,097,152 |  | 8 | 11 | 39,845,888 | 

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

| group | chip_name | idx | rows_used |
| --- | --- | --- | --- |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 4,123,195 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | 0 | 24,627 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 13,789,115 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 16,777,128 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1,594,074 | 
| leaf | AccessAdapter<2> | 0 | 11,841,856 | 
| leaf | AccessAdapter<4> | 0 | 5,920,928 | 
| leaf | AccessAdapter<8> | 0 | 455,482 | 
| leaf | Arc<BabyBearParameters>, 1> | 0 | 227,740 | 
| leaf | Boundary | 0 | 1,641,833 | 
| leaf | PhantomAir | 0 | 22,788 | 
| leaf | ProgramChip | 0 | 90,112 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 
| leaf | VmConnectorAir | 0 | 2 | 

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| fibonacci_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | 900,054 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | 300,002 | 
| fibonacci_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | 2 | 
| fibonacci_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | 200,009 | 
| fibonacci_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | 5 | 
| fibonacci_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | 100,010 | 
| fibonacci_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | 3 | 
| fibonacci_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | 13 | 
| fibonacci_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | 28 | 
| fibonacci_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | 9 | 
| fibonacci_program | AccessAdapter<8> | 0 | 36 | 
| fibonacci_program | Arc<BabyBearParameters>, 1> | 0 | 228 | 
| fibonacci_program | BitwiseOperationLookupAir<8> | 0 | 65,536 | 
| fibonacci_program | Boundary | 0 | 36 | 
| fibonacci_program | Merkle | 0 | 280 | 
| fibonacci_program | PhantomAir | 0 | 2 | 
| fibonacci_program | ProgramChip | 0 | 3,275 | 
| fibonacci_program | RangeTupleCheckerAir<2> | 0 | 524,288 | 
| fibonacci_program | VariableRangeCheckerAir | 0 | 262,144 | 
| fibonacci_program | VmConnectorAir | 0 | 2 | 

| group | dsl_ir | idx | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf |  | 0 | JAL | 1 | 
| leaf |  | 0 | STOREW | 2 | 
| leaf | AddE | 0 | FE4ADD | 683,186 | 
| leaf | AddEFFI | 0 | LOADW | 910,888 | 
| leaf | AddEFFI | 0 | STOREW | 2,732,664 | 
| leaf | AddEI | 0 | ADD | 1,825,116 | 
| leaf | AddFI | 0 | ADD | 1,821,976 | 
| leaf | AddV | 0 | ADD | 1,980 | 
| leaf | AddVI | 0 | ADD | 4,614,459 | 
| leaf | Alloc | 0 | ADD | 47,499 | 
| leaf | Alloc | 0 | LOADW | 47,499 | 
| leaf | Alloc | 0 | MUL | 24,720 | 
| leaf | AssertEqEI | 0 | BNE | 4 | 
| leaf | AssertEqV | 0 | BNE | 1,072 | 
| leaf | AssertEqVI | 0 | BNE | 85 | 
| leaf | AssertNeVI | 0 | BEQ | 1 | 
| leaf | CT-InitializePcsConst | 0 | PHANTOM | 2 | 
| leaf | CT-ReadProofsFromInput | 0 | PHANTOM | 2 | 
| leaf | CT-VerifyProofs | 0 | PHANTOM | 1 | 
| leaf | CT-stage-c-build-rounds | 0 | PHANTOM | 2 | 
| leaf | CT-stage-d-verifier-verify | 0 | PHANTOM | 1 | 
| leaf | CT-stage-d-verify-pcs | 0 | PHANTOM | 1 | 
| leaf | For | 0 | ADD | 205,401 | 
| leaf | For | 0 | BNE | 228,417 | 
| leaf | For | 0 | JAL | 23,016 | 
| leaf | For | 0 | LOADW | 44 | 
| leaf | For | 0 | STOREW | 22,972 | 
| leaf | HintInputVec | 0 | PHANTOM | 22,779 | 
| leaf | IfEq | 0 | BNE | 910,888 | 
| leaf | IfEqI | 0 | BNE | 1,822,015 | 
| leaf | IfNe | 0 | BEQ | 910,888 | 
| leaf | IfNeI | 0 | BEQ | 100 | 
| leaf | ImmE | 0 | STOREW | 4 | 
| leaf | ImmF | 0 | STOREW | 153 | 
| leaf | ImmV | 0 | STOREW | 3,188,826 | 
| leaf | LoadE | 0 | LOADW | 3,520 | 
| leaf | LoadF | 0 | ADD | 911,231 | 
| leaf | LoadF | 0 | LOADW | 2,733,036 | 
| leaf | LoadF | 0 | MUL | 243 | 
| leaf | LoadHeapPtr | 0 | ADD | 1 | 
| leaf | LoadV | 0 | ADD | 458,684 | 
| leaf | LoadV | 0 | LOADW | 484,663 | 
| leaf | LoadV | 0 | MUL | 458,614 | 
| leaf | MulEF | 0 | MUL | 80 | 
| leaf | MulEI | 0 | BBE4MUL | 910,888 | 
| leaf | MulEI | 0 | STOREW | 3,643,552 | 
| leaf | MulF | 0 | MUL | 91 | 
| leaf | MulFI | 0 | MUL | 20 | 
| leaf | MulVI | 0 | MUL | 2,622 | 
| leaf | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 227,740 | 
| leaf | StoreE | 0 | STOREW | 914,480 | 
| leaf | StoreF | 0 | ADD | 1,822,247 | 
| leaf | StoreF | 0 | MUL | 308 | 
| leaf | StoreF | 0 | STOREW | 1,822,374 | 
| leaf | StoreHintWord | 0 | ADD | 204,666 | 
| leaf | StoreHintWord | 0 | SHINTW | 227,445 | 
| leaf | StoreV | 0 | ADD | 1,237 | 
| leaf | StoreV | 0 | MUL | 1,160 | 
| leaf | StoreV | 0 | STOREW | 43,395 | 
| leaf | SubV | 0 | SUB | 20 | 
| leaf | SubVI | 0 | SUB | 910,888 | 
| leaf | UnsafeCastVF | 0 | ADD | 20 | 
| leaf | ZipFor | 0 | ADD | 475,832 | 
| leaf | ZipFor | 0 | BNE | 249,725 | 
| leaf | ZipFor | 0 | JAL | 1,610 | 
| leaf | ZipFor | 0 | LOADW | 1,611 | 

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| fibonacci_program |  | ADD | 0 | 900,045 | 
| fibonacci_program |  | AND | 0 | 2 | 
| fibonacci_program |  | AUIPC | 0 | 9 | 
| fibonacci_program |  | BEQ | 0 | 100,004 | 
| fibonacci_program |  | BGEU | 0 | 3 | 
| fibonacci_program |  | BLTU | 0 | 2 | 
| fibonacci_program |  | BNE | 0 | 100,005 | 
| fibonacci_program |  | HINT_STOREW | 0 | 3 | 
| fibonacci_program |  | JAL | 0 | 100,001 | 
| fibonacci_program |  | JALR | 0 | 13 | 
| fibonacci_program |  | LOADW | 0 | 13 | 
| fibonacci_program |  | LUI | 0 | 9 | 
| fibonacci_program |  | OR | 0 | 1 | 
| fibonacci_program |  | PHANTOM | 0 | 2 | 
| fibonacci_program |  | SLL | 0 | 2 | 
| fibonacci_program |  | SLTU | 0 | 300,002 | 
| fibonacci_program |  | STOREW | 0 | 15 | 
| fibonacci_program |  | SUB | 0 | 4 | 
| fibonacci_program |  | XOR | 0 | 2 | 

| group | idx | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf | 0 | 20,133 | 258,495 | 36,558,667 | 2,894,200,856 | 114,025 | 27,094 | 19,719 | 17,212 | 29,008 | 18,933 | 1,405,385,482 | 2,056 | 124,337 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 | 825 | 10,692 | 1,500,137 | 197,453,854 | 4,942 | 770 | 521 | 1,622 | 1,047 | 801 | 51,503,940 | 178 | 4,925 | 

</details>


<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9/fibonacci-f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9-leaf.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/f92aea1e6b6c34d7e978ac8bc5c4fc86929b88b9

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12683609581)
