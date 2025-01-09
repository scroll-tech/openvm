| Summary | Proof Time (s) | Parallel Proof Time (s) |
|:---|---:|---:|
| Total | <span style='color: red'>(+239 [+6230.5%])</span> 242.90 | <span style='color: red'>(+239 [+6230.5%])</span> 242.90 |
| verify_fibair | <span style='color: red'>(+239 [+6230.5%])</span> 242.90 | <span style='color: red'>(+239 [+6230.5%])</span> 242.90 |


| verify_fibair |||||
|:---|---:|---:|---:|---:|
|metric|avg|sum|max|min|
| `total_proof_time_ms ` | <span style='color: red'>(+239066 [+6230.5%])</span> 242,903 | <span style='color: red'>(+239066 [+6230.5%])</span> 242,903 | <span style='color: red'>(+239066 [+6230.5%])</span> 242,903 | <span style='color: red'>(+239066 [+6230.5%])</span> 242,903 |
| `main_cells_used     ` | <span style='color: red'>(+1337549435 [+4457.1%])</span> 1,367,558,709 | <span style='color: red'>(+1337549435 [+4457.1%])</span> 1,367,558,709 | <span style='color: red'>(+1337549435 [+4457.1%])</span> 1,367,558,709 | <span style='color: red'>(+1337549435 [+4457.1%])</span> 1,367,558,709 |
| `total_cycles        ` | <span style='color: red'>(+34585117 [+4635.5%])</span> 35,331,214 | <span style='color: red'>(+34585117 [+4635.5%])</span> 35,331,214 | <span style='color: red'>(+34585117 [+4635.5%])</span> 35,331,214 | <span style='color: red'>(+34585117 [+4635.5%])</span> 35,331,214 |
| `execute_time_ms     ` | <span style='color: red'>(+110335 [+71183.9%])</span> 110,490 | <span style='color: red'>(+110335 [+71183.9%])</span> 110,490 | <span style='color: red'>(+110335 [+71183.9%])</span> 110,490 | <span style='color: red'>(+110335 [+71183.9%])</span> 110,490 |
| `trace_gen_time_ms   ` | <span style='color: red'>(+18494 [+4109.8%])</span> 18,944 | <span style='color: red'>(+18494 [+4109.8%])</span> 18,944 | <span style='color: red'>(+18494 [+4109.8%])</span> 18,944 | <span style='color: red'>(+18494 [+4109.8%])</span> 18,944 |
| `stark_prove_excluding_trace_time_ms` | <span style='color: red'>(+110237 [+3410.8%])</span> 113,469 | <span style='color: red'>(+110237 [+3410.8%])</span> 113,469 | <span style='color: red'>(+110237 [+3410.8%])</span> 113,469 | <span style='color: red'>(+110237 [+3410.8%])</span> 113,469 |
| `main_trace_commit_time_ms` | <span style='color: red'>(+18576 [+3005.8%])</span> 19,194 | <span style='color: red'>(+18576 [+3005.8%])</span> 19,194 | <span style='color: red'>(+18576 [+3005.8%])</span> 19,194 | <span style='color: red'>(+18576 [+3005.8%])</span> 19,194 |
| `generate_perm_trace_time_ms` | <span style='color: red'>(+2028 [+2278.7%])</span> 2,117 | <span style='color: red'>(+2028 [+2278.7%])</span> 2,117 | <span style='color: red'>(+2028 [+2278.7%])</span> 2,117 | <span style='color: red'>(+2028 [+2278.7%])</span> 2,117 |
| `perm_trace_commit_time_ms` | <span style='color: red'>(+17055 [+3224.0%])</span> 17,584 | <span style='color: red'>(+17055 [+3224.0%])</span> 17,584 | <span style='color: red'>(+17055 [+3224.0%])</span> 17,584 | <span style='color: red'>(+17055 [+3224.0%])</span> 17,584 |
| `quotient_poly_compute_time_ms` | <span style='color: red'>(+26088 [+3958.7%])</span> 26,747 | <span style='color: red'>(+26088 [+3958.7%])</span> 26,747 | <span style='color: red'>(+26088 [+3958.7%])</span> 26,747 | <span style='color: red'>(+26088 [+3958.7%])</span> 26,747 |
| `quotient_poly_commit_time_ms` | <span style='color: red'>(+19276 [+3239.7%])</span> 19,871 | <span style='color: red'>(+19276 [+3239.7%])</span> 19,871 | <span style='color: red'>(+19276 [+3239.7%])</span> 19,871 | <span style='color: red'>(+19276 [+3239.7%])</span> 19,871 |
| `pcs_opening_time_ms ` | <span style='color: red'>(+27215 [+3682.7%])</span> 27,954 | <span style='color: red'>(+27215 [+3682.7%])</span> 27,954 | <span style='color: red'>(+27215 [+3682.7%])</span> 27,954 | <span style='color: red'>(+27215 [+3682.7%])</span> 27,954 |



<details>
<summary>Detailed Metrics</summary>

|  | verify_program_compile_ms | total_cells | stark_prove_excluding_trace_time_ms | quotient_poly_compute_time_ms | quotient_poly_commit_time_ms | perm_trace_commit_time_ms | pcs_opening_time_ms | main_trace_commit_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | 3 | 65,536 | 70 | 3 | 14 | 0 | 35 | 17 | 

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
| verify_fibair | 18,944 | 242,903 | 35,331,214 | 2,871,476,248 | 113,469 | 26,747 | 19,871 | 17,584 | 27,954 | 19,194 | 1,367,558,709 | 2,117 | 110,490 | 

| group | air_name | rows | prep_cols | perm_cols | main_cols | cells |
| --- | --- | --- | --- | --- | --- | --- |
| verify_fibair | AccessAdapterAir<2> | 16,777,216 |  | 16 | 11 | 452,984,832 | 
| verify_fibair | AccessAdapterAir<4> | 8,388,608 |  | 16 | 13 | 243,269,632 | 
| verify_fibair | AccessAdapterAir<8> | 524,288 |  | 16 | 17 | 17,301,504 | 
| verify_fibair | NativePoseidon2Air<BabyBearParameters>, 1> | 262,144 |  | 36 | 348 | 100,663,296 | 
| verify_fibair | PhantomAir | 16,384 |  | 8 | 6 | 229,376 | 
| verify_fibair | ProgramAir | 8,192 |  | 8 | 10 | 147,456 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 2 | 8 | 1 | 2,359,296 | 
| verify_fibair | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 4,194,304 |  | 28 | 23 | 213,909,504 | 
| verify_fibair | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 16,384 |  | 12 | 10 | 360,448 | 
| verify_fibair | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 16,777,216 |  | 20 | 30 | 838,860,800 | 
| verify_fibair | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 16,777,216 |  | 20 | 31 | 855,638,016 | 
| verify_fibair | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2,097,152 |  | 20 | 40 | 125,829,120 | 
| verify_fibair | VmConnectorAir | 2 | 1 | 8 | 4 | 24 | 
| verify_fibair | VolatileBoundaryAir | 1,048,576 |  | 8 | 11 | 19,922,944 | 

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | 92 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | 14,674 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | 1,626,330 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | 21,267,203 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | 42,534,567 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | 21,267,226 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | 46 | 
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | ZipFor | BNE | 5,520,299 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 10 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | For | JAL | 93,880 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | 10 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | ZipFor | JAL | 8,930 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | 55,554,840 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | 55,479,780 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | 27,630 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | 139,488,780 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | 589,650 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | 308,670 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | 1,839,660 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | ADD | 27,740,190 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadF | MUL | 240 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadHeapPtr | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | ADD | 3,180 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | LoadV | MUL | 3,120 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | 120 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | 60 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | 45,930 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | ADD | 55,480,500 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreF | MUL | 300 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | 1,838,250 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | ADD | 1,620 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreV | MUL | 1,530 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | 27,739,860 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | UnsafeCastVF | ADD | 30 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | ZipFor | ADD | 14,108,460 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 31 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | LOADW | 28,664,460 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | AddEFFI | STOREW | 85,993,380 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | Alloc | LOADW | 609,305 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | LOADW | 155 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | For | STOREW | 290,873 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmE | STOREW | 124 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmF | STOREW | 3,162 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ImmV | STOREW | 100,328,586 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadE | LOADW | 78,120 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadF | LOADW | 85,993,814 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | LoadV | LOADW | 14,673,912 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | MulEI | STOREW | 114,657,840 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreE | STOREW | 28,742,580 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreF | STOREW | 57,332,454 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | 2,189,871 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | StoreV | STOREW | 504,990 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | ZipFor | LOADW | 27,714 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | 27,739,800 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | 36,986,400 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | Poseidon2PermuteBabyBear | PERM_POS2 | 80,445,768 | 
| verify_fibair | PhantomAir | HintInputVec | PHANTOM | 56,196 | 

| group | chip_name | rows_used |
| --- | --- | --- |
| verify_fibair | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 4,010,019 | 
| verify_fibair | <JalNativeAdapterAir,JalCoreAir> | 10,283 | 
| verify_fibair | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 12,675,083 | 
| verify_fibair | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 16,777,142 | 
| verify_fibair | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1,618,155 | 
| verify_fibair | AccessAdapter<2> | 12,020,672 | 
| verify_fibair | AccessAdapter<4> | 6,010,336 | 
| verify_fibair | AccessAdapter<8> | 462,334 | 
| verify_fibair | Arc<BabyBearParameters>, 1> | 231,166 | 
| verify_fibair | Boundary | 1,014,904 | 
| verify_fibair | PhantomAir | 9,366 | 
| verify_fibair | ProgramChip | 5,852 | 
| verify_fibair | VariableRangeCheckerAir | 262,144 | 
| verify_fibair | VmConnectorAir | 2 | 

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| verify_fibair |  | JAL | 1 | 
| verify_fibair |  | STOREW | 2 | 
| verify_fibair | AddE | FE4ADD | 693,495 | 
| verify_fibair | AddEFFI | LOADW | 924,660 | 
| verify_fibair | AddEFFI | STOREW | 2,773,980 | 
| verify_fibair | AddEI | ADD | 1,851,828 | 
| verify_fibair | AddFI | ADD | 1,849,326 | 
| verify_fibair | AddV | ADD | 921 | 
| verify_fibair | AddVI | ADD | 4,649,626 | 
| verify_fibair | Alloc | ADD | 19,655 | 
| verify_fibair | Alloc | LOADW | 19,655 | 
| verify_fibair | Alloc | MUL | 10,289 | 
| verify_fibair | AssertEqEI | BNE | 4 | 
| verify_fibair | AssertEqV | BNE | 638 | 
| verify_fibair | For | ADD | 61,322 | 
| verify_fibair | For | BNE | 70,710 | 
| verify_fibair | For | JAL | 9,388 | 
| verify_fibair | For | LOADW | 5 | 
| verify_fibair | For | STOREW | 9,383 | 
| verify_fibair | HintInputVec | PHANTOM | 9,366 | 
| verify_fibair | IfEq | BNE | 924,661 | 
| verify_fibair | IfEqI | BNE | 1,849,329 | 
| verify_fibair | IfNe | BEQ | 924,662 | 
| verify_fibair | IfNe | JAL | 1 | 
| verify_fibair | IfNeI | BEQ | 2 | 
| verify_fibair | ImmE | STOREW | 4 | 
| verify_fibair | ImmF | STOREW | 102 | 
| verify_fibair | ImmV | STOREW | 3,236,406 | 
| verify_fibair | LoadE | LOADW | 2,520 | 
| verify_fibair | LoadF | ADD | 924,673 | 
| verify_fibair | LoadF | LOADW | 2,773,994 | 
| verify_fibair | LoadF | MUL | 8 | 
| verify_fibair | LoadHeapPtr | ADD | 1 | 
| verify_fibair | LoadV | ADD | 106 | 
| verify_fibair | LoadV | LOADW | 473,352 | 
| verify_fibair | LoadV | MUL | 104 | 
| verify_fibair | MulEF | MUL | 4 | 
| verify_fibair | MulEI | BBE4MUL | 924,660 | 
| verify_fibair | MulEI | STOREW | 3,698,640 | 
| verify_fibair | MulF | MUL | 2 | 
| verify_fibair | MulFI | MUL | 1 | 
| verify_fibair | MulVI | MUL | 1,531 | 
| verify_fibair | Poseidon2PermuteBabyBear | PERM_POS2 | 231,166 | 
| verify_fibair | StoreE | STOREW | 927,180 | 
| verify_fibair | StoreF | ADD | 1,849,350 | 
| verify_fibair | StoreF | MUL | 10 | 
| verify_fibair | StoreF | STOREW | 1,849,434 | 
| verify_fibair | StoreHintWord | ADD | 61,275 | 
| verify_fibair | StoreHintWord | SHINTW | 70,641 | 
| verify_fibair | StoreV | ADD | 54 | 
| verify_fibair | StoreV | MUL | 51 | 
| verify_fibair | StoreV | STOREW | 16,290 | 
| verify_fibair | SubV | SUB | 1 | 
| verify_fibair | SubVI | SUB | 924,662 | 
| verify_fibair | UnsafeCastVF | ADD | 1 | 
| verify_fibair | ZipFor | ADD | 470,282 | 
| verify_fibair | ZipFor | BNE | 240,013 | 
| verify_fibair | ZipFor | JAL | 893 | 
| verify_fibair | ZipFor | LOADW | 894 | 

</details>


<details>
<summary>Flamegraphs</summary>

[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.frequency.reverse.svg)
[![](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.frequency.svg)](https://openvm-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/a6cbb849ffeffcad1c8f91800c348d03c57f82c0/verify_fibair-a6cbb849ffeffcad1c8f91800c348d03c57f82c0-verify_fibair.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/openvm-org/openvm/commit/a6cbb849ffeffcad1c8f91800c348d03c57f82c0

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/openvm-org/openvm/actions/runs/12683783240)
