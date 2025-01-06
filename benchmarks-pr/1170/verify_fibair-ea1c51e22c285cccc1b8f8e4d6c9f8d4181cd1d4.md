| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+1 [+50.1%])</span> 2.22 | <span style='color: red'>(+1 [+50.1%])</span> 2.22 |
| verify_fibair | <span style='color: red'>(+1 [+50.1%])</span> 2.22 | <span style='color: red'>(+1 [+50.1%])</span> 2.22 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+741 [+50.1%])</span> 2,219 | <span style='color: red'>(+741 [+50.1%])</span> 2,219 | <span style='color: red'>(+741 [+50.1%])</span> 2,219 | <span style='color: red'>(+741 [+50.1%])</span> 2,219 |
| `main_cells_used     ` |  8,010,832 |  8,010,832 |  8,010,832 |  8,010,832 |
| `total_cycles        ` |  194,672 |  194,672 |  194,672 |  194,672 |
| `execute_time_ms     ` | <span style='color: red'>(+661 [+826.2%])</span> 741 | <span style='color: red'>(+661 [+826.2%])</span> 741 | <span style='color: red'>(+661 [+826.2%])</span> 741 | <span style='color: red'>(+661 [+826.2%])</span> 741 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+6 [+14.0%])</span> 49 | <span style='color: red'>(+6 [+14.0%])</span> 49 | <span style='color: red'>(+6 [+14.0%])</span> 49 | <span style='color: red'>(+6 [+14.0%])</span> 49 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+74 [+5.5%])</span> 1,429 | <span style='color: red'>(+74 [+5.5%])</span> 1,429 | <span style='color: red'>(+74 [+5.5%])</span> 1,429 | <span style='color: red'>(+74 [+5.5%])</span> 1,429 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+17 [+8.3%])</span> 221 | <span style='color: red'>(+17 [+8.3%])</span> 221 | <span style='color: red'>(+17 [+8.3%])</span> 221 | <span style='color: red'>(+17 [+8.3%])</span> 221 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+7 [+30.4%])</span> 30 | <span style='color: red'>(+7 [+30.4%])</span> 30 | <span style='color: red'>(+7 [+30.4%])</span> 30 | <span style='color: red'>(+7 [+30.4%])</span> 30 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+19 [+10.6%])</span> 198 | <span style='color: red'>(+19 [+10.6%])</span> 198 | <span style='color: red'>(+19 [+10.6%])</span> 198 | <span style='color: red'>(+19 [+10.6%])</span> 198 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+2 [+0.8%])</span> 258 | <span style='color: red'>(+2 [+0.8%])</span> 258 | <span style='color: red'>(+2 [+0.8%])</span> 258 | <span style='color: red'>(+2 [+0.8%])</span> 258 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+14 [+5.7%])</span> 261 | <span style='color: red'>(+14 [+5.7%])</span> 261 | <span style='color: red'>(+14 [+5.7%])</span> 261 | <span style='color: red'>(+14 [+5.7%])</span> 261 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+14 [+3.2%])</span> 458 | <span style='color: red'>(+14 [+3.2%])</span> 458 | <span style='color: red'>(+14 [+3.2%])</span> 458 | <span style='color: red'>(+14 [+3.2%])</span> 458 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 4 | 32 | 12 | 0 | 1 | 0 | 4 | 5 | 

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
| verify_fibair | 49 | 2,219 | 194,672 | 23,304,216 | 1,429 | 258 | 261 | 198 | 458 | 221 | 8,010,832 | 30 | 741 | 

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

| group | air_name | cycle_tracker_span | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | AssertEqE | BNE | 3,956 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | AssertEqEI | BNE | 92 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | AssertEqF | BNE | 78,016 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | AssertEqV | BNE | 4,071 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | AssertEqVI | BNE | 460 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | For | BNE | 486,864 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | IfEq | BNE | 7,383 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | IfEqI | BNE | 88,389 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | IfNe | BEQ | 26,749 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | IfNeI | BEQ | 5,865 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  |  | JAL | 10 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | For | JAL | 41,010 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | IfEqI | JAL | 9,840 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | IfNe | JAL | 20 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | AddEI | ADD | 77,520 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | AddF | ADD | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | AddFI | ADD | 19,440 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | AddV | ADD | 28,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | AddVI | ADD | 479,190 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | Alloc | ADD | 180,810 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | Alloc | MUL | 121,920 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DivFIN | DIV | 90 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | For | ADD | 512,010 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | LoadHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MulEF | MUL | 20,400 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MulF | MUL | 72,870 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MulFI | MUL | 40,020 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MulVI | MUL | 40,440 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | StoreHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | StoreHintWord | ADD | 310,650 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SubEF | SUB | 3,930 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SubEI | ADD | 240 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SubFI | SUB | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SubV | SUB | 42,870 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SubVI | SUB | 7,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SubVIN | SUB | 5,040 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | UnsafeCastVF | ADD | 30 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  |  | STOREW | 41 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | AddEFFI | LOADW | 2,870 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | AddEFFI | STOREW | 8,610 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | Alloc | LOADW | 247,107 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | DivEIN | STOREW | 164 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | For | LOADW | 12,054 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | For | STOREW | 156,087 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | ImmE | STOREW | 34,768 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | ImmF | STOREW | 177,735 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | ImmV | STOREW | 169,289 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LoadE | LOADW | 84,132 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LoadE | LOADW2 | 185,976 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LoadF | LOADW | 284,089 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LoadF | LOADW2 | 39,852 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LoadV | LOADW | 109,511 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LoadV | LOADW2 | 354,281 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | MulEI | STOREW | 5,412 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | StoreE | STOREW | 551,696 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | StoreE | STOREW2 | 83,312 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | StoreF | STOREW | 113,652 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | StoreF | STOREW2 | 68,839 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | StoreHintWord | SHINTW | 559,691 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | StoreV | STOREW | 23,206 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | StoreV | STOREW2 | 208,731 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SubEF | LOADW | 16,113 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | AddE | FE4ADD | 19,680 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | DivE | BBE4DIV | 11,840 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | DivEIN | BBE4DIV | 40 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | MulE | BBE4MUL | 34,320 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | MulEI | BBE4MUL | 1,320 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | SubE | FE4SUB | 20,240 | 
| verify_fibair | AccessAdapter<2> |  | AddE | FE4ADD | 10,824 | 
| verify_fibair | AccessAdapter<2> |  | AddEFFI | LOADW | 132 | 
| verify_fibair | AccessAdapter<2> |  | AddEFFI | STOREW | 132 | 
| verify_fibair | AccessAdapter<2> |  | AddEI | ADD | 12,716 | 
| verify_fibair | AccessAdapter<2> |  | Alloc | MUL | 33 | 
| verify_fibair | AccessAdapter<2> |  | AssertEqE | BNE | 946 | 
| verify_fibair | AccessAdapter<2> |  | DivE | BBE4DIV | 3,784 | 
| verify_fibair | AccessAdapter<2> |  | DivEIN | BBE4DIV | 22 | 
| verify_fibair | AccessAdapter<2> |  | DivEIN | STOREW | 11 | 
| verify_fibair | AccessAdapter<2> |  | FriReducedOpening | FRI_REDUCED_OPENING | 2,024 | 
| verify_fibair | AccessAdapter<2> |  | ImmE | STOREW | 946 | 
| verify_fibair | AccessAdapter<2> |  | LoadE | LOADW | 7,656 | 
| verify_fibair | AccessAdapter<2> |  | LoadE | LOADW2 | 12,936 | 
| verify_fibair | AccessAdapter<2> |  | LoadF | LOADW | 11,088 | 
| verify_fibair | AccessAdapter<2> |  | LoadF | LOADW2 | 396 | 
| verify_fibair | AccessAdapter<2> |  | MulE | BBE4MUL | 13,750 | 
| verify_fibair | AccessAdapter<2> |  | MulEF | MUL | 3,696 | 
| verify_fibair | AccessAdapter<2> |  | MulEI | BBE4MUL | 1,892 | 
| verify_fibair | AccessAdapter<2> |  | MulEI | STOREW | 682 | 
| verify_fibair | AccessAdapter<2> |  | Poseidon2CompressBabyBear | COMP_POS2 | 48,048 | 
| verify_fibair | AccessAdapter<2> |  | Poseidon2PermuteBabyBear | PERM_POS2 | 22,770 | 
| verify_fibair | AccessAdapter<2> |  | StoreE | STOREW | 3,696 | 
| verify_fibair | AccessAdapter<2> |  | StoreE | STOREW2 | 3,696 | 
| verify_fibair | AccessAdapter<2> |  | StoreF | STOREW2 | 132 | 
| verify_fibair | AccessAdapter<2> |  | SubE | FE4SUB | 18,656 | 
| verify_fibair | AccessAdapter<2> |  | SubEF | LOADW | 946 | 
| verify_fibair | AccessAdapter<2> |  | SubEF | SUB | 946 | 
| verify_fibair | AccessAdapter<2> |  | SubEI | ADD | 44 | 
| verify_fibair | AccessAdapter<4> |  | AddE | FE4ADD | 6,396 | 
| verify_fibair | AccessAdapter<4> |  | AddEFFI | LOADW | 156 | 
| verify_fibair | AccessAdapter<4> |  | AddEI | ADD | 7,514 | 
| verify_fibair | AccessAdapter<4> |  | Alloc | MUL | 39 | 
| verify_fibair | AccessAdapter<4> |  | AssertEqE | BNE | 559 | 
| verify_fibair | AccessAdapter<4> |  | DivE | BBE4DIV | 2,236 | 
| verify_fibair | AccessAdapter<4> |  | DivEIN | BBE4DIV | 13 | 
| verify_fibair | AccessAdapter<4> |  | FriReducedOpening | FRI_REDUCED_OPENING | 1,196 | 
| verify_fibair | AccessAdapter<4> |  | ImmE | STOREW | 559 | 
| verify_fibair | AccessAdapter<4> |  | LoadE | LOADW | 4,524 | 
| verify_fibair | AccessAdapter<4> |  | LoadE | LOADW2 | 7,644 | 
| verify_fibair | AccessAdapter<4> |  | LoadF | LOADW | 6,552 | 
| verify_fibair | AccessAdapter<4> |  | LoadF | LOADW2 | 234 | 
| verify_fibair | AccessAdapter<4> |  | MulE | BBE4MUL | 8,125 | 
| verify_fibair | AccessAdapter<4> |  | MulEF | MUL | 2,184 | 
| verify_fibair | AccessAdapter<4> |  | MulEI | BBE4MUL | 1,118 | 
| verify_fibair | AccessAdapter<4> |  | MulEI | STOREW | 390 | 
| verify_fibair | AccessAdapter<4> |  | Poseidon2CompressBabyBear | COMP_POS2 | 28,392 | 
| verify_fibair | AccessAdapter<4> |  | Poseidon2PermuteBabyBear | PERM_POS2 | 13,455 | 
| verify_fibair | AccessAdapter<4> |  | StoreE | STOREW | 2,184 | 
| verify_fibair | AccessAdapter<4> |  | StoreE | STOREW2 | 2,184 | 
| verify_fibair | AccessAdapter<4> |  | StoreF | STOREW2 | 78 | 
| verify_fibair | AccessAdapter<4> |  | SubE | FE4SUB | 11,024 | 
| verify_fibair | AccessAdapter<4> |  | SubEF | SUB | 1,118 | 
| verify_fibair | AccessAdapter<4> |  | SubEI | ADD | 26 | 
| verify_fibair | AccessAdapter<8> |  | LoadF | LOADW | 4,284 | 
| verify_fibair | AccessAdapter<8> |  | LoadF | LOADW2 | 204 | 
| verify_fibair | AccessAdapter<8> |  | Poseidon2CompressBabyBear | COMP_POS2 | 18,564 | 
| verify_fibair | AccessAdapter<8> |  | Poseidon2PermuteBabyBear | PERM_POS2 | 8,806 | 
| verify_fibair | AccessAdapter<8> |  | StoreF | STOREW2 | 17 | 
| verify_fibair | Arc<BabyBearParameters>, 1> |  | Poseidon2CompressBabyBear | COMP_POS2 | 380,016 | 
| verify_fibair | Arc<BabyBearParameters>, 1> |  | Poseidon2PermuteBabyBear | PERM_POS2 | 92,220 | 
| verify_fibair | Boundary |  |  | JAL | 11 | 
| verify_fibair | Boundary |  |  | STOREW | 11 | 
| verify_fibair | Boundary |  | AddE | FE4ADD | 748 | 
| verify_fibair | Boundary |  | AddEFFI | LOADW | 176 | 
| verify_fibair | Boundary |  | AddEFFI | STOREW | 528 | 
| verify_fibair | Boundary |  | AddEI | ADD | 440 | 
| verify_fibair | Boundary |  | AddFI | ADD | 231 | 
| verify_fibair | Boundary |  | AddV | ADD | 33 | 
| verify_fibair | Boundary |  | AddVI | ADD | 1,023 | 
| verify_fibair | Boundary |  | Alloc | LOADW | 649 | 
| verify_fibair | Boundary |  | DivE | BBE4DIV | 44 | 
| verify_fibair | Boundary |  | For | LOADW | 22 | 
| verify_fibair | Boundary |  | For | STOREW | 451 | 
| verify_fibair | Boundary |  | ImmE | STOREW | 44 | 
| verify_fibair | Boundary |  | ImmF | STOREW | 1,782 | 
| verify_fibair | Boundary |  | ImmV | STOREW | 682 | 
| verify_fibair | Boundary |  | LoadE | LOADW | 220 | 
| verify_fibair | Boundary |  | LoadF | LOADW | 979 | 
| verify_fibair | Boundary |  | LoadF | LOADW2 | 231 | 
| verify_fibair | Boundary |  | LoadHeapPtr | ADD | 11 | 
| verify_fibair | Boundary |  | LoadV | LOADW | 374 | 
| verify_fibair | Boundary |  | LoadV | LOADW2 | 946 | 
| verify_fibair | Boundary |  | MulE | BBE4MUL | 528 | 
| verify_fibair | Boundary |  | MulEF | MUL | 88 | 
| verify_fibair | Boundary |  | MulEI | BBE4MUL | 924 | 
| verify_fibair | Boundary |  | MulEI | STOREW | 33 | 
| verify_fibair | Boundary |  | MulF | MUL | 649 | 
| verify_fibair | Boundary |  | MulFI | MUL | 660 | 
| verify_fibair | Boundary |  | MulVI | MUL | 33 | 
| verify_fibair | Boundary |  | StoreE | STOREW | 148,016 | 
| verify_fibair | Boundary |  | StoreE | STOREW2 | 7,568 | 
| verify_fibair | Boundary |  | StoreF | STOREW | 30,492 | 
| verify_fibair | Boundary |  | StoreF | STOREW2 | 15,070 | 
| verify_fibair | Boundary |  | StoreHintWord | SHINTW | 150,161 | 
| verify_fibair | Boundary |  | StoreV | STOREW | 6,226 | 
| verify_fibair | Boundary |  | StoreV | STOREW2 | 44,286 | 
| verify_fibair | Boundary |  | SubE | FE4SUB | 220 | 
| verify_fibair | Boundary |  | SubEF | LOADW | 99 | 
| verify_fibair | Boundary |  | SubEF | SUB | 33 | 
| verify_fibair | Boundary |  | SubFI | SUB | 649 | 
| verify_fibair | Boundary |  | SubV | SUB | 44 | 
| verify_fibair | Boundary |  | SubVI | SUB | 99 | 
| verify_fibair | FriReducedOpeningAir |  | FriReducedOpening | FRI_REDUCED_OPENING | 21,504 | 
| verify_fibair | PhantomAir |  | HintBitsF | PHANTOM | 258 | 
| verify_fibair | PhantomAir |  | HintInputVec | PHANTOM | 11,778 | 

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | 3,956 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | 92 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | 78,016 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | 4,071 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | 460 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | 486,864 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | 7,383 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | 88,389 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | 26,749 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | 5,865 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 10 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | For | JAL | 41,010 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | 9,840 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | 20 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | 77,520 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | 19,440 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | 28,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | 479,190 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | 180,810 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | 121,920 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | 90 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | 512,010 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | 20,400 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | 72,870 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | 40,020 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | 40,440 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | 310,650 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | 3,930 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | 240 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | SUB | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | 42,870 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | 7,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | 5,040 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | ADD | 30 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 41 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | LOADW | 2,870 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | STOREW | 8,610 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | LOADW | 247,107 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | DivEIN | STOREW | 164 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | LOADW | 12,054 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | STOREW | 156,087 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | STOREW | 34,768 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | STOREW | 177,735 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | STOREW | 169,289 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW | 84,132 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW2 | 185,976 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW | 284,089 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW2 | 39,852 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW | 109,511 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW2 | 354,281 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | STOREW | 5,412 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW | 551,696 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW2 | 83,312 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW | 113,652 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW2 | 68,839 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | 559,691 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW | 23,206 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW2 | 208,731 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | SubEF | LOADW | 16,113 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | 19,680 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | 11,840 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | 40 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | 34,320 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | 1,320 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | 20,240 | 
| verify_fibair | AccessAdapter<2> | AddE | FE4ADD | 10,824 | 
| verify_fibair | AccessAdapter<2> | AddEFFI | LOADW | 132 | 
| verify_fibair | AccessAdapter<2> | AddEFFI | STOREW | 132 | 
| verify_fibair | AccessAdapter<2> | AddEI | ADD | 12,716 | 
| verify_fibair | AccessAdapter<2> | Alloc | MUL | 33 | 
| verify_fibair | AccessAdapter<2> | AssertEqE | BNE | 946 | 
| verify_fibair | AccessAdapter<2> | DivE | BBE4DIV | 3,784 | 
| verify_fibair | AccessAdapter<2> | DivEIN | BBE4DIV | 22 | 
| verify_fibair | AccessAdapter<2> | DivEIN | STOREW | 11 | 
| verify_fibair | AccessAdapter<2> | FriReducedOpening | FRI_REDUCED_OPENING | 2,024 | 
| verify_fibair | AccessAdapter<2> | ImmE | STOREW | 946 | 
| verify_fibair | AccessAdapter<2> | LoadE | LOADW | 7,656 | 
| verify_fibair | AccessAdapter<2> | LoadE | LOADW2 | 12,936 | 
| verify_fibair | AccessAdapter<2> | LoadF | LOADW | 11,088 | 
| verify_fibair | AccessAdapter<2> | LoadF | LOADW2 | 396 | 
| verify_fibair | AccessAdapter<2> | MulE | BBE4MUL | 13,750 | 
| verify_fibair | AccessAdapter<2> | MulEF | MUL | 3,696 | 
| verify_fibair | AccessAdapter<2> | MulEI | BBE4MUL | 1,892 | 
| verify_fibair | AccessAdapter<2> | MulEI | STOREW | 682 | 
| verify_fibair | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | 48,048 | 
| verify_fibair | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | 22,770 | 
| verify_fibair | AccessAdapter<2> | StoreE | STOREW | 3,696 | 
| verify_fibair | AccessAdapter<2> | StoreE | STOREW2 | 3,696 | 
| verify_fibair | AccessAdapter<2> | StoreF | STOREW2 | 132 | 
| verify_fibair | AccessAdapter<2> | SubE | FE4SUB | 18,656 | 
| verify_fibair | AccessAdapter<2> | SubEF | LOADW | 946 | 
| verify_fibair | AccessAdapter<2> | SubEF | SUB | 946 | 
| verify_fibair | AccessAdapter<2> | SubEI | ADD | 44 | 
| verify_fibair | AccessAdapter<4> | AddE | FE4ADD | 6,396 | 
| verify_fibair | AccessAdapter<4> | AddEFFI | LOADW | 156 | 
| verify_fibair | AccessAdapter<4> | AddEI | ADD | 7,514 | 
| verify_fibair | AccessAdapter<4> | Alloc | MUL | 39 | 
| verify_fibair | AccessAdapter<4> | AssertEqE | BNE | 559 | 
| verify_fibair | AccessAdapter<4> | DivE | BBE4DIV | 2,236 | 
| verify_fibair | AccessAdapter<4> | DivEIN | BBE4DIV | 13 | 
| verify_fibair | AccessAdapter<4> | FriReducedOpening | FRI_REDUCED_OPENING | 1,196 | 
| verify_fibair | AccessAdapter<4> | ImmE | STOREW | 559 | 
| verify_fibair | AccessAdapter<4> | LoadE | LOADW | 4,524 | 
| verify_fibair | AccessAdapter<4> | LoadE | LOADW2 | 7,644 | 
| verify_fibair | AccessAdapter<4> | LoadF | LOADW | 6,552 | 
| verify_fibair | AccessAdapter<4> | LoadF | LOADW2 | 234 | 
| verify_fibair | AccessAdapter<4> | MulE | BBE4MUL | 8,125 | 
| verify_fibair | AccessAdapter<4> | MulEF | MUL | 2,184 | 
| verify_fibair | AccessAdapter<4> | MulEI | BBE4MUL | 1,118 | 
| verify_fibair | AccessAdapter<4> | MulEI | STOREW | 390 | 
| verify_fibair | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | 28,392 | 
| verify_fibair | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | 13,455 | 
| verify_fibair | AccessAdapter<4> | StoreE | STOREW | 2,184 | 
| verify_fibair | AccessAdapter<4> | StoreE | STOREW2 | 2,184 | 
| verify_fibair | AccessAdapter<4> | StoreF | STOREW2 | 78 | 
| verify_fibair | AccessAdapter<4> | SubE | FE4SUB | 11,024 | 
| verify_fibair | AccessAdapter<4> | SubEF | SUB | 1,118 | 
| verify_fibair | AccessAdapter<4> | SubEI | ADD | 26 | 
| verify_fibair | AccessAdapter<8> | LoadF | LOADW | 4,284 | 
| verify_fibair | AccessAdapter<8> | LoadF | LOADW2 | 204 | 
| verify_fibair | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | 18,564 | 
| verify_fibair | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | 8,806 | 
| verify_fibair | AccessAdapter<8> | StoreF | STOREW2 | 17 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2CompressBabyBear | COMP_POS2 | 380,016 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | PERM_POS2 | 92,220 | 
| verify_fibair | Boundary |  | JAL | 11 | 
| verify_fibair | Boundary |  | STOREW | 11 | 
| verify_fibair | Boundary | AddE | FE4ADD | 748 | 
| verify_fibair | Boundary | AddEFFI | LOADW | 176 | 
| verify_fibair | Boundary | AddEFFI | STOREW | 528 | 
| verify_fibair | Boundary | AddEI | ADD | 440 | 
| verify_fibair | Boundary | AddFI | ADD | 231 | 
| verify_fibair | Boundary | AddV | ADD | 33 | 
| verify_fibair | Boundary | AddVI | ADD | 1,023 | 
| verify_fibair | Boundary | Alloc | LOADW | 649 | 
| verify_fibair | Boundary | DivE | BBE4DIV | 44 | 
| verify_fibair | Boundary | For | LOADW | 22 | 
| verify_fibair | Boundary | For | STOREW | 451 | 
| verify_fibair | Boundary | ImmE | STOREW | 44 | 
| verify_fibair | Boundary | ImmF | STOREW | 1,782 | 
| verify_fibair | Boundary | ImmV | STOREW | 682 | 
| verify_fibair | Boundary | LoadE | LOADW | 220 | 
| verify_fibair | Boundary | LoadF | LOADW | 979 | 
| verify_fibair | Boundary | LoadF | LOADW2 | 231 | 
| verify_fibair | Boundary | LoadHeapPtr | ADD | 11 | 
| verify_fibair | Boundary | LoadV | LOADW | 374 | 
| verify_fibair | Boundary | LoadV | LOADW2 | 946 | 
| verify_fibair | Boundary | MulE | BBE4MUL | 528 | 
| verify_fibair | Boundary | MulEF | MUL | 88 | 
| verify_fibair | Boundary | MulEI | BBE4MUL | 924 | 
| verify_fibair | Boundary | MulEI | STOREW | 33 | 
| verify_fibair | Boundary | MulF | MUL | 649 | 
| verify_fibair | Boundary | MulFI | MUL | 660 | 
| verify_fibair | Boundary | MulVI | MUL | 33 | 
| verify_fibair | Boundary | StoreE | STOREW | 148,016 | 
| verify_fibair | Boundary | StoreE | STOREW2 | 7,568 | 
| verify_fibair | Boundary | StoreF | STOREW | 30,492 | 
| verify_fibair | Boundary | StoreF | STOREW2 | 15,070 | 
| verify_fibair | Boundary | StoreHintWord | SHINTW | 150,161 | 
| verify_fibair | Boundary | StoreV | STOREW | 6,226 | 
| verify_fibair | Boundary | StoreV | STOREW2 | 44,286 | 
| verify_fibair | Boundary | SubE | FE4SUB | 220 | 
| verify_fibair | Boundary | SubEF | LOADW | 99 | 
| verify_fibair | Boundary | SubEF | SUB | 33 | 
| verify_fibair | Boundary | SubFI | SUB | 649 | 
| verify_fibair | Boundary | SubV | SUB | 44 | 
| verify_fibair | Boundary | SubVI | SUB | 99 | 
| verify_fibair | FriReducedOpeningAir | FriReducedOpening | FRI_REDUCED_OPENING | 21,504 | 
| verify_fibair | PhantomAir | HintBitsF | PHANTOM | 258 | 
| verify_fibair | PhantomAir | HintInputVec | PHANTOM | 11,778 | 

| group | chip_name | rows_used |
| --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 30,515 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | 5,088 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 68,095 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 85,299 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2,186 | 
| verify_fibair | AccessAdapter<2> | 22,140 | 
| verify_fibair | AccessAdapter<4> | 11,070 | 
| verify_fibair | AccessAdapter<8> | 3,220 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | 1,357 | 
| verify_fibair | Boundary | 37,775 | 
| verify_fibair | FriReducedOpeningAir | 336 | 
| verify_fibair | PhantomAir | 2,006 | 
| verify_fibair | ProgramChip | 4,915 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 
| verify_fibair | VmConnectorAir | 2 | 

| group | cycle_tracker_span | dsl_ir | opcode | frequency |
| --- | --- | --- | --- | --- |
| verify_fibair |  |  | JAL | 1 | 
| verify_fibair |  |  | STOREW | 2 | 
| verify_fibair |  | AddE | FE4ADD | 492 | 
| verify_fibair |  | AddEFFI | LOADW | 70 | 
| verify_fibair |  | AddEFFI | STOREW | 210 | 
| verify_fibair |  | AddEI | ADD | 2,584 | 
| verify_fibair |  | AddF | ADD | 1,333 | 
| verify_fibair |  | AddFI | ADD | 648 | 
| verify_fibair |  | AddV | ADD | 939 | 
| verify_fibair |  | AddVI | ADD | 15,973 | 
| verify_fibair |  | Alloc | ADD | 6,027 | 
| verify_fibair |  | Alloc | LOADW | 6,027 | 
| verify_fibair |  | Alloc | MUL | 4,064 | 
| verify_fibair |  | AssertEqE | BNE | 172 | 
| verify_fibair |  | AssertEqEI | BNE | 4 | 
| verify_fibair |  | AssertEqF | BNE | 3,392 | 
| verify_fibair |  | AssertEqV | BNE | 177 | 
| verify_fibair |  | AssertEqVI | BNE | 20 | 
| verify_fibair |  | DivE | BBE4DIV | 296 | 
| verify_fibair |  | DivEIN | BBE4DIV | 1 | 
| verify_fibair |  | DivEIN | STOREW | 4 | 
| verify_fibair |  | DivFIN | DIV | 3 | 
| verify_fibair |  | For | ADD | 17,067 | 
| verify_fibair |  | For | BNE | 21,168 | 
| verify_fibair |  | For | JAL | 4,101 | 
| verify_fibair |  | For | LOADW | 294 | 
| verify_fibair |  | For | STOREW | 3,807 | 
| verify_fibair |  | FriReducedOpening | FRI_REDUCED_OPENING | 126 | 
| verify_fibair |  | HintBitsF | PHANTOM | 43 | 
| verify_fibair |  | HintInputVec | PHANTOM | 1,963 | 
| verify_fibair |  | IfEq | BNE | 321 | 
| verify_fibair |  | IfEqI | BNE | 3,843 | 
| verify_fibair |  | IfEqI | JAL | 984 | 
| verify_fibair |  | IfNe | BEQ | 1,163 | 
| verify_fibair |  | IfNe | JAL | 2 | 
| verify_fibair |  | IfNeI | BEQ | 255 | 
| verify_fibair |  | ImmE | STOREW | 848 | 
| verify_fibair |  | ImmF | STOREW | 4,335 | 
| verify_fibair |  | ImmV | STOREW | 4,129 | 
| verify_fibair |  | LoadE | LOADW | 2,052 | 
| verify_fibair |  | LoadE | LOADW2 | 4,536 | 
| verify_fibair |  | LoadF | LOADW | 6,929 | 
| verify_fibair |  | LoadF | LOADW2 | 972 | 
| verify_fibair |  | LoadHeapPtr | ADD | 1 | 
| verify_fibair |  | LoadV | LOADW | 2,671 | 
| verify_fibair |  | LoadV | LOADW2 | 8,641 | 
| verify_fibair |  | MulE | BBE4MUL | 858 | 
| verify_fibair |  | MulEF | MUL | 680 | 
| verify_fibair |  | MulEI | BBE4MUL | 33 | 
| verify_fibair |  | MulEI | STOREW | 132 | 
| verify_fibair |  | MulF | MUL | 2,429 | 
| verify_fibair |  | MulFI | MUL | 1,334 | 
| verify_fibair |  | MulVI | MUL | 1,348 | 
| verify_fibair |  | Poseidon2CompressBabyBear | COMP_POS2 | 1,092 | 
| verify_fibair |  | Poseidon2PermuteBabyBear | PERM_POS2 | 265 | 
| verify_fibair |  | StoreE | STOREW | 13,456 | 
| verify_fibair |  | StoreE | STOREW2 | 2,032 | 
| verify_fibair |  | StoreF | STOREW | 2,772 | 
| verify_fibair |  | StoreF | STOREW2 | 1,679 | 
| verify_fibair |  | StoreHeapPtr | ADD | 1 | 
| verify_fibair |  | StoreHintWord | ADD | 10,355 | 
| verify_fibair |  | StoreHintWord | SHINTW | 13,651 | 
| verify_fibair |  | StoreV | STOREW | 566 | 
| verify_fibair |  | StoreV | STOREW2 | 5,091 | 
| verify_fibair |  | SubE | FE4SUB | 506 | 
| verify_fibair |  | SubEF | LOADW | 393 | 
| verify_fibair |  | SubEF | SUB | 131 | 
| verify_fibair |  | SubEI | ADD | 8 | 
| verify_fibair |  | SubFI | SUB | 1,333 | 
| verify_fibair |  | SubV | SUB | 1,429 | 
| verify_fibair |  | SubVI | SUB | 239 | 
| verify_fibair |  | SubVIN | SUB | 168 | 
| verify_fibair |  | UnsafeCastVF | ADD | 1 | 

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| verify_fibair |  | JAL | 1 | 
| verify_fibair |  | STOREW | 2 | 
| verify_fibair | AddE | FE4ADD | 492 | 
| verify_fibair | AddEFFI | LOADW | 70 | 
| verify_fibair | AddEFFI | STOREW | 210 | 
| verify_fibair | AddEI | ADD | 2,584 | 
| verify_fibair | AddF | ADD | 1,333 | 
| verify_fibair | AddFI | ADD | 648 | 
| verify_fibair | AddV | ADD | 939 | 
| verify_fibair | AddVI | ADD | 15,973 | 
| verify_fibair | Alloc | ADD | 6,027 | 
| verify_fibair | Alloc | LOADW | 6,027 | 
| verify_fibair | Alloc | MUL | 4,064 | 
| verify_fibair | AssertEqE | BNE | 172 | 
| verify_fibair | AssertEqEI | BNE | 4 | 
| verify_fibair | AssertEqF | BNE | 3,392 | 
| verify_fibair | AssertEqV | BNE | 177 | 
| verify_fibair | AssertEqVI | BNE | 20 | 
| verify_fibair | DivE | BBE4DIV | 296 | 
| verify_fibair | DivEIN | BBE4DIV | 1 | 
| verify_fibair | DivEIN | STOREW | 4 | 
| verify_fibair | DivFIN | DIV | 3 | 
| verify_fibair | For | ADD | 17,067 | 
| verify_fibair | For | BNE | 21,168 | 
| verify_fibair | For | JAL | 4,101 | 
| verify_fibair | For | LOADW | 294 | 
| verify_fibair | For | STOREW | 3,807 | 
| verify_fibair | FriReducedOpening | FRI_REDUCED_OPENING | 126 | 
| verify_fibair | HintBitsF | PHANTOM | 43 | 
| verify_fibair | HintInputVec | PHANTOM | 1,963 | 
| verify_fibair | IfEq | BNE | 321 | 
| verify_fibair | IfEqI | BNE | 3,843 | 
| verify_fibair | IfEqI | JAL | 984 | 
| verify_fibair | IfNe | BEQ | 1,163 | 
| verify_fibair | IfNe | JAL | 2 | 
| verify_fibair | IfNeI | BEQ | 255 | 
| verify_fibair | ImmE | STOREW | 848 | 
| verify_fibair | ImmF | STOREW | 4,335 | 
| verify_fibair | ImmV | STOREW | 4,129 | 
| verify_fibair | LoadE | LOADW | 2,052 | 
| verify_fibair | LoadE | LOADW2 | 4,536 | 
| verify_fibair | LoadF | LOADW | 6,929 | 
| verify_fibair | LoadF | LOADW2 | 972 | 
| verify_fibair | LoadHeapPtr | ADD | 1 | 
| verify_fibair | LoadV | LOADW | 2,671 | 
| verify_fibair | LoadV | LOADW2 | 8,641 | 
| verify_fibair | MulE | BBE4MUL | 858 | 
| verify_fibair | MulEF | MUL | 680 | 
| verify_fibair | MulEI | BBE4MUL | 33 | 
| verify_fibair | MulEI | STOREW | 132 | 
| verify_fibair | MulF | MUL | 2,429 | 
| verify_fibair | MulFI | MUL | 1,334 | 
| verify_fibair | MulVI | MUL | 1,348 | 
| verify_fibair | Poseidon2CompressBabyBear | COMP_POS2 | 1,092 | 
| verify_fibair | Poseidon2PermuteBabyBear | PERM_POS2 | 265 | 
| verify_fibair | StoreE | STOREW | 13,456 | 
| verify_fibair | StoreE | STOREW2 | 2,032 | 
| verify_fibair | StoreF | STOREW | 2,772 | 
| verify_fibair | StoreF | STOREW2 | 1,679 | 
| verify_fibair | StoreHeapPtr | ADD | 1 | 
| verify_fibair | StoreHintWord | ADD | 10,355 | 
| verify_fibair | StoreHintWord | SHINTW | 13,651 | 
| verify_fibair | StoreV | STOREW | 566 | 
| verify_fibair | StoreV | STOREW2 | 5,091 | 
| verify_fibair | SubE | FE4SUB | 506 | 
| verify_fibair | SubEF | LOADW | 393 | 
| verify_fibair | SubEF | SUB | 131 | 
| verify_fibair | SubEI | ADD | 8 | 
| verify_fibair | SubFI | SUB | 1,333 | 
| verify_fibair | SubV | SUB | 1,429 | 
| verify_fibair | SubVI | SUB | 239 | 
| verify_fibair | SubVIN | SUB | 168 | 
| verify_fibair | UnsafeCastVF | ADD | 1 | 

</details>


Commit: https://github.com/openvm-org/openvm/commit/ea1c51e22c285cccc1b8f8e4d6c9f8d4181cd1d4

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12628869433)
