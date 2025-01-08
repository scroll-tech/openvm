| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+17 [+78.7%])</span> 39.32 | <span style='color: red'>(+17 [+78.7%])</span> 39.32 |
| fibonacci_program | <span style='color: red'>(+5 [+71.6%])</span> 11.05 | <span style='color: red'>(+5 [+71.6%])</span> 11.05 |
| leaf | <span style='color: red'>(+13 [+81.6%])</span> 28.27 | <span style='color: red'>(+13 [+81.6%])</span> 28.27 |


| fibonacci_program |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+4611 [+71.6%])</span> 11,050 | <span style='color: red'>(+4611 [+71.6%])</span> 11,050 | <span style='color: red'>(+4611 [+71.6%])</span> 11,050 | <span style='color: red'>(+4611 [+71.6%])</span> 11,050 |
| `main_cells_used     ` |  51,503,940 |  51,503,940 |  51,503,940 |  51,503,940 |
| `total_cycles        ` |  1,500,137 |  1,500,137 |  1,500,137 |  1,500,137 |
| `execute_time_ms     ` | <span style='color: red'>(+4594 [+1044.1%])</span> 5,034 | <span style='color: red'>(+4594 [+1044.1%])</span> 5,034 | <span style='color: red'>(+4594 [+1044.1%])</span> 5,034 | <span style='color: red'>(+4594 [+1044.1%])</span> 5,034 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+1 [+0.1%])</span> 925 | <span style='color: red'>(+1 [+0.1%])</span> 925 | <span style='color: red'>(+1 [+0.1%])</span> 925 | <span style='color: red'>(+1 [+0.1%])</span> 925 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+16 [+0.3%])</span> 5,091 | <span style='color: red'>(+16 [+0.3%])</span> 5,091 | <span style='color: red'>(+16 [+0.3%])</span> 5,091 | <span style='color: red'>(+16 [+0.3%])</span> 5,091 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+4 [+0.5%])</span> 817 | <span style='color: red'>(+4 [+0.5%])</span> 817 | <span style='color: red'>(+4 [+0.5%])</span> 817 | <span style='color: red'>(+4 [+0.5%])</span> 817 |
| `generate_perm_trace_time_ms` |  179 |  179 |  179 |  179 |
| `perm_trace_commit_time_ms` |  1,751 |  1,751 |  1,751 |  1,751 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+3 [+0.4%])</span> 794 | <span style='color: red'>(+3 [+0.4%])</span> 794 | <span style='color: red'>(+3 [+0.4%])</span> 794 | <span style='color: red'>(+3 [+0.4%])</span> 794 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+9 [+1.8%])</span> 501 | <span style='color: red'>(+9 [+1.8%])</span> 501 | <span style='color: red'>(+9 [+1.8%])</span> 501 | <span style='color: red'>(+9 [+1.8%])</span> 501 |
| `pcs_opening_time_ms ` |  1,047 |  1,047 |  1,047 |  1,047 |

| leaf |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+12704 [+81.6%])</span> 28,272 | <span style='color: red'>(+12704 [+81.6%])</span> 28,272 | <span style='color: red'>(+12704 [+81.6%])</span> 28,272 | <span style='color: red'>(+12704 [+81.6%])</span> 28,272 |
| `main_cells_used     ` | <span style='color: red'>(+9581270 [+7.4%])</span> 138,441,657 | <span style='color: red'>(+9581270 [+7.4%])</span> 138,441,657 | <span style='color: red'>(+9581270 [+7.4%])</span> 138,441,657 | <span style='color: red'>(+9581270 [+7.4%])</span> 138,441,657 |
| `total_cycles        ` | <span style='color: red'>(+814114 [+25.7%])</span> 3,985,751 | <span style='color: red'>(+814114 [+25.7%])</span> 3,985,751 | <span style='color: red'>(+814114 [+25.7%])</span> 3,985,751 | <span style='color: red'>(+814114 [+25.7%])</span> 3,985,751 |
| `execute_time_ms     ` | <span style='color: red'>(+13262 [+1129.6%])</span> 14,436 | <span style='color: red'>(+13262 [+1129.6%])</span> 14,436 | <span style='color: red'>(+13262 [+1129.6%])</span> 14,436 | <span style='color: red'>(+13262 [+1129.6%])</span> 14,436 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+175 [+7.5%])</span> 2,499 | <span style='color: red'>(+175 [+7.5%])</span> 2,499 | <span style='color: red'>(+175 [+7.5%])</span> 2,499 | <span style='color: red'>(+175 [+7.5%])</span> 2,499 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-733 [-6.1%])</span> 11,337 | <span style='color: green'>(-733 [-6.1%])</span> 11,337 | <span style='color: green'>(-733 [-6.1%])</span> 11,337 | <span style='color: green'>(-733 [-6.1%])</span> 11,337 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-123 [-5.3%])</span> 2,177 | <span style='color: green'>(-123 [-5.3%])</span> 2,177 | <span style='color: green'>(-123 [-5.3%])</span> 2,177 | <span style='color: green'>(-123 [-5.3%])</span> 2,177 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-65 [-19.6%])</span> 266 | <span style='color: green'>(-65 [-19.6%])</span> 266 | <span style='color: green'>(-65 [-19.6%])</span> 266 | <span style='color: green'>(-65 [-19.6%])</span> 266 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-127 [-6.5%])</span> 1,836 | <span style='color: green'>(-127 [-6.5%])</span> 1,836 | <span style='color: green'>(-127 [-6.5%])</span> 1,836 | <span style='color: green'>(-127 [-6.5%])</span> 1,836 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-424 [-13.8%])</span> 2,655 | <span style='color: green'>(-424 [-13.8%])</span> 2,655 | <span style='color: green'>(-424 [-13.8%])</span> 2,655 | <span style='color: green'>(-424 [-13.8%])</span> 2,655 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+20 [+1.0%])</span> 2,013 | <span style='color: red'>(+20 [+1.0%])</span> 2,013 | <span style='color: red'>(+20 [+1.0%])</span> 2,013 | <span style='color: red'>(+20 [+1.0%])</span> 2,013 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-14 [-0.6%])</span> 2,388 | <span style='color: green'>(-14 [-0.6%])</span> 2,388 | <span style='color: green'>(-14 [-0.6%])</span> 2,388 | <span style='color: green'>(-14 [-0.6%])</span> 2,388 |



<details>
<summary>Detailed Metrics</summary>

| group | num_segments | keygen_time_ms | commit_exe_time_ms |
| --- | --- | --- | --- |
| fibonacci_program | 1 | 360 | 5 | 

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
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 0 | BNE | 5,704 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | BNE | 92 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 0 | BNE | 248,216 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | BNE | 24,679 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | BNE | 5,543 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | BEQ | 23 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | BNE | 10,273,226 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | BNE | 682,364 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | BNE | 2,985,538 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | BEQ | 362,641 | 
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | BEQ | 67,045 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | 10 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | For | 0 | JAL | 448,310 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 0 | JAL | 304,330 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | JAL | 10 | 
| leaf | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 0 | PUBLISH | 828 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 0 | ADD | 18,480 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | ADD | 829,560 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | 0 | ADD | 39,990 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | ADD | 1,298,040 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | ADD | 443,820 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | ADD | 9,790,590 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | ADD | 1,720,860 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | MUL | 1,024,500 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 0 | ADD | 30 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 0 | DIV | 3,840 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | ADD | 12,054,930 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadE | 0 | ADD | 493,380 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadE | 0 | MUL | 493,380 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | ADD | 2,417,760 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | 0 | MUL | 443,130 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | 0 | ADD | 30 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | ADD | 6,068,880 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | 0 | MUL | 3,344,940 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | MUL | 113,760 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 0 | MUL | 15,000 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | MUL | 2,510,010 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | MUL | 40,590 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | MUL | 601,680 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 0 | MUL | 5,160 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreE | 0 | ADD | 257,640 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreE | 0 | MUL | 257,640 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | ADD | 2,097,180 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | 0 | MUL | 21,990 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHeapPtr | 0 | ADD | 30 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | ADD | 6,152,970 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | ADD | 1,868,220 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | 0 | MUL | 1,750,800 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 0 | SUB | 161,820 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 0 | ADD | 10,320 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 0 | ADD | 12,960 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | 0 | SUB | 39,990 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | SUB | 1,486,140 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | SUB | 30,000 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 0 | SUB | 25,200 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | 0 | ADD | 600 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | 31 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | 0 | LOADW | 5,456 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | 0 | STOREW | 16,368 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | 0 | LOADW | 1,778,222 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | DivEIN | 0 | STOREW | 6,696 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | 0 | LOADW | 88,536 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | 0 | STOREW | 1,301,225 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | 0 | STOREW | 99,448 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | 0 | STOREW | 1,448,971 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | 0 | STOREW | 890,971 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | 0 | LOADW | 2,710,392 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | 0 | LOADW | 3,405,474 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | 0 | LOADW | 7,086,042 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | 0 | STOREW | 189,224 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | 0 | STOREW | 1,823,172 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | 0 | STOREW | 3,384,704 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | 0 | SHINTW | 7,118,964 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | 0 | STOREW | 2,021,448 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | SubEF | 0 | LOADW | 501,642 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | FE4ADD | 497,320 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 0 | BBE4DIV | 248,560 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 0 | BBE4DIV | 2,160 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 0 | BBE4MUL | 341,400 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | BBE4MUL | 61,040 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 0 | FE4SUB | 132,240 | 
| leaf | Arc<BabyBearParameters>, 1> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | 6,025,620 | 
| leaf | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 3,218,652 | 
| leaf | FriReducedOpeningAir | FriReducedOpening | 0 | FRI_REDUCED_OPENING | 7,547,904 | 
| leaf | PhantomAir | CT-ExtractPublicValuesCommit | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-InitializePcsConst | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-ReadProofsFromInput | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-VerifyProofs | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-compute-reduced-opening | 0 | PHANTOM | 4,032 | 
| leaf | PhantomAir | CT-exp-reverse-bits-len | 0 | PHANTOM | 41,328 | 
| leaf | PhantomAir | CT-poseidon2-hash | 0 | PHANTOM | 23,688 | 
| leaf | PhantomAir | CT-poseidon2-hash-ext | 0 | PHANTOM | 10,080 | 
| leaf | PhantomAir | CT-poseidon2-hash-setup | 0 | PHANTOM | 744,912 | 
| leaf | PhantomAir | CT-single-reduced-opening-eval | 0 | PHANTOM | 64,008 | 
| leaf | PhantomAir | CT-stage-c-build-rounds | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-stage-d-verifier-verify | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-stage-d-verify-pcs | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-stage-e-verify-constraints | 0 | PHANTOM | 12 | 
| leaf | PhantomAir | CT-verify-batch | 0 | PHANTOM | 4,032 | 
| leaf | PhantomAir | CT-verify-batch-ext | 0 | PHANTOM | 10,080 | 
| leaf | PhantomAir | CT-verify-batch-reduce-fast | 0 | PHANTOM | 33,768 | 
| leaf | PhantomAir | CT-verify-batch-reduce-fast-setup | 0 | PHANTOM | 33,768 | 
| leaf | PhantomAir | CT-verify-query | 0 | PHANTOM | 504 | 
| leaf | PhantomAir | HintBitsF | 0 | PHANTOM | 258 | 
| leaf | PhantomAir | HintInputVec | 0 | PHANTOM | 139,272 | 

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
| leaf | AccessAdapterAir<2> | 0 | 524,288 |  | 16 | 11 | 14,155,776 | 
| leaf | AccessAdapterAir<4> | 0 | 262,144 |  | 16 | 13 | 7,602,176 | 
| leaf | AccessAdapterAir<8> | 0 | 65,536 |  | 16 | 17 | 2,162,688 | 
| leaf | FriReducedOpeningAir | 0 | 131,072 |  | 76 | 64 | 18,350,080 | 
| leaf | NativePoseidon2Air<BabyBearParameters>, 1> | 0 | 32,768 |  | 36 | 348 | 12,582,912 | 
| leaf | PhantomAir | 0 | 262,144 |  | 8 | 6 | 3,670,016 | 
| leaf | ProgramAir | 0 | 131,072 |  | 8 | 10 | 2,359,296 | 
| leaf | VariableRangeCheckerAir | 0 | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| leaf | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1,048,576 |  | 28 | 23 | 53,477,376 | 
| leaf | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 131,072 |  | 12 | 10 | 2,883,584 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 64 |  | 16 | 23 | 2,496 | 
| leaf | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 2,097,152 |  | 20 | 30 | 104,857,600 | 
| leaf | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 2,097,152 |  | 20 | 31 | 106,954,752 | 
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

| group | chip_name | idx | rows_used |
| --- | --- | --- | --- |
| leaf | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 637,177 | 
| leaf | <JalNativeAdapterAir,JalCoreAir> | 0 | 75,266 | 
| leaf | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 36 | 
| leaf | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1,931,528 | 
| leaf | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 1,092,807 | 
| leaf | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 32,068 | 
| leaf | AccessAdapter<2> | 0 | 370,730 | 
| leaf | AccessAdapter<4> | 0 | 185,618 | 
| leaf | AccessAdapter<8> | 0 | 55,694 | 
| leaf | Arc<BabyBearParameters>, 1> | 0 | 26,564 | 
| leaf | Boundary | 0 | 385,022 | 
| leaf | FriReducedOpeningAir | 0 | 117,936 | 
| leaf | PhantomAir | 0 | 184,971 | 
| leaf | ProgramChip | 0 | 90,263 | 
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
| leaf | AddE | 0 | FE4ADD | 12,433 | 
| leaf | AddEFFI | 0 | LOADW | 176 | 
| leaf | AddEFFI | 0 | STOREW | 528 | 
| leaf | AddEFI | 0 | ADD | 616 | 
| leaf | AddEI | 0 | ADD | 27,652 | 
| leaf | AddF | 0 | ADD | 1,333 | 
| leaf | AddFI | 0 | ADD | 43,268 | 
| leaf | AddV | 0 | ADD | 14,794 | 
| leaf | AddVI | 0 | ADD | 326,353 | 
| leaf | Alloc | 0 | ADD | 57,362 | 
| leaf | Alloc | 0 | LOADW | 57,362 | 
| leaf | Alloc | 0 | MUL | 34,150 | 
| leaf | AssertEqE | 0 | BNE | 248 | 
| leaf | AssertEqEI | 0 | BNE | 4 | 
| leaf | AssertEqF | 0 | BNE | 10,792 | 
| leaf | AssertEqV | 0 | BNE | 1,073 | 
| leaf | AssertEqVI | 0 | BNE | 241 | 
| leaf | AssertNeVI | 0 | BEQ | 1 | 
| leaf | CT-ExtractPublicValuesCommit | 0 | PHANTOM | 2 | 
| leaf | CT-InitializePcsConst | 0 | PHANTOM | 2 | 
| leaf | CT-ReadProofsFromInput | 0 | PHANTOM | 2 | 
| leaf | CT-VerifyProofs | 0 | PHANTOM | 2 | 
| leaf | CT-compute-reduced-opening | 0 | PHANTOM | 672 | 
| leaf | CT-exp-reverse-bits-len | 0 | PHANTOM | 6,888 | 
| leaf | CT-poseidon2-hash | 0 | PHANTOM | 3,948 | 
| leaf | CT-poseidon2-hash-ext | 0 | PHANTOM | 1,680 | 
| leaf | CT-poseidon2-hash-setup | 0 | PHANTOM | 124,152 | 
| leaf | CT-single-reduced-opening-eval | 0 | PHANTOM | 10,668 | 
| leaf | CT-stage-c-build-rounds | 0 | PHANTOM | 2 | 
| leaf | CT-stage-d-verifier-verify | 0 | PHANTOM | 2 | 
| leaf | CT-stage-d-verify-pcs | 0 | PHANTOM | 2 | 
| leaf | CT-stage-e-verify-constraints | 0 | PHANTOM | 2 | 
| leaf | CT-verify-batch | 0 | PHANTOM | 672 | 
| leaf | CT-verify-batch-ext | 0 | PHANTOM | 1,680 | 
| leaf | CT-verify-batch-reduce-fast | 0 | PHANTOM | 5,628 | 
| leaf | CT-verify-batch-reduce-fast-setup | 0 | PHANTOM | 5,628 | 
| leaf | CT-verify-query | 0 | PHANTOM | 84 | 
| leaf | CastFV | 0 | ADD | 1 | 
| leaf | DivE | 0 | BBE4DIV | 6,214 | 
| leaf | DivEIN | 0 | BBE4DIV | 54 | 
| leaf | DivEIN | 0 | STOREW | 216 | 
| leaf | DivFIN | 0 | DIV | 128 | 
| leaf | For | 0 | ADD | 401,831 | 
| leaf | For | 0 | BNE | 446,662 | 
| leaf | For | 0 | JAL | 44,831 | 
| leaf | For | 0 | LOADW | 2,856 | 
| leaf | For | 0 | STOREW | 41,975 | 
| leaf | FriReducedOpening | 0 | FRI_REDUCED_OPENING | 5,334 | 
| leaf | HintBitsF | 0 | PHANTOM | 43 | 
| leaf | HintInputVec | 0 | PHANTOM | 23,212 | 
| leaf | IfEq | 0 | BNE | 29,668 | 
| leaf | IfEqI | 0 | BNE | 129,806 | 
| leaf | IfEqI | 0 | JAL | 30,433 | 
| leaf | IfNe | 0 | BEQ | 15,767 | 
| leaf | IfNe | 0 | JAL | 1 | 
| leaf | IfNeI | 0 | BEQ | 2,915 | 
| leaf | ImmE | 0 | STOREW | 3,208 | 
| leaf | ImmF | 0 | STOREW | 46,741 | 
| leaf | ImmV | 0 | STOREW | 28,741 | 
| leaf | LoadE | 0 | ADD | 16,446 | 
| leaf | LoadE | 0 | LOADW | 87,432 | 
| leaf | LoadE | 0 | MUL | 16,446 | 
| leaf | LoadF | 0 | ADD | 80,592 | 
| leaf | LoadF | 0 | LOADW | 109,854 | 
| leaf | LoadF | 0 | MUL | 14,771 | 
| leaf | LoadHeapPtr | 0 | ADD | 1 | 
| leaf | LoadV | 0 | ADD | 202,296 | 
| leaf | LoadV | 0 | LOADW | 228,582 | 
| leaf | LoadV | 0 | MUL | 111,498 | 
| leaf | MulE | 0 | BBE4MUL | 8,535 | 
| leaf | MulEF | 0 | MUL | 3,792 | 
| leaf | MulEFI | 0 | MUL | 500 | 
| leaf | MulEI | 0 | BBE4MUL | 1,526 | 
| leaf | MulEI | 0 | STOREW | 6,104 | 
| leaf | MulF | 0 | MUL | 83,667 | 
| leaf | MulFI | 0 | MUL | 1,353 | 
| leaf | MulVI | 0 | MUL | 20,056 | 
| leaf | NegE | 0 | MUL | 172 | 
| leaf | Poseidon2CompressBabyBear | 0 | COMP_POS2 | 17,315 | 
| leaf | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | 9,249 | 
| leaf | Publish | 0 | PUBLISH | 36 | 
| leaf | StoreE | 0 | ADD | 8,588 | 
| leaf | StoreE | 0 | MUL | 8,588 | 
| leaf | StoreE | 0 | STOREW | 58,812 | 
| leaf | StoreF | 0 | ADD | 69,906 | 
| leaf | StoreF | 0 | MUL | 733 | 
| leaf | StoreF | 0 | STOREW | 109,184 | 
| leaf | StoreHeapPtr | 0 | ADD | 1 | 
| leaf | StoreHintWord | 0 | ADD | 205,099 | 
| leaf | StoreHintWord | 0 | SHINTW | 229,644 | 
| leaf | StoreV | 0 | ADD | 62,274 | 
| leaf | StoreV | 0 | MUL | 58,360 | 
| leaf | StoreV | 0 | STOREW | 65,208 | 
| leaf | SubE | 0 | FE4SUB | 3,306 | 
| leaf | SubEF | 0 | LOADW | 16,182 | 
| leaf | SubEF | 0 | SUB | 5,394 | 
| leaf | SubEFI | 0 | ADD | 344 | 
| leaf | SubEI | 0 | ADD | 432 | 
| leaf | SubFI | 0 | SUB | 1,333 | 
| leaf | SubV | 0 | SUB | 49,538 | 
| leaf | SubVI | 0 | SUB | 1,000 | 
| leaf | SubVIN | 0 | SUB | 840 | 
| leaf | UnsafeCastVF | 0 | ADD | 20 | 

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
| leaf | 0 | 2,499 | 28,272 | 3,985,751 | 343,345,624 | 11,337 | 2,655 | 2,013 | 1,836 | 2,388 | 2,177 | 138,441,657 | 266 | 14,436 | 

| group | segment | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_program | 0 | 925 | 11,050 | 1,500,137 | 197,453,854 | 5,091 | 794 | 501 | 1,751 | 1,047 | 817 | 51,503,940 | 179 | 5,034 | 

</details>


<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-fibonacci_program.dsl_ir.opcode.frequency.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e/fibonacci-6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e-leaf.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/6c9908aacda3816bf0b0c24b3ec2102f12d6cd0e

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12665017863)
