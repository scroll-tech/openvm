| group | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_proof_time_ms | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | <span style="color: green">(-2.0 [-0.1%])</span> <div style='text-align: right'>1,372.0</div>  | <span style="color: green">(-1,536 [-0.1%])</span> <div style='text-align: right'>1,915,152</div>  | <span style="color: green">(-520 [-0.4%])</span> <div style='text-align: right'>145,208</div>  | <span style="color: green">(-5.0 [-0.4%])</span> <div style='text-align: right'>1,391.0</div>  | <span style="color: green">(-3.0 [-13.6%])</span> <div style='text-align: right'>19.0</div>  |  |
| inner_verifier | <span style="color: green">(-1,849.0 [-2.2%])</span> <div style='text-align: right'>83,927.0</div>  | <span style="color: green">(-8,388,608 [-1.2%])</span> <div style='text-align: right'>706,805,784</div>  | <span style="color: green">(-5,472,177 [-1.7%])</span> <div style='text-align: right'>322,714,060</div>  | <span style="color: green">(-2,277.0 [-2.3%])</span> <div style='text-align: right'>96,757.0</div>  | <span style="color: green">(-428.0 [-3.2%])</span> <div style='text-align: right'>12,830.0</div>  | <span style="color: green">(-549.0 [-1.2%])</span> <div style='text-align: right'>45,517.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| bench_program_inner | BranchEqual | <div style='text-align: right'>5</div>  |
| bench_program_inner | ByteXor | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | Core | <div style='text-align: right'>20</div>  |
| bench_program_inner | FieldArithmetic | <div style='text-align: right'>13</div>  |
| bench_program_inner | FieldExtension | <div style='text-align: right'>1</div>  |
| bench_program_inner | Jal | <div style='text-align: right'>2</div>  |
| bench_program_inner | Keccak256 | <div style='text-align: right'>24</div>  |
| bench_program_inner | Memory AccessAdapter<2> | <div style='text-align: right'>26</div>  |
| bench_program_inner | Memory AccessAdapter<4> | <div style='text-align: right'>13</div>  |
| bench_program_inner | Memory AccessAdapter<8> | <div style='text-align: right'>5</div>  |
| bench_program_inner | Memory Boundary | <div style='text-align: right'>65</div>  |
| bench_program_inner | ProgramChip | <div style='text-align: right'>37</div>  |
| inner_verifier | BranchEqual | <span style="color: green">(-1,356 [-0.2%])</span> <div style='text-align: right'>706,183</div>  |
| inner_verifier | Core | <span style="color: green">(-7,130 [-0.2%])</span> <div style='text-align: right'>3,156,577</div>  |
| inner_verifier | FieldArithmetic | <span style="color: green">(-3,048 [-0.2%])</span> <div style='text-align: right'>1,594,136</div>  |
| inner_verifier | FieldExtension | <span style="color: green">(-2,100 [-0.2%])</span> <div style='text-align: right'>859,724</div>  |
| inner_verifier | Jal | <span style="color: red">(+506 [+1.7%])</span> <div style='text-align: right'>29,602</div>  |
| inner_verifier | Memory AccessAdapter<2> | <span style="color: green">(-4,514 [-0.2%])</span> <div style='text-align: right'>1,980,437</div>  |
| inner_verifier | Memory AccessAdapter<4> | <span style="color: green">(-2,257 [-0.2%])</span> <div style='text-align: right'>990,283</div>  |
| inner_verifier | Memory AccessAdapter<8> | <span style="color: green">(-42 [-0.1%])</span> <div style='text-align: right'>33,644</div>  |
| inner_verifier | Memory Boundary | <span style="color: green">(-348 [-0.1%])</span> <div style='text-align: right'>597,920</div>  |
| inner_verifier | Poseidon2 | <span style="color: green">(-21 [-0.1%])</span> <div style='text-align: right'>20,439</div>  |
| inner_verifier | ProgramChip | <span style="color: green">(-376 [-0.2%])</span> <div style='text-align: right'>202,445</div>  |

<details>
<summary>Click to expand</summary>

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| bench_program_inner |  | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner |  | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | AddE | FE4ADD | <div style='text-align: right'>1</div>  |
| bench_program_inner | AddF | ADD | <div style='text-align: right'>1</div>  |
| bench_program_inner | AddVI | ADD | <div style='text-align: right'>6</div>  |
| bench_program_inner | Alloc | ADD | <div style='text-align: right'>2</div>  |
| bench_program_inner | Alloc | LOADW | <div style='text-align: right'>2</div>  |
| bench_program_inner | Alloc | MUL | <div style='text-align: right'>2</div>  |
| bench_program_inner | For | ADD | <div style='text-align: right'>2</div>  |
| bench_program_inner | For | BNE | <div style='text-align: right'>3</div>  |
| bench_program_inner | For | JAL | <div style='text-align: right'>1</div>  |
| bench_program_inner | For | STOREW | <div style='text-align: right'>1</div>  |
| bench_program_inner | IfEqI | BNE | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmE | STOREW | <div style='text-align: right'>8</div>  |
| bench_program_inner | ImmF | STOREW | <div style='text-align: right'>2</div>  |
| bench_program_inner | ImmV | STOREW | <div style='text-align: right'>3</div>  |
| bench_program_inner | Keccak256 | KECCAK256 | <div style='text-align: right'>1</div>  |
| bench_program_inner | StoreV | STOREW2 | <div style='text-align: right'>2</div>  |
| inner_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| inner_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| inner_verifier | AddE | FE4ADD | <span style="color: green">(-530 [-0.2%])</span> <div style='text-align: right'>228,023</div>  |
| inner_verifier | AddEFFI | LOADW | <div style='text-align: right'>132</div>  |
| inner_verifier | AddEFFI | STOREW | <div style='text-align: right'>396</div>  |
| inner_verifier | AddEFI | ADD | <span style="color: red">(+4 [+1.8%])</span> <div style='text-align: right'>224</div>  |
| inner_verifier | AddEI | ADD | <span style="color: green">(-92 [-0.1%])</span> <div style='text-align: right'>67,484</div>  |
| inner_verifier | AddFI | ADD | <span style="color: red">(+20 [+0.1%])</span> <div style='text-align: right'>13,627</div>  |
| inner_verifier | AddV | ADD | <div style='text-align: right'>6,239</div>  |
| inner_verifier | AddVI | ADD | <span style="color: green">(-504 [-0.2%])</span> <div style='text-align: right'>278,594</div>  |
| inner_verifier | Alloc | ADD | <div style='text-align: right'>24,510</div>  |
| inner_verifier | Alloc | LOADW | <div style='text-align: right'>24,510</div>  |
| inner_verifier | Alloc | MUL | <div style='text-align: right'>14,809</div>  |
| inner_verifier | AssertEqE | BNE | <div style='text-align: right'>140</div>  |
| inner_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| inner_verifier | AssertEqF | BNE | <div style='text-align: right'>4,054</div>  |
| inner_verifier | AssertEqV | BNE | <div style='text-align: right'>1,204</div>  |
| inner_verifier | AssertEqVI | BNE | <div style='text-align: right'>171</div>  |
| inner_verifier | CycleTrackerEnd | CT_END | <span style="color: green">(-252 [-0.2%])</span> <div style='text-align: right'>107,107</div>  |
| inner_verifier | CycleTrackerStart | CT_START | <span style="color: green">(-252 [-0.2%])</span> <div style='text-align: right'>107,107</div>  |
| inner_verifier | DivE | BBE4DIV | <span style="color: green">(-504 [-0.3%])</span> <div style='text-align: right'>198,961</div>  |
| inner_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>36</div>  |
| inner_verifier | DivEIN | STOREW | <div style='text-align: right'>144</div>  |
| inner_verifier | DivFIN | DIV | <div style='text-align: right'>86</div>  |
| inner_verifier | For | ADD | <span style="color: green">(-1,104 [-0.2%])</span> <div style='text-align: right'>540,643</div>  |
| inner_verifier | For | BNE | <span style="color: green">(-1,104 [-0.2%])</span> <div style='text-align: right'>560,394</div>  |
| inner_verifier | For | JAL | <div style='text-align: right'>19,751</div>  |
| inner_verifier | For | LOADW | <div style='text-align: right'>1,029</div>  |
| inner_verifier | For | STOREW | <div style='text-align: right'>18,722</div>  |
| inner_verifier | HintBitsF | HINT_BITS | <div style='text-align: right'>22</div>  |
| inner_verifier | HintInputVec | HINT_INPUT | <div style='text-align: right'>9,701</div>  |
| inner_verifier | IfEq | BNE | <div style='text-align: right'>7,587</div>  |
| inner_verifier | IfEqI | BNE | <span style="color: green">(-252 [-0.2%])</span> <div style='text-align: right'>124,657</div>  |
| inner_verifier | IfEqI | JAL | <span style="color: red">(+506 [+5.4%])</span> <div style='text-align: right'>9,830</div>  |
| inner_verifier | IfNe | BEQ | <div style='text-align: right'>6,893</div>  |
| inner_verifier | IfNe | JAL | <div style='text-align: right'>20</div>  |
| inner_verifier | IfNeI | BEQ | <div style='text-align: right'>1,079</div>  |
| inner_verifier | ImmE | STOREW | <span style="color: green">(-60 [-0.5%])</span> <div style='text-align: right'>12,508</div>  |
| inner_verifier | ImmF | STOREW | <div style='text-align: right'>15,873</div>  |
| inner_verifier | ImmV | STOREW | <span style="color: green">(-42 [-0.2%])</span> <div style='text-align: right'>22,303</div>  |
| inner_verifier | LoadE | LOADW | <span style="color: green">(-96 [-0.2%])</span> <div style='text-align: right'>42,232</div>  |
| inner_verifier | LoadE | LOADW2 | <span style="color: green">(-2,016 [-0.2%])</span> <div style='text-align: right'>818,552</div>  |
| inner_verifier | LoadF | LOADW | <div style='text-align: right'>11,473</div>  |
| inner_verifier | LoadF | LOADW2 | <span style="color: green">(-756 [-0.2%])</span> <div style='text-align: right'>305,859</div>  |
| inner_verifier | LoadV | LOADW | <div style='text-align: right'>11,843</div>  |
| inner_verifier | LoadV | LOADW2 | <div style='text-align: right'>70,929</div>  |
| inner_verifier | MulE | BBE4MUL | <span style="color: green">(-1,038 [-0.2%])</span> <div style='text-align: right'>416,127</div>  |
| inner_verifier | MulEF | MUL | <div style='text-align: right'>1,716</div>  |
| inner_verifier | MulEFI | MUL | <span style="color: green">(-16 [-1.1%])</span> <div style='text-align: right'>1,424</div>  |
| inner_verifier | MulEI | BBE4MUL | <span style="color: green">(-8 [-0.3%])</span> <div style='text-align: right'>2,720</div>  |
| inner_verifier | MulEI | STOREW | <span style="color: green">(-32 [-0.3%])</span> <div style='text-align: right'>10,880</div>  |
| inner_verifier | MulF | MUL | <div style='text-align: right'>24,377</div>  |
| inner_verifier | MulFI | MUL | <div style='text-align: right'>14</div>  |
| inner_verifier | MulV | MUL | <div style='text-align: right'>682</div>  |
| inner_verifier | MulVI | MUL | <div style='text-align: right'>8,441</div>  |
| inner_verifier | NegE | MUL | <div style='text-align: right'>204</div>  |
| inner_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>7,287</div>  |
| inner_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-21 [-0.2%])</span> <div style='text-align: right'>13,152</div>  |
| inner_verifier | StoreE | STOREW | <div style='text-align: right'>11,268</div>  |
| inner_verifier | StoreE | STOREW2 | <div style='text-align: right'>12,500</div>  |
| inner_verifier | StoreF | STOREW | <div style='text-align: right'>13,388</div>  |
| inner_verifier | StoreF | STOREW2 | <span style="color: green">(-252 [-0.2%])</span> <div style='text-align: right'>103,958</div>  |
| inner_verifier | StoreHintWord | ADD | <span style="color: green">(-348 [-0.2%])</span> <div style='text-align: right'>195,323</div>  |
| inner_verifier | StoreHintWord | SHINTW | <span style="color: green">(-348 [-0.2%])</span> <div style='text-align: right'>205,706</div>  |
| inner_verifier | StoreV | STOREW | <div style='text-align: right'>1,424</div>  |
| inner_verifier | StoreV | STOREW2 | <div style='text-align: right'>25,463</div>  |
| inner_verifier | SubE | FE4SUB | <span style="color: green">(-20 [-0.1%])</span> <div style='text-align: right'>13,857</div>  |
| inner_verifier | SubEF | LOADW | <span style="color: green">(-3,024 [-0.3%])</span> <div style='text-align: right'>1,191,546</div>  |
| inner_verifier | SubEF | SUB | <span style="color: green">(-1,008 [-0.3%])</span> <div style='text-align: right'>397,182</div>  |
| inner_verifier | SubEFI | ADD | <div style='text-align: right'>1,256</div>  |
| inner_verifier | SubEI | ADD | <div style='text-align: right'>288</div>  |
| inner_verifier | SubV | SUB | <div style='text-align: right'>15,386</div>  |
| inner_verifier | SubVI | SUB | <div style='text-align: right'>1,270</div>  |
| inner_verifier | SubVIN | SUB | <div style='text-align: right'>357</div>  |

</details>

<details>
<summary>Click to expand</summary>

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary |  | JAL | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  |
| bench_program_inner | Boundary |  | STOREW | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| bench_program_inner | CoreAir |  | STOREW | <div style='text-align: right'>110</div>  |
| bench_program_inner | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>40</div>  |
| bench_program_inner | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>66</div>  |
| bench_program_inner | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>39</div>  |
| bench_program_inner | Boundary | AddE | FE4ADD | <span style="color: green">(-32 [-42.1%])</span> <div style='text-align: right'>44</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddF | ADD | <div style='text-align: right'>30</div>  |
| bench_program_inner | Boundary | AddF | ADD | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>180</div>  |
| bench_program_inner | Boundary | AddVI | ADD | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | Boundary | Alloc | LOADW | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| bench_program_inner | CoreAir | Alloc | LOADW | <div style='text-align: right'>110</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>60</div>  |
| bench_program_inner | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>60</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>69</div>  |
| bench_program_inner | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>10</div>  |
| bench_program_inner | Boundary | For | STOREW | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  |
| bench_program_inner | CoreAir | For | STOREW | <div style='text-align: right'>55</div>  |
| bench_program_inner | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>46</div>  |
| bench_program_inner | Boundary | ImmE | STOREW | <span style="color: green">(-64 [-42.1%])</span> <div style='text-align: right'>88</div>  |
| bench_program_inner | CoreAir | ImmE | STOREW | <div style='text-align: right'>440</div>  |
| bench_program_inner | Boundary | ImmF | STOREW | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| bench_program_inner | CoreAir | ImmF | STOREW | <div style='text-align: right'>110</div>  |
| bench_program_inner | Boundary | ImmV | STOREW | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| bench_program_inner | CoreAir | ImmV | STOREW | <div style='text-align: right'>165</div>  |
| bench_program_inner | AccessAdapter<2> | Keccak256 | KECCAK256 | <div style='text-align: right'>220</div>  |
| bench_program_inner | AccessAdapter<4> | Keccak256 | KECCAK256 | <div style='text-align: right'>130</div>  |
| bench_program_inner | AccessAdapter<8> | Keccak256 | KECCAK256 | <div style='text-align: right'>85</div>  |
| bench_program_inner | Boundary | Keccak256 | KECCAK256 | <span style="color: green">(-304 [-42.1%])</span> <div style='text-align: right'>418</div>  |
| bench_program_inner | KeccakVmAir | Keccak256 | KECCAK256 | <div style='text-align: right'>76,752</div>  |
| bench_program_inner | Boundary | StoreV | STOREW2 | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| bench_program_inner | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>110</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| inner_verifier | Boundary |  | JAL | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  |
| inner_verifier | Boundary |  | STOREW | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| inner_verifier | CoreAir |  | STOREW | <div style='text-align: right'>110</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <span style="color: green">(-21,200 [-0.2%])</span> <div style='text-align: right'>9,120,920</div>  |
| inner_verifier | AccessAdapter<2> | AddE | FE4ADD | <span style="color: green">(-242 [-0.0%])</span> <div style='text-align: right'>1,111,726</div>  |
| inner_verifier | AccessAdapter<4> | AddE | FE4ADD | <span style="color: green">(-143 [-0.0%])</span> <div style='text-align: right'>656,929</div>  |
| inner_verifier | Boundary | AddE | FE4ADD | <span style="color: green">(-871,936 [-42.1%])</span> <div style='text-align: right'>1,198,912</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | LOADW | <span style="color: green">(-22 [-2.3%])</span> <div style='text-align: right'>946</div>  |
| inner_verifier | AccessAdapter<4> | AddEFFI | LOADW | <span style="color: green">(-26 [-2.3%])</span> <div style='text-align: right'>1,118</div>  |
| inner_verifier | Boundary | AddEFFI | LOADW | <span style="color: green">(-176 [-42.1%])</span> <div style='text-align: right'>242</div>  |
| inner_verifier | CoreAir | AddEFFI | LOADW | <div style='text-align: right'>7,260</div>  |
| inner_verifier | AccessAdapter<2> | AddEFFI | STOREW | <span style="color: green">(-22 [-2.3%])</span> <div style='text-align: right'>946</div>  |
| inner_verifier | Boundary | AddEFFI | STOREW | <span style="color: green">(-528 [-42.1%])</span> <div style='text-align: right'>726</div>  |
| inner_verifier | CoreAir | AddEFFI | STOREW | <div style='text-align: right'>21,780</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <span style="color: red">(+120 [+1.8%])</span> <div style='text-align: right'>6,720</div>  |
| inner_verifier | AccessAdapter<2> | AddEFI | ADD | <span style="color: red">(+110 [+13.9%])</span> <div style='text-align: right'>902</div>  |
| inner_verifier | AccessAdapter<4> | AddEFI | ADD | <span style="color: red">(+65 [+13.9%])</span> <div style='text-align: right'>533</div>  |
| inner_verifier | Boundary | AddEFI | ADD | <span style="color: green">(-896 [-42.1%])</span> <div style='text-align: right'>1,232</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <span style="color: green">(-2,760 [-0.1%])</span> <div style='text-align: right'>2,024,520</div>  |
| inner_verifier | AccessAdapter<2> | AddEI | ADD | <span style="color: green">(-264 [-0.1%])</span> <div style='text-align: right'>378,730</div>  |
| inner_verifier | AccessAdapter<4> | AddEI | ADD | <span style="color: green">(-156 [-0.1%])</span> <div style='text-align: right'>223,795</div>  |
| inner_verifier | Boundary | AddEI | ADD | <span style="color: green">(-476,100 [-42.1%])</span> <div style='text-align: right'>654,324</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <span style="color: red">(+600 [+0.1%])</span> <div style='text-align: right'>408,810</div>  |
| inner_verifier | Boundary | AddFI | ADD | <span style="color: green">(-192 [-42.1%])</span> <div style='text-align: right'>264</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>187,170</div>  |
| inner_verifier | Boundary | AddV | ADD | <span style="color: green">(-16 [-42.1%])</span> <div style='text-align: right'>22</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <span style="color: green">(-15,120 [-0.2%])</span> <div style='text-align: right'>8,357,820</div>  |
| inner_verifier | Boundary | AddVI | ADD | <span style="color: green">(-6,280 [-42.1%])</span> <div style='text-align: right'>8,635</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>735,300</div>  |
| inner_verifier | Boundary | Alloc | LOADW | <span style="color: green">(-696 [-42.1%])</span> <div style='text-align: right'>957</div>  |
| inner_verifier | CoreAir | Alloc | LOADW | <div style='text-align: right'>1,348,050</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>444,270</div>  |
| inner_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>26</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <div style='text-align: right'>3,220</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>770</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>455</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| inner_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| inner_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>93,242</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <div style='text-align: right'>27,692</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <div style='text-align: right'>3,933</div>  |
| inner_verifier | CoreAir | CycleTrackerEnd | CT_END | <span style="color: green">(-13,860 [-0.2%])</span> <div style='text-align: right'>5,890,885</div>  |
| inner_verifier | CoreAir | CycleTrackerStart | CT_START | <span style="color: green">(-13,860 [-0.2%])</span> <div style='text-align: right'>5,890,885</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <span style="color: green">(-20,160 [-0.3%])</span> <div style='text-align: right'>7,958,440</div>  |
| inner_verifier | AccessAdapter<2> | DivE | BBE4DIV | <span style="color: green">(-22,176 [-0.3%])</span> <div style='text-align: right'>8,738,290</div>  |
| inner_verifier | AccessAdapter<4> | DivE | BBE4DIV | <span style="color: green">(-13,104 [-0.3%])</span> <div style='text-align: right'>5,163,535</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>1,440</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <span style="color: red">(+66 [+4.5%])</span> <div style='text-align: right'>1,540</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <span style="color: red">(+39 [+4.5%])</span> <div style='text-align: right'>910</div>  |
| inner_verifier | Boundary | DivEIN | BBE4DIV | <span style="color: green">(-192 [-42.1%])</span> <div style='text-align: right'>264</div>  |
| inner_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>517</div>  |
| inner_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>143</div>  |
| inner_verifier | CoreAir | DivEIN | STOREW | <div style='text-align: right'>7,920</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>2,580</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <span style="color: green">(-33,120 [-0.2%])</span> <div style='text-align: right'>16,219,290</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <span style="color: green">(-25,392 [-0.2%])</span> <div style='text-align: right'>12,889,062</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>197,510</div>  |
| inner_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>473</div>  |
| inner_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>559</div>  |
| inner_verifier | Boundary | For | LOADW | <span style="color: green">(-168 [-42.1%])</span> <div style='text-align: right'>231</div>  |
| inner_verifier | CoreAir | For | LOADW | <div style='text-align: right'>56,595</div>  |
| inner_verifier | Boundary | For | STOREW | <span style="color: green">(-408 [-42.1%])</span> <div style='text-align: right'>561</div>  |
| inner_verifier | CoreAir | For | STOREW | <div style='text-align: right'>1,029,710</div>  |
| inner_verifier | CoreAir | HintBitsF | HINT_BITS | <div style='text-align: right'>1,210</div>  |
| inner_verifier | CoreAir | HintInputVec | HINT_INPUT | <div style='text-align: right'>533,555</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <div style='text-align: right'>174,501</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <span style="color: green">(-5,796 [-0.2%])</span> <div style='text-align: right'>2,867,111</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <span style="color: red">(+5,060 [+5.4%])</span> <div style='text-align: right'>98,300</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>158,539</div>  |
| inner_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>200</div>  |
| inner_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <div style='text-align: right'>24,817</div>  |
| inner_verifier | AccessAdapter<2> | ImmE | STOREW | <span style="color: red">(+22 [+0.5%])</span> <div style='text-align: right'>4,114</div>  |
| inner_verifier | AccessAdapter<4> | ImmE | STOREW | <span style="color: red">(+13 [+0.5%])</span> <div style='text-align: right'>2,431</div>  |
| inner_verifier | Boundary | ImmE | STOREW | <span style="color: green">(-90,532 [-42.2%])</span> <div style='text-align: right'>124,168</div>  |
| inner_verifier | CoreAir | ImmE | STOREW | <span style="color: green">(-3,300 [-0.5%])</span> <div style='text-align: right'>687,940</div>  |
| inner_verifier | Boundary | ImmF | STOREW | <span style="color: green">(-984 [-42.1%])</span> <div style='text-align: right'>1,353</div>  |
| inner_verifier | CoreAir | ImmF | STOREW | <div style='text-align: right'>873,015</div>  |
| inner_verifier | Boundary | ImmV | STOREW | <span style="color: green">(-6,344 [-42.1%])</span> <div style='text-align: right'>8,723</div>  |
| inner_verifier | CoreAir | ImmV | STOREW | <span style="color: green">(-2,310 [-0.2%])</span> <div style='text-align: right'>1,226,665</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW | <span style="color: green">(-682 [-1.1%])</span> <div style='text-align: right'>61,534</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW | <span style="color: green">(-403 [-1.1%])</span> <div style='text-align: right'>36,361</div>  |
| inner_verifier | Boundary | LoadE | LOADW | <span style="color: green">(-217,200 [-42.0%])</span> <div style='text-align: right'>299,904</div>  |
| inner_verifier | CoreAir | LoadE | LOADW | <span style="color: green">(-5,280 [-0.2%])</span> <div style='text-align: right'>2,322,760</div>  |
| inner_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>24,090</div>  |
| inner_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>14,235</div>  |
| inner_verifier | Boundary | LoadE | LOADW2 | <span style="color: green">(-32 [-42.1%])</span> <div style='text-align: right'>44</div>  |
| inner_verifier | CoreAir | LoadE | LOADW2 | <span style="color: green">(-110,880 [-0.2%])</span> <div style='text-align: right'>45,020,360</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>22,176</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>13,104</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>8,568</div>  |
| inner_verifier | Boundary | LoadF | LOADW | <span style="color: green">(-208 [-42.1%])</span> <div style='text-align: right'>286</div>  |
| inner_verifier | CoreAir | LoadF | LOADW | <div style='text-align: right'>631,015</div>  |
| inner_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>605</div>  |
| inner_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>364</div>  |
| inner_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>391</div>  |
| inner_verifier | Boundary | LoadF | LOADW2 | <span style="color: green">(-216 [-42.1%])</span> <div style='text-align: right'>297</div>  |
| inner_verifier | CoreAir | LoadF | LOADW2 | <span style="color: green">(-41,580 [-0.2%])</span> <div style='text-align: right'>16,822,245</div>  |
| inner_verifier | Boundary | LoadV | LOADW | <span style="color: green">(-5,784 [-42.1%])</span> <div style='text-align: right'>7,953</div>  |
| inner_verifier | CoreAir | LoadV | LOADW | <div style='text-align: right'>651,365</div>  |
| inner_verifier | Boundary | LoadV | LOADW2 | <span style="color: green">(-680 [-42.1%])</span> <div style='text-align: right'>935</div>  |
| inner_verifier | CoreAir | LoadV | LOADW2 | <div style='text-align: right'>3,901,095</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <span style="color: green">(-41,520 [-0.2%])</span> <div style='text-align: right'>16,645,080</div>  |
| inner_verifier | AccessAdapter<2> | MulE | BBE4MUL | <span style="color: green">(-572 [-0.1%])</span> <div style='text-align: right'>498,498</div>  |
| inner_verifier | AccessAdapter<4> | MulE | BBE4MUL | <span style="color: green">(-338 [-0.1%])</span> <div style='text-align: right'>294,567</div>  |
| inner_verifier | Boundary | MulE | BBE4MUL | <span style="color: green">(-510,372 [-42.1%])</span> <div style='text-align: right'>701,448</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>51,480</div>  |
| inner_verifier | AccessAdapter<2> | MulEF | MUL | <span style="color: green">(-44 [-0.5%])</span> <div style='text-align: right'>8,448</div>  |
| inner_verifier | AccessAdapter<4> | MulEF | MUL | <span style="color: green">(-26 [-0.5%])</span> <div style='text-align: right'>4,992</div>  |
| inner_verifier | Boundary | MulEF | MUL | <span style="color: green">(-384 [-42.1%])</span> <div style='text-align: right'>528</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <span style="color: green">(-480 [-1.1%])</span> <div style='text-align: right'>42,720</div>  |
| inner_verifier | AccessAdapter<2> | MulEFI | MUL | <span style="color: green">(-110 [-5.5%])</span> <div style='text-align: right'>1,892</div>  |
| inner_verifier | AccessAdapter<4> | MulEFI | MUL | <span style="color: green">(-65 [-5.5%])</span> <div style='text-align: right'>1,118</div>  |
| inner_verifier | Boundary | MulEFI | MUL | <span style="color: green">(-10,048 [-42.1%])</span> <div style='text-align: right'>13,816</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <span style="color: green">(-320 [-0.3%])</span> <div style='text-align: right'>108,800</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <span style="color: green">(-22 [-0.0%])</span> <div style='text-align: right'>163,856</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <span style="color: green">(-13 [-0.0%])</span> <div style='text-align: right'>96,824</div>  |
| inner_verifier | Boundary | MulEI | BBE4MUL | <span style="color: green">(-64,544 [-42.1%])</span> <div style='text-align: right'>88,748</div>  |
| inner_verifier | AccessAdapter<2> | MulEI | STOREW | <span style="color: green">(-176 [-0.3%])</span> <div style='text-align: right'>59,598</div>  |
| inner_verifier | AccessAdapter<4> | MulEI | STOREW | <span style="color: green">(-104 [-0.3%])</span> <div style='text-align: right'>35,087</div>  |
| inner_verifier | Boundary | MulEI | STOREW | <span style="color: green">(-24 [-42.1%])</span> <div style='text-align: right'>33</div>  |
| inner_verifier | CoreAir | MulEI | STOREW | <span style="color: green">(-1,760 [-0.3%])</span> <div style='text-align: right'>598,400</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>731,310</div>  |
| inner_verifier | Boundary | MulF | MUL | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>420</div>  |
| inner_verifier | Boundary | MulFI | MUL | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>20,460</div>  |
| inner_verifier | Boundary | MulV | MUL | <span style="color: green">(-5,432 [-42.1%])</span> <div style='text-align: right'>7,469</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>253,230</div>  |
| inner_verifier | Boundary | MulVI | MUL | <span style="color: green">(-56 [-42.1%])</span> <div style='text-align: right'>77</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>6,120</div>  |
| inner_verifier | AccessAdapter<2> | NegE | MUL | <span style="color: red">(+66 [+5.2%])</span> <div style='text-align: right'>1,342</div>  |
| inner_verifier | AccessAdapter<4> | NegE | MUL | <span style="color: red">(+39 [+5.2%])</span> <div style='text-align: right'>793</div>  |
| inner_verifier | Boundary | NegE | MUL | <span style="color: green">(-992 [-42.1%])</span> <div style='text-align: right'>1,364</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>298,452</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>176,358</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>115,311</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>3,045,966</div>  |
| inner_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-1,386 [-0.2%])</span> <div style='text-align: right'>618,926</div>  |
| inner_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-819 [-0.2%])</span> <div style='text-align: right'>366,145</div>  |
| inner_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-357 [-0.1%])</span> <div style='text-align: right'>241,179</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | Poseidon2PermuteBabyBear | PERM_POS2 | <span style="color: green">(-8,778 [-0.2%])</span> <div style='text-align: right'>5,497,536</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>7,898</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>4,667</div>  |
| inner_verifier | Boundary | StoreE | STOREW | <span style="color: green">(-90,144 [-42.1%])</span> <div style='text-align: right'>123,948</div>  |
| inner_verifier | CoreAir | StoreE | STOREW | <div style='text-align: right'>619,740</div>  |
| inner_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>52,668</div>  |
| inner_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>31,122</div>  |
| inner_verifier | Boundary | StoreE | STOREW2 | <span style="color: green">(-11,968 [-42.1%])</span> <div style='text-align: right'>16,456</div>  |
| inner_verifier | CoreAir | StoreE | STOREW2 | <div style='text-align: right'>687,500</div>  |
| inner_verifier | Boundary | StoreF | STOREW | <span style="color: green">(-107,104 [-42.1%])</span> <div style='text-align: right'>147,268</div>  |
| inner_verifier | CoreAir | StoreF | STOREW | <div style='text-align: right'>736,340</div>  |
| inner_verifier | AccessAdapter<2> | StoreF | STOREW2 | <span style="color: green">(-1,386 [-0.3%])</span> <div style='text-align: right'>529,562</div>  |
| inner_verifier | AccessAdapter<4> | StoreF | STOREW2 | <span style="color: green">(-819 [-0.3%])</span> <div style='text-align: right'>313,339</div>  |
| inner_verifier | AccessAdapter<8> | StoreF | STOREW2 | <span style="color: green">(-357 [-0.2%])</span> <div style='text-align: right'>206,499</div>  |
| inner_verifier | Boundary | StoreF | STOREW2 | <span style="color: green">(-24,928 [-42.1%])</span> <div style='text-align: right'>34,276</div>  |
| inner_verifier | CoreAir | StoreF | STOREW2 | <span style="color: green">(-13,860 [-0.2%])</span> <div style='text-align: right'>5,717,690</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <span style="color: green">(-10,440 [-0.2%])</span> <div style='text-align: right'>5,859,690</div>  |
| inner_verifier | Boundary | StoreHintWord | SHINTW | <span style="color: green">(-1,652,260 [-42.2%])</span> <div style='text-align: right'>2,262,766</div>  |
| inner_verifier | CoreAir | StoreHintWord | SHINTW | <span style="color: green">(-19,140 [-0.2%])</span> <div style='text-align: right'>11,313,830</div>  |
| inner_verifier | Boundary | StoreV | STOREW | <span style="color: green">(-11,392 [-42.1%])</span> <div style='text-align: right'>15,664</div>  |
| inner_verifier | CoreAir | StoreV | STOREW | <div style='text-align: right'>78,320</div>  |
| inner_verifier | Boundary | StoreV | STOREW2 | <span style="color: green">(-201,904 [-42.1%])</span> <div style='text-align: right'>277,618</div>  |
| inner_verifier | CoreAir | StoreV | STOREW2 | <div style='text-align: right'>1,400,465</div>  |
| inner_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <span style="color: green">(-800 [-0.1%])</span> <div style='text-align: right'>554,280</div>  |
| inner_verifier | AccessAdapter<2> | SubE | FE4SUB | <span style="color: green">(-506 [-0.1%])</span> <div style='text-align: right'>455,510</div>  |
| inner_verifier | AccessAdapter<4> | SubE | FE4SUB | <span style="color: green">(-299 [-0.1%])</span> <div style='text-align: right'>269,165</div>  |
| inner_verifier | Boundary | SubE | FE4SUB | <span style="color: green">(-403,460 [-42.1%])</span> <div style='text-align: right'>554,444</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | LOADW | <span style="color: green">(-11,088 [-0.3%])</span> <div style='text-align: right'>4,369,002</div>  |
| inner_verifier | CoreAir | SubEF | LOADW | <span style="color: green">(-166,320 [-0.3%])</span> <div style='text-align: right'>65,535,030</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <span style="color: green">(-30,240 [-0.3%])</span> <div style='text-align: right'>11,915,460</div>  |
| inner_verifier | AccessAdapter<2> | SubEF | SUB | <span style="color: green">(-11,088 [-0.3%])</span> <div style='text-align: right'>4,369,002</div>  |
| inner_verifier | AccessAdapter<4> | SubEF | SUB | <span style="color: green">(-13,104 [-0.3%])</span> <div style='text-align: right'>5,163,366</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>37,680</div>  |
| inner_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>506</div>  |
| inner_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>299</div>  |
| inner_verifier | Boundary | SubEFI | ADD | <span style="color: green">(-9,312 [-42.1%])</span> <div style='text-align: right'>12,804</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>8,640</div>  |
| inner_verifier | AccessAdapter<2> | SubEI | ADD | <span style="color: green">(-132 [-5.6%])</span> <div style='text-align: right'>2,244</div>  |
| inner_verifier | AccessAdapter<4> | SubEI | ADD | <span style="color: green">(-78 [-5.6%])</span> <div style='text-align: right'>1,326</div>  |
| inner_verifier | Boundary | SubEI | ADD | <span style="color: green">(-384 [-42.1%])</span> <div style='text-align: right'>528</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>461,580</div>  |
| inner_verifier | Boundary | SubV | SUB | <span style="color: green">(-32 [-42.1%])</span> <div style='text-align: right'>44</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>38,100</div>  |
| inner_verifier | Boundary | SubVI | SUB | <span style="color: green">(-5,624 [-42.1%])</span> <div style='text-align: right'>7,733</div>  |
| inner_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>10,710</div>  |

</details>

| group | air_name | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| bench_program_inner | ProgramAir | <div style='text-align: right'>1,152</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | VmConnectorAir | <div style='text-align: right'>32</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | CoreAir | <div style='text-align: right'>3,168</div>  | <div style='text-align: right'>83</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>55</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>568</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  |
| bench_program_inner | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>60</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| bench_program_inner | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>1,056</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>76</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  |
| bench_program_inner | KeccakVmAir | <div style='text-align: right'>132,544</div>  | <div style='text-align: right'>2,251</div>  | <div style='text-align: right'>235</div>  | <div style='text-align: right'>3,198</div>  | <div style='text-align: right'>944</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | XorLookupAir<8> | <div style='text-align: right'>589,824</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>65,536</div>  |
| bench_program_inner | VolatileBoundaryAir | <span style="color: green">(-1,536 [-34.3%])</span> <div style='text-align: right'>2,944</div>  | <span style="color: green">(-4 [-19.0%])</span> <div style='text-align: right'>17</div>  | <span style="color: green">(-2 [-33.3%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  | <span style="color: green">(-4 [-25.0%])</span> <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>128</div>  |
| bench_program_inner | AccessAdapterAir<2> | <div style='text-align: right'>2,240</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>64</div>  |
| bench_program_inner | AccessAdapterAir<4> | <div style='text-align: right'>1,184</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>32</div>  |
| bench_program_inner | AccessAdapterAir<8> | <div style='text-align: right'>656</div>  | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>16</div>  |
| bench_program_inner | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |
| inner_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>262,144</div>  |
| inner_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| inner_verifier | CoreAir | <div style='text-align: right'>314,572,800</div>  | <div style='text-align: right'>77</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>55</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>45,088,768</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>720,896</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>58,720,256</div>  | <div style='text-align: right'>22</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | Poseidon2VmAir<BabyBear> | <div style='text-align: right'>14,614,528</div>  | <div style='text-align: right'>374</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>418</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>32,768</div>  |
| inner_verifier | VolatileBoundaryAir | <span style="color: green">(-8,388,608 [-29.6%])</span> <div style='text-align: right'>19,922,944</div>  | <span style="color: green">(-3 [-15.8%])</span> <div style='text-align: right'>16</div>  | <span style="color: green">(-2 [-33.3%])</span> <div style='text-align: right'>4</div>  | <span style="color: green">(-8 [-42.1%])</span> <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <span style="color: green">(-4 [-50.0%])</span> <div style='text-align: right'>4</div>  | <div style='text-align: right'>1,048,576</div>  |
| inner_verifier | AccessAdapterAir<2> | <div style='text-align: right'>96,468,992</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>4,194,304</div>  |
| inner_verifier | AccessAdapterAir<4> | <div style='text-align: right'>52,428,800</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2,097,152</div>  |
| inner_verifier | AccessAdapterAir<8> | <div style='text-align: right'>1,900,544</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>65,536</div>  |
| inner_verifier | VariableRangeCheckerAir | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>131,072</div>  |



[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-staging-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/eeb1745868e35aa5b785c3c143f6473be20dd6fb/small_e2e.dsl_ir.opcode.frequency.svg)
Commit: https://github.com/axiom-crypto/afs-prototype/commit/eeb1745868e35aa5b785c3c143f6473be20dd6fb
AWS Instance Type: [r7g.16xlarge](https://instances.vantage.sh/aws/ec2/r7g.16xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11489278125)
