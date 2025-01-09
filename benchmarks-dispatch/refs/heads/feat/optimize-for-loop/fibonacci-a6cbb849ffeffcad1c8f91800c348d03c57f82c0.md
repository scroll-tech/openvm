| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+245 [+1177.1%])</span> 265.94 | <span style='color: red'>(+245 [+1177.1%])</span> 265.94 |
| fibonacci_program | <span style='color: red'>(+5 [+78.1%])</span> 10.74 | <span style='color: red'>(+5 [+78.1%])</span> 10.74 |
| leaf | <span style='color: red'>(+240 [+1624.9%])</span> 255.20 | <span style='color: red'>(+240 [+1624.9%])</span> 255.20 |


| fibonacci_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+4711 [+78.1%])</span> 10,740 | <span style='color: red'>(+4711 [+78.1%])</span> 10,740 | <span style='color: red'>(+4711 [+78.1%])</span> 10,740 | <span style='color: red'>(+4711 [+78.1%])</span> 10,740 |
| `main_cells_used     ` |  51,503,940 |  51,503,940 |  51,503,940 |  51,503,940 |
| `total_cycles        ` |  1,500,137 |  1,500,137 |  1,500,137 |  1,500,137 |
| `execute_time_ms     ` | <span style='color: red'>(+4610 [+1482.3%])</span> 4,921 | <span style='color: red'>(+4610 [+1482.3%])</span> 4,921 | <span style='color: red'>(+4610 [+1482.3%])</span> 4,921 | <span style='color: red'>(+4610 [+1482.3%])</span> 4,921 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+17 [+2.1%])</span> 824 | <span style='color: red'>(+17 [+2.1%])</span> 824 | <span style='color: red'>(+17 [+2.1%])</span> 824 | <span style='color: red'>(+17 [+2.1%])</span> 824 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+84 [+1.7%])</span> 4,995 | <span style='color: red'>(+84 [+1.7%])</span> 4,995 | <span style='color: red'>(+84 [+1.7%])</span> 4,995 | <span style='color: red'>(+84 [+1.7%])</span> 4,995 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+9 [+1.1%])</span> 806 | <span style='color: red'>(+9 [+1.1%])</span> 806 | <span style='color: red'>(+9 [+1.1%])</span> 806 | <span style='color: red'>(+9 [+1.1%])</span> 806 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-1 [-0.6%])</span> 177 | <span style='color: green'>(-1 [-0.6%])</span> 177 | <span style='color: green'>(-1 [-0.6%])</span> 177 | <span style='color: green'>(-1 [-0.6%])</span> 177 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+9 [+0.6%])</span> 1,598 | <span style='color: red'>(+9 [+0.6%])</span> 1,598 | <span style='color: red'>(+9 [+0.6%])</span> 1,598 | <span style='color: red'>(+9 [+0.6%])</span> 1,598 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+13 [+1.7%])</span> 793 | <span style='color: red'>(+13 [+1.7%])</span> 793 | <span style='color: red'>(+13 [+1.7%])</span> 793 | <span style='color: red'>(+13 [+1.7%])</span> 793 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+16 [+3.2%])</span> 513 | <span style='color: red'>(+16 [+3.2%])</span> 513 | <span style='color: red'>(+16 [+3.2%])</span> 513 | <span style='color: red'>(+16 [+3.2%])</span> 513 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+38 [+3.6%])</span> 1,106 | <span style='color: red'>(+38 [+3.6%])</span> 1,106 | <span style='color: red'>(+38 [+3.6%])</span> 1,106 | <span style='color: red'>(+38 [+3.6%])</span> 1,106 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+240401 [+1624.9%])</span> 255,196 | <span style='color: red'>(+240401 [+1624.9%])</span> 255,196 | <span style='color: red'>(+240401 [+1624.9%])</span> 255,196 | <span style='color: red'>(+240401 [+1624.9%])</span> 255,196 |
| `main_cells_used     ` | <span style='color: red'>(+1244186026 [+965.5%])</span> 1,373,051,513 | <span style='color: red'>(+1244186026 [+965.5%])</span> 1,373,051,513 | <span style='color: red'>(+1244186026 [+965.5%])</span> 1,373,051,513 | <span style='color: red'>(+1244186026 [+965.5%])</span> 1,373,051,513 |
| `total_cycles        ` | <span style='color: red'>(+32475835 [+1023.8%])</span> 35,647,849 | <span style='color: red'>(+32475835 [+1023.8%])</span> 35,647,849 | <span style='color: red'>(+32475835 [+1023.8%])</span> 35,647,849 | <span style='color: red'>(+32475835 [+1023.8%])</span> 35,647,849 |
| `execute_time_ms     ` | <span style='color: red'>(+120106 [+17714.7%])</span> 120,784 | <span style='color: red'>(+120106 [+17714.7%])</span> 120,784 | <span style='color: red'>(+120106 [+17714.7%])</span> 120,784 | <span style='color: red'>(+120106 [+17714.7%])</span> 120,784 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+17255 [+871.5%])</span> 19,235 | <span style='color: red'>(+17255 [+871.5%])</span> 19,235 | <span style='color: red'>(+17255 [+871.5%])</span> 19,235 | <span style='color: red'>(+17255 [+871.5%])</span> 19,235 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+103040 [+849.0%])</span> 115,177 | <span style='color: red'>(+103040 [+849.0%])</span> 115,177 | <span style='color: red'>(+103040 [+849.0%])</span> 115,177 | <span style='color: red'>(+103040 [+849.0%])</span> 115,177 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+16638 [+680.8%])</span> 19,082 | <span style='color: red'>(+16638 [+680.8%])</span> 19,082 | <span style='color: red'>(+16638 [+680.8%])</span> 19,082 | <span style='color: red'>(+16638 [+680.8%])</span> 19,082 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+1803 [+565.2%])</span> 2,122 | <span style='color: red'>(+1803 [+565.2%])</span> 2,122 | <span style='color: red'>(+1803 [+565.2%])</span> 2,122 | <span style='color: red'>(+1803 [+565.2%])</span> 2,122 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+15039 [+712.4%])</span> 17,150 | <span style='color: red'>(+15039 [+712.4%])</span> 17,150 | <span style='color: red'>(+15039 [+712.4%])</span> 17,150 | <span style='color: red'>(+15039 [+712.4%])</span> 17,150 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+24753 [+915.1%])</span> 27,458 | <span style='color: red'>(+24753 [+915.1%])</span> 27,458 | <span style='color: red'>(+24753 [+915.1%])</span> 27,458 | <span style='color: red'>(+24753 [+915.1%])</span> 27,458 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+17782 [+842.7%])</span> 19,892 | <span style='color: red'>(+17782 [+842.7%])</span> 19,892 | <span style='color: red'>(+17782 [+842.7%])</span> 19,892 | <span style='color: red'>(+17782 [+842.7%])</span> 19,892 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+27025 [+1104.9%])</span> 29,471 | <span style='color: red'>(+27025 [+1104.9%])</span> 29,471 | <span style='color: red'>(+27025 [+1104.9%])</span> 29,471 | <span style='color: red'>(+27025 [+1104.9%])</span> 29,471 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | keygen_time_ms | commit_exe_time_ms |
| --- | --- | --- | --- |
| fibonacci_program | 1 | 350 | 6 | 

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
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | BNE | 20,950,470 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | BNE | 41,906,460 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | BEQ | 20,950,470 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | BEQ | 2,300 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | ZipFor | 0 | BNE | 5,743,675 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | 10 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | For | 0 | JAL | 230,160 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | ZipFor | 0 | JAL | 16,100 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | ADD | 54,753,480 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | ADD | 54,659,430 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | ADD | 59,400 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | ADD | 138,434,100 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | ADD | 1,424,970 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | MUL | 741,600 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | ADD | 6,162,030 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | ADD | 27,336,990 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | MUL | 7,290 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | 0 | ADD | 30 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | ADD | 97,440 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | MUL | 95,340 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | MUL | 2,400 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | MUL | 2,730 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | MUL | 600 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | MUL | 78,660 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | ADD | 54,667,560 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | MUL | 9,240 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | ADD | 6,139,980 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | ADD | 37,110 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | MUL | 34,800 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | SUB | 600 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | SUB | 27,326,700 | 
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
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | 0 | STOREW | 98,853,854 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | 0 | LOADW | 109,120 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | 0 | LOADW | 84,724,302 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | 0 | LOADW | 15,024,553 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | 0 | STOREW | 112,950,112 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | 0 | STOREW | 28,348,880 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | 0 | STOREW | 56,493,749 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | 0 | SHINTW | 7,050,795 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | 0 | STOREW | 1,345,245 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ZipFor | 0 | LOADW | 49,941 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | FE4ADD | 27,327,440 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | BBE4MUL | 36,435,520 | 
| leaf | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 79,253,868 | 
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
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 4,123,204 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | 0 | 24,627 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 12,878,268 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 16,777,147 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1,594,074 | 
| leaf | AccessAdapter<2> | 0 | 11,841,864 | 
| leaf | AccessAdapter<4> | 0 | 5,920,932 | 
| leaf | AccessAdapter<8> | 0 | 455,484 | 
| leaf | Arc<BabyBearParameters>, 1> | 0 | 227,741 | 
| leaf | Boundary | 0 | 1,186,390 | 
| leaf | PhantomAir | 0 | 22,788 | 
| leaf | ProgramChip | 0 | 90,108 | 
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
| leaf | AddFI | 0 | ADD | 1,821,981 | 
| leaf | AddV | 0 | ADD | 1,980 | 
| leaf | AddVI | 0 | ADD | 4,614,470 | 
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
| leaf | IfEq | 0 | BNE | 910,890 | 
| leaf | IfEqI | 0 | BNE | 1,822,020 | 
| leaf | IfNe | 0 | BEQ | 910,890 | 
| leaf | IfNeI | 0 | BEQ | 100 | 
| leaf | ImmE | 0 | STOREW | 4 | 
| leaf | ImmF | 0 | STOREW | 153 | 
| leaf | ImmV | 0 | STOREW | 3,188,834 | 
| leaf | LoadE | 0 | LOADW | 3,520 | 
| leaf | LoadF | 0 | ADD | 911,233 | 
| leaf | LoadF | 0 | LOADW | 2,733,042 | 
| leaf | LoadF | 0 | MUL | 243 | 
| leaf | LoadHeapPtr | 0 | ADD | 1 | 
| leaf | LoadV | 0 | ADD | 3,248 | 
| leaf | LoadV | 0 | LOADW | 484,663 | 
| leaf | LoadV | 0 | MUL | 3,178 | 
| leaf | MulEF | 0 | MUL | 80 | 
| leaf | MulEI | 0 | BBE4MUL | 910,888 | 
| leaf | MulEI | 0 | STOREW | 3,643,552 | 
| leaf | MulF | 0 | MUL | 91 | 
| leaf | MulFI | 0 | MUL | 20 | 
| leaf | MulVI | 0 | MUL | 2,622 | 
| leaf | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 227,741 | 
| leaf | StoreE | 0 | STOREW | 914,480 | 
| leaf | StoreF | 0 | ADD | 1,822,252 | 
| leaf | StoreF | 0 | MUL | 308 | 
| leaf | StoreF | 0 | STOREW | 1,822,379 | 
| leaf | StoreHintWord | 0 | ADD | 204,666 | 
| leaf | StoreHintWord | 0 | SHINTW | 227,445 | 
| leaf | StoreV | 0 | ADD | 1,237 | 
| leaf | StoreV | 0 | MUL | 1,160 | 
| leaf | StoreV | 0 | STOREW | 43,395 | 
| leaf | SubV | 0 | SUB | 20 | 
| leaf | SubVI | 0 | SUB | 910,890 | 
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
| leaf | 0 | 19,235 | 255,196 | 35,647,849 | 2,894,200,856 | 115,177 | 27,458 | 19,892 | 17,150 | 29,471 | 19,082 | 1,373,051,513 | 2,122 | 120,784 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 | 824 | 10,740 | 1,500,137 | 197,453,854 | 4,995 | 793 | 513 | 1,598 | 1,106 | 806 | 51,503,940 | 177 | 4,921 | 

</details>


<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/fibonacci-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-leaf.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/a6cbb849ffeffcad1c8f91800c348d03c57f82c0

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12683783240)
