| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+2 [+58.0%])</span> 6.06 | <span style='color: red'>(+2 [+58.0%])</span> 6.06 |
| verify_fibair | <span style='color: red'>(+2 [+58.0%])</span> 6.06 | <span style='color: red'>(+2 [+58.0%])</span> 6.06 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+2224 [+58.0%])</span> 6,061 | <span style='color: red'>(+2224 [+58.0%])</span> 6,061 | <span style='color: red'>(+2224 [+58.0%])</span> 6,061 | <span style='color: red'>(+2224 [+58.0%])</span> 6,061 |
| `main_cells_used     ` | <span style='color: green'>(-782954 [-2.6%])</span> 29,226,320 | <span style='color: green'>(-782954 [-2.6%])</span> 29,226,320 | <span style='color: green'>(-782954 [-2.6%])</span> 29,226,320 | <span style='color: green'>(-782954 [-2.6%])</span> 29,226,320 |
| `total_cycles        ` | <span style='color: red'>(+63744 [+8.5%])</span> 809,841 | <span style='color: red'>(+63744 [+8.5%])</span> 809,841 | <span style='color: red'>(+63744 [+8.5%])</span> 809,841 | <span style='color: red'>(+63744 [+8.5%])</span> 809,841 |
| `execute_time_ms     ` | <span style='color: red'>(+2339 [+1509.0%])</span> 2,494 | <span style='color: red'>(+2339 [+1509.0%])</span> 2,494 | <span style='color: red'>(+2339 [+1509.0%])</span> 2,494 | <span style='color: red'>(+2339 [+1509.0%])</span> 2,494 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+20 [+4.4%])</span> 470 | <span style='color: red'>(+20 [+4.4%])</span> 470 | <span style='color: red'>(+20 [+4.4%])</span> 470 | <span style='color: red'>(+20 [+4.4%])</span> 470 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: green'>(-135 [-4.2%])</span> 3,097 | <span style='color: green'>(-135 [-4.2%])</span> 3,097 | <span style='color: green'>(-135 [-4.2%])</span> 3,097 | <span style='color: green'>(-135 [-4.2%])</span> 3,097 |
| `main_trace_commit_time_ms` | <span style='color: green'>(-11 [-1.8%])</span> 607 | <span style='color: green'>(-11 [-1.8%])</span> 607 | <span style='color: green'>(-11 [-1.8%])</span> 607 | <span style='color: green'>(-11 [-1.8%])</span> 607 |
| `generate_perm_trace_time_ms` | <span style='color: green'>(-12 [-13.5%])</span> 77 | <span style='color: green'>(-12 [-13.5%])</span> 77 | <span style='color: green'>(-12 [-13.5%])</span> 77 | <span style='color: green'>(-12 [-13.5%])</span> 77 |
| `perm_trace_commit_time_ms` | <span style='color: green'>(-18 [-3.4%])</span> 511 | <span style='color: green'>(-18 [-3.4%])</span> 511 | <span style='color: green'>(-18 [-3.4%])</span> 511 | <span style='color: green'>(-18 [-3.4%])</span> 511 |
| `quotient_poly_compute_time_ms` | <span style='color: green'>(-69 [-10.5%])</span> 590 | <span style='color: green'>(-69 [-10.5%])</span> 590 | <span style='color: green'>(-69 [-10.5%])</span> 590 | <span style='color: green'>(-69 [-10.5%])</span> 590 |
| `quotient_poly_commit_time_ms` | <span style='color: green'>(-19 [-3.2%])</span> 576 | <span style='color: green'>(-19 [-3.2%])</span> 576 | <span style='color: green'>(-19 [-3.2%])</span> 576 | <span style='color: green'>(-19 [-3.2%])</span> 576 |
| `pcs_opening_time_ms ` | <span style='color: green'>(-6 [-0.8%])</span> 733 | <span style='color: green'>(-6 [-0.8%])</span> 733 | <span style='color: green'>(-6 [-0.8%])</span> 733 | <span style='color: green'>(-6 [-0.8%])</span> 733 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 3 | 65,536 | 66 | 3 | 13 | 0 | 31 | 17 | 

| air_name | rows | quotient_deg | main_cols | interactions | constraints | cells |
| --- | --- | --- | --- | --- | --- | --- |
| AccessAdapterAir<2> |  | 4 |  | 5 | 12 |  | 
| AccessAdapterAir<4> |  | 4 |  | 5 | 12 |  | 
| AccessAdapterAir<8> |  | 4 |  | 5 | 12 |  | 
| FibonacciAir | 32,768 | 1 | 2 |  | 5 | 65,536 | 
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
| verify_fibair | 470 | 6,061 | 809,841 | 82,499,608 | 3,097 | 590 | 576 | 511 | 733 | 607 | 29,226,320 | 77 | 2,494 | 

| group | air_name | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | AccessAdapterAir<2> | 131,072 |  | 16 | 11 | 3,538,944 | 
| verify_fibair | AccessAdapterAir<4> | 65,536 |  | 16 | 13 | 1,900,544 | 
| verify_fibair | AccessAdapterAir<8> | 32,768 |  | 16 | 17 | 1,081,344 | 
| verify_fibair | FriReducedOpeningAir | 512 |  | 76 | 64 | 71,680 | 
| verify_fibair | NativePoseidon2Air<BabyBearParameters>, 1> | 8,192 |  | 36 | 348 | 3,145,728 | 
| verify_fibair | PhantomAir | 16,384 |  | 8 | 6 | 229,376 | 
| verify_fibair | ProgramAir | 8,192 |  | 8 | 10 | 147,456 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 262,144 |  | 28 | 23 | 13,369,344 | 
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 32,768 |  | 12 | 10 | 720,896 | 
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 524,288 |  | 20 | 30 | 26,214,400 | 
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 524,288 |  | 20 | 31 | 26,738,688 | 
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 8,192 |  | 20 | 40 | 491,520 | 
| verify_fibair | VmConnectorAir | 2 | 1 | 8 | 4 | 24 | 
| verify_fibair | VolatileBoundaryAir | 131,072 |  | 8 | 11 | 2,490,368 | 

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | 3,956 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | 92 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | 163,024 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | 14,697 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | 460 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | 2,168,026 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | 24,817 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | 350,543 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | 167,831 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | 14,559 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | ZipFor | BNE | 203,941 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 10 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | For | JAL | 133,870 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | 50,930 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | 20 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | ZipFor | JAL | 8,930 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | 246,360 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | 50,160 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | 227,190 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | 2,051,220 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | 735,960 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | 454,980 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | 90 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | 2,426,250 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadE | ADD | 103,320 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadE | MUL | 103,320 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | ADD | 23,280 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | MUL | 15,480 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | ADD | 807,630 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | MUL | 466,080 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | 75,840 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | 128,310 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | 40,020 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | 263,940 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreE | ADD | 23,940 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreE | MUL | 23,940 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | ADD | 156,690 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | MUL | 300 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | 1,838,250 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | ADD | 70,110 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | MUL | 49,410 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | 3,930 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | 240 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubFI | SUB | 39,990 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | 112,170 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | 22,350 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | 18,900 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | ZipFor | ADD | 239,640 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 31 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | LOADW | 3,534 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | STOREW | 10,602 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | LOADW | 760,492 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | DivEIN | STOREW | 124 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | LOADW | 28,830 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | STOREW | 386,167 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | STOREW | 26,288 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | STOREW | 248,961 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | STOREW | 302,622 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW | 668,112 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW | 607,445 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW | 1,182,371 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | STOREW | 9,548 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW | 825,592 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW | 370,233 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | 2,231,194 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW | 634,353 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | SubEF | LOADW | 12,183 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ZipFor | LOADW | 27,714 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | 57,960 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | 30,320 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | 40 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | 108,680 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | 3,080 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | 75,680 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2CompressBabyBear | COMP_POS2 | 2,470,104 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | PERM_POS2 | 257,520 | 
| verify_fibair | FriReducedOpeningAir | FriReducedOpening | FRI_REDUCED_OPENING | 21,504 | 
| verify_fibair | PhantomAir | HintBitsF | PHANTOM | 258 | 
| verify_fibair | PhantomAir | HintInputVec | PHANTOM | 56,196 | 

| group | chip_name | rows_used |
| --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 135,302 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | 19,376 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 361,979 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 268,917 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 6,894 | 
| verify_fibair | AccessAdapter<2> | 95,758 | 
| verify_fibair | AccessAdapter<4> | 47,880 | 
| verify_fibair | AccessAdapter<8> | 17,106 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | 7,838 | 
| verify_fibair | Boundary | 128,081 | 
| verify_fibair | FriReducedOpeningAir | 336 | 
| verify_fibair | PhantomAir | 9,409 | 
| verify_fibair | ProgramChip | 5,852 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 
| verify_fibair | VmConnectorAir | 2 | 

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| verify_fibair |  | JAL | 1 | 
| verify_fibair |  | STOREW | 2 | 
| verify_fibair | AddE | FE4ADD | 1,449 | 
| verify_fibair | AddEFFI | LOADW | 114 | 
| verify_fibair | AddEFFI | STOREW | 342 | 
| verify_fibair | AddEI | ADD | 8,212 | 
| verify_fibair | AddF | ADD | 1,333 | 
| verify_fibair | AddFI | ADD | 1,672 | 
| verify_fibair | AddV | ADD | 7,573 | 
| verify_fibair | AddVI | ADD | 68,374 | 
| verify_fibair | Alloc | ADD | 24,532 | 
| verify_fibair | Alloc | LOADW | 24,532 | 
| verify_fibair | Alloc | MUL | 15,166 | 
| verify_fibair | AssertEqE | BNE | 172 | 
| verify_fibair | AssertEqEI | BNE | 4 | 
| verify_fibair | AssertEqF | BNE | 7,088 | 
| verify_fibair | AssertEqV | BNE | 639 | 
| verify_fibair | AssertEqVI | BNE | 20 | 
| verify_fibair | DivE | BBE4DIV | 758 | 
| verify_fibair | DivEIN | BBE4DIV | 1 | 
| verify_fibair | DivEIN | STOREW | 4 | 
| verify_fibair | DivFIN | DIV | 3 | 
| verify_fibair | For | ADD | 80,875 | 
| verify_fibair | For | BNE | 94,262 | 
| verify_fibair | For | JAL | 13,387 | 
| verify_fibair | For | LOADW | 930 | 
| verify_fibair | For | STOREW | 12,457 | 
| verify_fibair | FriReducedOpening | FRI_REDUCED_OPENING | 126 | 
| verify_fibair | HintBitsF | PHANTOM | 43 | 
| verify_fibair | HintInputVec | PHANTOM | 9,366 | 
| verify_fibair | IfEq | BNE | 1,079 | 
| verify_fibair | IfEqI | BNE | 15,241 | 
| verify_fibair | IfEqI | JAL | 5,093 | 
| verify_fibair | IfNe | BEQ | 7,297 | 
| verify_fibair | IfNe | JAL | 2 | 
| verify_fibair | IfNeI | BEQ | 633 | 
| verify_fibair | ImmE | STOREW | 848 | 
| verify_fibair | ImmF | STOREW | 8,031 | 
| verify_fibair | ImmV | STOREW | 9,762 | 
| verify_fibair | LoadE | ADD | 3,444 | 
| verify_fibair | LoadE | LOADW | 21,552 | 
| verify_fibair | LoadE | MUL | 3,444 | 
| verify_fibair | LoadF | ADD | 776 | 
| verify_fibair | LoadF | LOADW | 19,595 | 
| verify_fibair | LoadF | MUL | 516 | 
| verify_fibair | LoadHeapPtr | ADD | 1 | 
| verify_fibair | LoadV | ADD | 26,921 | 
| verify_fibair | LoadV | LOADW | 38,141 | 
| verify_fibair | LoadV | MUL | 15,536 | 
| verify_fibair | MulE | BBE4MUL | 2,717 | 
| verify_fibair | MulEF | MUL | 2,528 | 
| verify_fibair | MulEI | BBE4MUL | 77 | 
| verify_fibair | MulEI | STOREW | 308 | 
| verify_fibair | MulF | MUL | 4,277 | 
| verify_fibair | MulFI | MUL | 1,334 | 
| verify_fibair | MulVI | MUL | 8,798 | 
| verify_fibair | Poseidon2CompressBabyBear | COMP_POS2 | 7,098 | 
| verify_fibair | Poseidon2PermuteBabyBear | PERM_POS2 | 740 | 
| verify_fibair | StoreE | ADD | 798 | 
| verify_fibair | StoreE | MUL | 798 | 
| verify_fibair | StoreE | STOREW | 26,632 | 
| verify_fibair | StoreF | ADD | 5,223 | 
| verify_fibair | StoreF | MUL | 10 | 
| verify_fibair | StoreF | STOREW | 11,943 | 
| verify_fibair | StoreHeapPtr | ADD | 1 | 
| verify_fibair | StoreHintWord | ADD | 61,275 | 
| verify_fibair | StoreHintWord | SHINTW | 71,974 | 
| verify_fibair | StoreV | ADD | 2,337 | 
| verify_fibair | StoreV | MUL | 1,647 | 
| verify_fibair | StoreV | STOREW | 20,463 | 
| verify_fibair | SubE | FE4SUB | 1,892 | 
| verify_fibair | SubEF | LOADW | 393 | 
| verify_fibair | SubEF | SUB | 131 | 
| verify_fibair | SubEI | ADD | 8 | 
| verify_fibair | SubFI | SUB | 1,333 | 
| verify_fibair | SubV | SUB | 3,739 | 
| verify_fibair | SubVI | SUB | 745 | 
| verify_fibair | SubVIN | SUB | 630 | 
| verify_fibair | UnsafeCastVF | ADD | 1 | 
| verify_fibair | ZipFor | ADD | 7,988 | 
| verify_fibair | ZipFor | BNE | 8,867 | 
| verify_fibair | ZipFor | JAL | 893 | 
| verify_fibair | ZipFor | LOADW | 894 | 

</details>


<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/0d3a6332ecd96f89625c6d1b9318f113ddcda165/verify_fibair-0d3a6332ecd96f89625c6d1b9318f113ddcda165-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/0d3a6332ecd96f89625c6d1b9318f113ddcda165

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12684059059)
