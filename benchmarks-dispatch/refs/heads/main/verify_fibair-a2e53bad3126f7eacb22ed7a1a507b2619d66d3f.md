| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+1 [+38.2%])</span> 1.98 | <span style='color: red'>(+1 [+38.2%])</span> 1.98 |
| verify_fibair | <span style='color: red'>(+1 [+38.2%])</span> 1.98 | <span style='color: red'>(+1 [+38.2%])</span> 1.98 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+548 [+38.2%])</span> 1,983 | <span style='color: red'>(+548 [+38.2%])</span> 1,983 | <span style='color: red'>(+548 [+38.2%])</span> 1,983 | <span style='color: red'>(+548 [+38.2%])</span> 1,983 |
| `main_cells_used     ` |  8,027,672 |  8,027,672 |  8,027,672 |  8,027,672 |
| `total_cycles        ` |  195,184 |  195,184 |  195,184 |  195,184 |
| `execute_time_ms     ` | <span style='color: red'>(+566 [+959.3%])</span> 625 | <span style='color: red'>(+566 [+959.3%])</span> 625 | <span style='color: red'>(+566 [+959.3%])</span> 625 | <span style='color: red'>(+566 [+959.3%])</span> 625 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+6 [+4.4%])</span> 142 | <span style='color: red'>(+6 [+4.4%])</span> 142 | <span style='color: red'>(+6 [+4.4%])</span> 142 | <span style='color: red'>(+6 [+4.4%])</span> 142 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-24 [-1.9%])</span> 1,216 | <span style='color: green'>(-24 [-1.9%])</span> 1,216 | <span style='color: green'>(-24 [-1.9%])</span> 1,216 | <span style='color: green'>(-24 [-1.9%])</span> 1,216 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-6 [-2.8%])</span> 205 | <span style='color: green'>(-6 [-2.8%])</span> 205 | <span style='color: green'>(-6 [-2.8%])</span> 205 | <span style='color: green'>(-6 [-2.8%])</span> 205 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+2 [+8.7%])</span> 25 | <span style='color: red'>(+2 [+8.7%])</span> 25 | <span style='color: red'>(+2 [+8.7%])</span> 25 | <span style='color: red'>(+2 [+8.7%])</span> 25 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-1 [-0.6%])</span> 180 | <span style='color: green'>(-1 [-0.6%])</span> 180 | <span style='color: green'>(-1 [-0.6%])</span> 180 | <span style='color: green'>(-1 [-0.6%])</span> 180 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-5 [-2.2%])</span> 218 | <span style='color: green'>(-5 [-2.2%])</span> 218 | <span style='color: green'>(-5 [-2.2%])</span> 218 | <span style='color: green'>(-5 [-2.2%])</span> 218 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-12 [-4.8%])</span> 239 | <span style='color: green'>(-12 [-4.8%])</span> 239 | <span style='color: green'>(-12 [-4.8%])</span> 239 | <span style='color: green'>(-12 [-4.8%])</span> 239 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-2 [-0.6%])</span> 345 | <span style='color: green'>(-2 [-0.6%])</span> 345 | <span style='color: green'>(-2 [-0.6%])</span> 345 | <span style='color: green'>(-2 [-0.6%])</span> 345 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 4 | 32 | 11 | 0 | 1 | 0 | 4 | 5 | 

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
| verify_fibair | 142 | 1,983 | 195,184 | 23,304,216 | 1,216 | 218 | 239 | 180 | 345 | 205 | 8,027,672 | 25 | 625 | 

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
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | 11,120 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | 20 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | 77,400 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | 19,800 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | 28,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | 479,970 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | 180,900 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | 122,010 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | 90 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | 512,850 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | 20,400 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | 72,870 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | 40,020 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | 40,440 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | 311,010 | 
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
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | LOADW | 247,230 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | DivEIN | STOREW | 164 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | LOADW | 12,054 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | STOREW | 156,251 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | STOREW | 34,768 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | STOREW | 177,735 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | STOREW | 170,068 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW | 90,856 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW2 | 186,632 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW | 284,089 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW2 | 40,344 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW | 109,511 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW2 | 354,281 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | STOREW | 5,412 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW | 552,188 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW2 | 83,312 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW | 113,652 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW2 | 69,331 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | 560,183 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW | 23,206 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW2 | 208,731 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | SubEF | LOADW | 16,113 | 
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
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | 5,220 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 68,175 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 85,553 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2,186 | 
| verify_fibair | AccessAdapter<2> | 22,182 | 
| verify_fibair | AccessAdapter<4> | 11,092 | 
| verify_fibair | AccessAdapter<8> | 3,224 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | 1,359 | 
| verify_fibair | Boundary | 37,791 | 
| verify_fibair | FriReducedOpeningAir | 336 | 
| verify_fibair | PhantomAir | 2,006 | 
| verify_fibair | ProgramChip | 4,921 | 
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
| verify_fibair | AddV | ADD | 939 | 
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
| verify_fibair | For | LOADW | 294 | 
| verify_fibair | For | STOREW | 3,811 | 
| verify_fibair | FriReducedOpening | FRI_REDUCED_OPENING | 126 | 
| verify_fibair | HintBitsF | PHANTOM | 43 | 
| verify_fibair | HintInputVec | PHANTOM | 1,963 | 
| verify_fibair | IfEq | BNE | 321 | 
| verify_fibair | IfEqI | BNE | 3,855 | 
| verify_fibair | IfEqI | JAL | 1,112 | 
| verify_fibair | IfNe | BEQ | 1,163 | 
| verify_fibair | IfNe | JAL | 2 | 
| verify_fibair | IfNeI | BEQ | 255 | 
| verify_fibair | ImmE | STOREW | 848 | 
| verify_fibair | ImmF | STOREW | 4,335 | 
| verify_fibair | ImmV | STOREW | 4,148 | 
| verify_fibair | LoadE | LOADW | 2,216 | 
| verify_fibair | LoadE | LOADW2 | 4,552 | 
| verify_fibair | LoadF | LOADW | 6,929 | 
| verify_fibair | LoadF | LOADW2 | 984 | 
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
| verify_fibair | Poseidon2PermuteBabyBear | PERM_POS2 | 267 | 
| verify_fibair | StoreE | STOREW | 13,468 | 
| verify_fibair | StoreE | STOREW2 | 2,032 | 
| verify_fibair | StoreF | STOREW | 2,772 | 
| verify_fibair | StoreF | STOREW2 | 1,691 | 
| verify_fibair | StoreHeapPtr | ADD | 1 | 
| verify_fibair | StoreHintWord | ADD | 10,367 | 
| verify_fibair | StoreHintWord | SHINTW | 13,663 | 
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


<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f/verify_fibair-a2e53bad3126f7eacb22ed7a1a507b2619d66d3f-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/a2e53bad3126f7eacb22ed7a1a507b2619d66d3f

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12657226496)
