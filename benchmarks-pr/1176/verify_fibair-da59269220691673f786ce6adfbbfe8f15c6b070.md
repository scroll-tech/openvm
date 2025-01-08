| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+1 [+45.8%])</span> 2.13 | <span style='color: red'>(+1 [+45.8%])</span> 2.13 |
| verify_fibair | <span style='color: red'>(+1 [+45.8%])</span> 2.13 | <span style='color: red'>(+1 [+45.8%])</span> 2.13 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+669 [+45.8%])</span> 2,131 | <span style='color: red'>(+669 [+45.8%])</span> 2,131 | <span style='color: red'>(+669 [+45.8%])</span> 2,131 | <span style='color: red'>(+669 [+45.8%])</span> 2,131 |
| `main_cells_used     ` | <span style='color: red'>(+38823 [+0.5%])</span> 8,067,035 | <span style='color: red'>(+38823 [+0.5%])</span> 8,067,035 | <span style='color: red'>(+38823 [+0.5%])</span> 8,067,035 | <span style='color: red'>(+38823 [+0.5%])</span> 8,067,035 |
| `total_cycles        ` | <span style='color: red'>(+29773 [+15.3%])</span> 224,948 | <span style='color: red'>(+29773 [+15.3%])</span> 224,948 | <span style='color: red'>(+29773 [+15.3%])</span> 224,948 | <span style='color: red'>(+29773 [+15.3%])</span> 224,948 |
| `execute_time_ms     ` | <span style='color: red'>(+651 [+1050.0%])</span> 713 | <span style='color: red'>(+651 [+1050.0%])</span> 713 | <span style='color: red'>(+651 [+1050.0%])</span> 713 | <span style='color: red'>(+651 [+1050.0%])</span> 713 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+13 [+9.4%])</span> 152 | <span style='color: red'>(+13 [+9.4%])</span> 152 | <span style='color: red'>(+13 [+9.4%])</span> 152 | <span style='color: red'>(+13 [+9.4%])</span> 152 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+5 [+0.4%])</span> 1,266 | <span style='color: red'>(+5 [+0.4%])</span> 1,266 | <span style='color: red'>(+5 [+0.4%])</span> 1,266 | <span style='color: red'>(+5 [+0.4%])</span> 1,266 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-5 [-2.4%])</span> 206 | <span style='color: green'>(-5 [-2.4%])</span> 206 | <span style='color: green'>(-5 [-2.4%])</span> 206 | <span style='color: green'>(-5 [-2.4%])</span> 206 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-9 [-28.1%])</span> 23 | <span style='color: green'>(-9 [-28.1%])</span> 23 | <span style='color: green'>(-9 [-28.1%])</span> 23 | <span style='color: green'>(-9 [-28.1%])</span> 23 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+2 [+1.1%])</span> 188 | <span style='color: red'>(+2 [+1.1%])</span> 188 | <span style='color: red'>(+2 [+1.1%])</span> 188 | <span style='color: red'>(+2 [+1.1%])</span> 188 |
| `quotient_poly_compute_time_ms` |  223 |  223 |  223 |  223 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+16 [+6.5%])</span> 261 | <span style='color: red'>(+16 [+6.5%])</span> 261 | <span style='color: red'>(+16 [+6.5%])</span> 261 | <span style='color: red'>(+16 [+6.5%])</span> 261 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+2 [+0.6%])</span> 363 | <span style='color: red'>(+2 [+0.6%])</span> 363 | <span style='color: red'>(+2 [+0.6%])</span> 363 | <span style='color: red'>(+2 [+0.6%])</span> 363 |



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
| VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> |  | 4 |  | 15 | 24 |  | 
| VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> |  | 4 |  | 15 | 23 |  | 
| VmConnectorAir |  | 4 |  | 3 | 8 |  | 
| VolatileBoundaryAir |  | 4 |  | 4 | 16 |  | 

| group | trace_gen_time_ms | total_proof_time_ms | total_cycles | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms | main_cells_used | generate_perm_trace_time_ms | execute_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | 152 | 2,131 | 224,948 | 21,469,208 | 1,266 | 223 | 261 | 188 | 363 | 206 | 8,067,035 | 23 | 713 | 

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
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 131,072 |  | 20 | 31 | 6,684,672 | 
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 4,096 |  | 20 | 40 | 245,760 | 
| verify_fibair | VmConnectorAir | 2 | 1 | 8 | 4 | 24 | 
| verify_fibair | VolatileBoundaryAir | 65,536 |  | 8 | 11 | 1,245,184 | 

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | 3,956 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | 92 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | 78,016 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | 4,071 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | 460 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | 487,600 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | 7,383 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | 88,665 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | 26,749 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | 5,865 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 10 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | For | JAL | 41,050 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | 10,540 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | 20 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | 77,400 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | 19,800 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | 33,360 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | 479,970 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | 180,900 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | 122,010 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | 90 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | 512,850 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadE | ADD | 34,140 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadE | MUL | 34,140 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | ADD | 21,960 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | MUL | 15,480 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | ADD | 253,470 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | MUL | 175,260 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | 20,400 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | 72,870 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | 40,020 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | 43,050 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreE | ADD | 15,240 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreE | MUL | 15,240 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | ADD | 50,730 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | MUL | 300 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | 311,010 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | ADD | 152,730 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | MUL | 118,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | 3,930 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | 240 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | SUB | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | 42,870 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | 7,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | 5,040 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | ADD | 30 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 31 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | LOADW | 2,170 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | STOREW | 6,510 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | LOADW | 186,930 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | DivEIN | STOREW | 124 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | LOADW | 14,477 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | STOREW | 112,778 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | STOREW | 26,288 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | STOREW | 134,385 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | STOREW | 128,588 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW | 209,808 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW | 245,303 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW | 350,672 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | STOREW | 4,092 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW | 480,500 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW | 138,353 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | 423,553 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW | 175,367 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | SubEF | LOADW | 12,183 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | 19,680 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | 11,840 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | 40 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | 34,320 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | 1,320 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | 20,240 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2CompressBabyBear | COMP_POS2 | 380,016 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | PERM_POS2 | 92,916 | 
| verify_fibair | FriReducedOpeningAir | FriReducedOpening | FRI_REDUCED_OPENING | 21,504 | 
| verify_fibair | PhantomAir | HintBitsF | PHANTOM | 258 | 
| verify_fibair | PhantomAir | HintInputVec | PHANTOM | 11,778 | 

| group | chip_name | rows_used |
| --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 30,559 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | 5,162 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 97,997 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 85,553 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2,186 | 
| verify_fibair | AccessAdapter<2> | 22,166 | 
| verify_fibair | AccessAdapter<4> | 11,084 | 
| verify_fibair | AccessAdapter<8> | 3,224 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | 1,359 | 
| verify_fibair | Boundary | 37,800 | 
| verify_fibair | FriReducedOpeningAir | 336 | 
| verify_fibair | PhantomAir | 2,006 | 
| verify_fibair | ProgramChip | 5,915 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 
| verify_fibair | VmConnectorAir | 2 | 

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| verify_fibair |  | JAL | 1 | 
| verify_fibair |  | STOREW | 2 | 
| verify_fibair | AddE | FE4ADD | 492 | 
| verify_fibair | AddEFFI | LOADW | 70 | 
| verify_fibair | AddEFFI | STOREW | 210 | 
| verify_fibair | AddEI | ADD | 2,580 | 
| verify_fibair | AddF | ADD | 1,333 | 
| verify_fibair | AddFI | ADD | 660 | 
| verify_fibair | AddV | ADD | 1,112 | 
| verify_fibair | AddVI | ADD | 15,999 | 
| verify_fibair | Alloc | ADD | 6,030 | 
| verify_fibair | Alloc | LOADW | 6,030 | 
| verify_fibair | Alloc | MUL | 4,067 | 
| verify_fibair | AssertEqE | BNE | 172 | 
| verify_fibair | AssertEqEI | BNE | 4 | 
| verify_fibair | AssertEqF | BNE | 3,392 | 
| verify_fibair | AssertEqV | BNE | 177 | 
| verify_fibair | AssertEqVI | BNE | 20 | 
| verify_fibair | DivE | BBE4DIV | 296 | 
| verify_fibair | DivEIN | BBE4DIV | 1 | 
| verify_fibair | DivEIN | STOREW | 4 | 
| verify_fibair | DivFIN | DIV | 3 | 
| verify_fibair | For | ADD | 17,095 | 
| verify_fibair | For | BNE | 21,200 | 
| verify_fibair | For | JAL | 4,105 | 
| verify_fibair | For | LOADW | 467 | 
| verify_fibair | For | STOREW | 3,638 | 
| verify_fibair | FriReducedOpening | FRI_REDUCED_OPENING | 126 | 
| verify_fibair | HintBitsF | PHANTOM | 43 | 
| verify_fibair | HintInputVec | PHANTOM | 1,963 | 
| verify_fibair | IfEq | BNE | 321 | 
| verify_fibair | IfEqI | BNE | 3,855 | 
| verify_fibair | IfEqI | JAL | 1,054 | 
| verify_fibair | IfNe | BEQ | 1,163 | 
| verify_fibair | IfNe | JAL | 2 | 
| verify_fibair | IfNeI | BEQ | 255 | 
| verify_fibair | ImmE | STOREW | 848 | 
| verify_fibair | ImmF | STOREW | 4,335 | 
| verify_fibair | ImmV | STOREW | 4,148 | 
| verify_fibair | LoadE | ADD | 1,138 | 
| verify_fibair | LoadE | LOADW | 6,768 | 
| verify_fibair | LoadE | MUL | 1,138 | 
| verify_fibair | LoadF | ADD | 732 | 
| verify_fibair | LoadF | LOADW | 7,913 | 
| verify_fibair | LoadF | MUL | 516 | 
| verify_fibair | LoadHeapPtr | ADD | 1 | 
| verify_fibair | LoadV | ADD | 8,449 | 
| verify_fibair | LoadV | LOADW | 11,312 | 
| verify_fibair | LoadV | MUL | 5,842 | 
| verify_fibair | MulE | BBE4MUL | 858 | 
| verify_fibair | MulEF | MUL | 680 | 
| verify_fibair | MulEI | BBE4MUL | 33 | 
| verify_fibair | MulEI | STOREW | 132 | 
| verify_fibair | MulF | MUL | 2,429 | 
| verify_fibair | MulFI | MUL | 1,334 | 
| verify_fibair | MulVI | MUL | 1,435 | 
| verify_fibair | Poseidon2CompressBabyBear | COMP_POS2 | 1,092 | 
| verify_fibair | Poseidon2PermuteBabyBear | PERM_POS2 | 267 | 
| verify_fibair | StoreE | ADD | 508 | 
| verify_fibair | StoreE | MUL | 508 | 
| verify_fibair | StoreE | STOREW | 15,500 | 
| verify_fibair | StoreF | ADD | 1,691 | 
| verify_fibair | StoreF | MUL | 10 | 
| verify_fibair | StoreF | STOREW | 4,463 | 
| verify_fibair | StoreHeapPtr | ADD | 1 | 
| verify_fibair | StoreHintWord | ADD | 10,367 | 
| verify_fibair | StoreHintWord | SHINTW | 13,663 | 
| verify_fibair | StoreV | ADD | 5,091 | 
| verify_fibair | StoreV | MUL | 3,939 | 
| verify_fibair | StoreV | STOREW | 5,657 | 
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


Commit: https://github.com/openvm-org/openvm/commit/da59269220691673f786ce6adfbbfe8f15c6b070

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12665230541)
